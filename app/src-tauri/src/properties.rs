use rusqlite::{params, Transaction};
use serde_json::{json, Value};
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
pub fn catalog() -> Value {
    serde_json::from_str(include_str!("../../src/property-catalog.json")).expect("property catalog")
}
pub fn write(tx: &Transaction<'_>, id: &str, values: &Value) -> Result<(), String> {
    let kind: String = tx
        .query_row("SELECT kind FROM entity WHERE id=?", [id], |r| r.get(0))
        .map_err(err)?;
    let catalog = catalog();
    let mut fields = catalog["common"].as_array().unwrap().clone();
    fields.extend(
        catalog[&kind]
            .as_array()
            .ok_or("Unsupported kind")?
            .iter()
            .cloned(),
    );
    for (key, value) in values.as_object().ok_or("Properties must be an object")? {
        if key == "links" {
            for (relation, ids) in value.as_object().ok_or("Links must be an object")? {
                if !allowed_links(&kind).contains(&relation.as_str()) {
                    return Err("Unsupported link relationship for this kind".into());
                }
                let ids = ids.as_array().ok_or("Link assignments must be arrays")?;
                tx.execute(
                    "DELETE FROM entity_link WHERE source_id=? AND relation=?",
                    params![id, relation],
                )
                .map_err(err)?;
                for target in ids {
                    tx.execute(
                        "INSERT INTO entity_link VALUES(?,?,?)",
                        params![
                            id,
                            target.as_str().ok_or("Link IDs must be text")?,
                            relation
                        ],
                    )
                    .map_err(err)?;
                }
            }
            continue;
        }
        if key == "user_data" {
            validate_custom(tx, &kind, value)?;
        } else {
            let field = fields
                .iter()
                .find(|f| f["key"] == *key)
                .ok_or_else(|| format!("Unsupported property: {key}"))?;
            let field_type = field["type"].as_str().unwrap();
            if value.is_null() { /* unset fields are allowed */
            } else if field_type == "checkbox" {
                if !value.is_boolean() {
                    return Err(format!("{key} must be boolean"));
                }
            } else if field_type == "integer" {
                if !value.as_i64().is_some_and(|v| v >= 0) {
                    return Err(format!("{key} must be a non-negative integer"));
                }
            } else {
                let text = value
                    .as_str()
                    .ok_or_else(|| format!("{key} must be text"))?;
                if field_type == "date" && !text.is_empty() && !valid_date(text) {
                    return Err(format!("{key} must be a valid date"));
                }
                if field_type == "select"
                    && !text.is_empty()
                    && !field["options"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|v| v == text)
                {
                    return Err(format!("Invalid {key} choice"));
                }
                if field_type == "lookup" && !text.is_empty() {
                    let table = field["table"].as_str().unwrap();
                    let valid: bool = tx
                        .query_row(
                            "SELECT EXISTS(SELECT 1 FROM lookup_value WHERE id=? AND table_key=?)",
                            params![text, table],
                            |r| r.get(0),
                        )
                        .map_err(err)?;
                    if !valid {
                        return Err(format!("{key} must reference {table}"));
                    }
                }
                if field_type == "entity" && !text.is_empty() {
                    let valid: bool = tx
                        .query_row(
                            "SELECT EXISTS(SELECT 1 FROM entity WHERE id=? AND kind=?)",
                            params![text, field["kind"].as_str().unwrap()],
                            |r| r.get(0),
                        )
                        .map_err(err)?;
                    if !valid {
                        return Err(format!("Invalid reference for {key}"));
                    }
                }
            }
            if key == "effectiveness_id" {
                let rating = value.as_str().filter(|v| !v.is_empty());
                tx.execute(
                    "UPDATE entity SET effectiveness_id=? WHERE id=?",
                    params![rating, id],
                )
                .map_err(err)?;
            }
        }
        tx.execute("INSERT INTO entity_property VALUES(?,?,?) ON CONFLICT(entity_id,property_key) DO UPDATE SET value_json=excluded.value_json",params![id,key,value.to_string()]).map_err(err)?;
    }
    Ok(())
}
pub fn allowed_links(kind: &str) -> Vec<&'static str> {
    match kind {
        "hazard" => vec!["document", "accountable", "sign_off_authority"],
        "top_event" | "threat" | "consequence" | "escalation_factor" | "product" => {
            vec!["document"]
        }
        "barrier" => vec!["document", "activity", "system", "accountable"],
        "activity" => vec![
            "document",
            "competency",
            "responsible",
            "input",
            "output",
            "sign_off_authority",
        ],
        "action" => vec!["action_party", "sign_off_authority"],
        _ => vec![],
    }
}
pub fn valid_date(text: &str) -> bool {
    let parts = text.split('-').collect::<Vec<_>>();
    if parts.len() != 3 || parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
        return false;
    }
    let (Ok(y), Ok(m), Ok(d)) = (
        parts[0].parse::<u32>(),
        parts[1].parse::<u32>(),
        parts[2].parse::<u32>(),
    ) else {
        return false;
    };
    let max = match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => return false,
    };
    y > 0 && d > 0 && d <= max
}
pub fn definitions(db: &rusqlite::Connection) -> Result<Value, String> {
    let value:String=db.query_row("SELECT coalesce((SELECT value_json FROM entity_property p JOIN entity e ON e.id=p.entity_id WHERE e.kind='organization' AND property_key='custom_definitions'),'{}')",[],|r|r.get(0)).map_err(err)?;
    serde_json::from_str(&value).map_err(err)
}
pub fn validate_custom(tx: &Transaction<'_>, kind: &str, values: &Value) -> Result<(), String> {
    let defs = definitions(tx)?;
    for (key, v) in values.as_object().ok_or("User data must be an object")? {
        let field_type = defs[kind][key].as_str().ok_or("Unknown custom field")?;
        if !v.is_null()
            && !match field_type {
                "text" => v.is_string(),
                "number" => v.is_number(),
                "boolean" => v.is_boolean(),
                _ => false,
            }
        {
            return Err("Custom field type mismatch".into());
        }
    }
    Ok(())
}
impl crate::case::CaseStore {
    pub fn custom_field(
        &mut self,
        kind: &str,
        name: &str,
        field_type: &str,
        old_name: Option<&str>,
        remove: bool,
    ) -> Result<crate::case::CaseSnapshot, String> {
        let lookup_table = kind.strip_prefix("lookup:");
        let valid_lookup = lookup_table.is_some_and(|table| {
            catalog()
                .as_object()
                .unwrap()
                .values()
                .filter_map(Value::as_array)
                .flatten()
                .any(|f| f["table"] == table)
        });
        if (catalog().get(kind).is_none() && !valid_lookup)
            || kind == "common"
            || name.trim().is_empty()
            || !["text", "number", "boolean"].contains(&field_type)
        {
            return Err("Invalid custom field definition".into());
        }
        let tx = self
            .active
            .as_mut()
            .ok_or("No case is open")?
            .db
            .transaction()
            .map_err(err)?;
        let mut defs = definitions(&tx)?;
        if defs.get(kind).is_none() {
            defs[kind] = json!({});
        }
        let old = old_name.unwrap_or(name);
        let map = defs[kind]
            .as_object_mut()
            .ok_or("Invalid custom definitions")?;
        if !remove && name != old && map.contains_key(name) {
            return Err("Custom field name already exists".into());
        }
        if map.contains_key(old) && old == name && !remove {
            return Err("Field exists; rename it or delete and redefine it".into());
        }
        let existing_type = map.remove(old);
        if !remove {
            map.insert(name.into(), existing_type.unwrap_or(json!(field_type)));
        }
        let rows = {
            let sql = if lookup_table.is_some() {
                "SELECT id,coalesce(json_extract(metadata_json,'$.user_data'),'{}') FROM lookup_value WHERE table_key=?"
            } else {
                "SELECT e.id,coalesce(p.value_json,'{}') FROM entity e LEFT JOIN entity_property p ON p.entity_id=e.id AND p.property_key='user_data' WHERE e.kind=?"
            };
            let mut q = tx.prepare(sql).map_err(err)?;
            let rows = q
                .query_map([lookup_table.unwrap_or(kind)], |r| {
                    Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
                })
                .map_err(err)?
                .collect::<rusqlite::Result<Vec<_>>>()
                .map_err(err)?;
            rows
        };
        for (id, value) in rows {
            let mut values: Value = serde_json::from_str(&value).map_err(err)?;
            let map = values.as_object_mut().ok_or("Invalid stored user data")?;
            if let Some(v) = map.remove(old) {
                if !remove {
                    map.insert(name.into(), v);
                }
                if lookup_table.is_some() {
                    tx.execute("UPDATE lookup_value SET metadata_json=json_set(metadata_json,'$.user_data',json(?)) WHERE id=?",params![values.to_string(),id]).map_err(err)?;
                } else {
                    tx.execute("INSERT INTO entity_property VALUES(?,'user_data',?) ON CONFLICT(entity_id,property_key) DO UPDATE SET value_json=excluded.value_json",params![id,values.to_string()]).map_err(err)?;
                }
            }
        }
        tx.execute("INSERT INTO entity_property VALUES((SELECT id FROM entity WHERE kind='organization'),'custom_definitions',?) ON CONFLICT(entity_id,property_key) DO UPDATE SET value_json=excluded.value_json",[defs.to_string()]).map_err(err)?;
        tx.commit().map_err(err)?;
        self.edited()
    }
}
