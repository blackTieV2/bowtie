-- Original case-file schema proposal. Not a legacy format.
-- Authority: SPEC-013 sections 3.4-3.12, 7.2-7.4, 7.12-7.16;
-- future reference contracts: SPEC-021 24.7, 29.2; SPEC-002 chapter 10.
PRAGMA foreign_keys = ON;
BEGIN;
PRAGMA user_version = 1;
CREATE TABLE case_info (
 id INTEGER PRIMARY KEY CHECK(id = 1),
 title TEXT NOT NULL CHECK(length(trim(title)) > 0),
 author TEXT NOT NULL DEFAULT '', message TEXT NOT NULL DEFAULT '',
 metadata_json TEXT NOT NULL DEFAULT '{}' CHECK(json_valid(metadata_json) AND json_type(metadata_json)='object')
);
CREATE TABLE lookup_value (
 id TEXT PRIMARY KEY NOT NULL CHECK(length(id)>0),
 table_key TEXT NOT NULL CHECK(length(table_key)>0),
 code TEXT NOT NULL DEFAULT '', title TEXT NOT NULL CHECK(length(trim(title))>0),
 color TEXT, sort_order INTEGER NOT NULL DEFAULT 0 CHECK(sort_order>=0),
 metadata_json TEXT NOT NULL DEFAULT '{}' CHECK(json_valid(metadata_json) AND json_type(metadata_json)='object'),
 UNIQUE(table_key,id)
);
CREATE TABLE entity (
 id TEXT PRIMARY KEY NOT NULL CHECK(length(id)>0),
 kind TEXT NOT NULL CHECK(kind IN ('organization','group','hazard','diagram','top_event','threat','consequence','barrier','escalation_factor','activity','document','system','competency','job_title','product','action')),
 parent_id TEXT REFERENCES entity(id) ON DELETE RESTRICT,
 title TEXT NOT NULL CHECK(length(trim(title))>0),
 description TEXT NOT NULL DEFAULT '', type_key TEXT NOT NULL DEFAULT '',
 effectiveness_id TEXT REFERENCES lookup_value(id) ON DELETE RESTRICT,
 sort_order INTEGER NOT NULL DEFAULT 0 CHECK(sort_order>=0),
 metadata_json TEXT NOT NULL DEFAULT '{}' CHECK(json_valid(metadata_json) AND json_type(metadata_json)='object'),
 CHECK((kind='organization' AND parent_id IS NULL) OR (kind<>'organization' AND parent_id IS NOT NULL))
);
CREATE UNIQUE INDEX one_organization ON entity(kind) WHERE kind='organization';
CREATE UNIQUE INDEX one_top_event_per_diagram ON entity(parent_id) WHERE kind='top_event';
CREATE INDEX entity_children ON entity(parent_id,sort_order,id);
CREATE TABLE entity_property (
 entity_id TEXT NOT NULL REFERENCES entity(id) ON DELETE RESTRICT,
 property_key TEXT NOT NULL CHECK(length(property_key)>0),
 value_json TEXT NOT NULL CHECK(json_valid(value_json)),
 PRIMARY KEY(entity_id,property_key)
);
CREATE TABLE entity_link (
 source_id TEXT NOT NULL REFERENCES entity(id) ON DELETE RESTRICT,
 target_id TEXT NOT NULL REFERENCES entity(id) ON DELETE RESTRICT,
 relation TEXT NOT NULL CHECK(relation IN ('activity','document','system','competency','accountable','responsible','input','output','action_party','sign_off_authority')),
 PRIMARY KEY(source_id,target_id,relation)
);
CREATE INDEX link_backlinks ON entity_link(target_id,relation);
CREATE TRIGGER entity_parent_insert BEFORE INSERT ON entity BEGIN
 SELECT CASE WHEN NEW.parent_id IS NOT NULL AND NOT EXISTS(SELECT 1 FROM entity p WHERE p.id=NEW.parent_id AND ((NEW.kind='group' AND p.kind IN ('organization','group')) OR (NEW.kind='hazard' AND p.kind IN ('group')) OR (NEW.kind='diagram' AND p.kind IN ('hazard')) OR (NEW.kind='top_event' AND p.kind IN ('diagram')) OR (NEW.kind='threat' AND p.kind IN ('top_event')) OR (NEW.kind='consequence' AND p.kind IN ('top_event')) OR (NEW.kind='barrier' AND p.kind IN ('threat','consequence','escalation_factor')) OR (NEW.kind='escalation_factor' AND p.kind IN ('barrier')) OR (NEW.kind='activity' AND p.kind IN ('organization','activity')) OR (NEW.kind='document' AND p.kind IN ('organization','document')) OR (NEW.kind='system' AND p.kind IN ('organization','system')) OR (NEW.kind='competency' AND p.kind IN ('organization')) OR (NEW.kind='job_title' AND p.kind IN ('organization')) OR (NEW.kind='product' AND p.kind IN ('organization')) OR (NEW.kind='action' AND p.kind IN ('hazard','top_event','threat','consequence','barrier','escalation_factor','activity')))) THEN RAISE(ABORT,'Invalid parent kind or missing parent') END;
 SELECT CASE WHEN NEW.effectiveness_id IS NOT NULL AND NOT EXISTS(SELECT 1 FROM lookup_value WHERE id=NEW.effectiveness_id AND table_key='effectiveness') THEN RAISE(ABORT,'Effectiveness must reference effectiveness taxonomy') END;
 SELECT CASE WHEN EXISTS(WITH RECURSIVE ancestors(id,parent_id) AS (SELECT id,parent_id FROM entity WHERE id=NEW.parent_id UNION SELECT p.id,p.parent_id FROM entity p JOIN ancestors a ON p.id=a.parent_id) SELECT 1 FROM ancestors WHERE id=NEW.id) THEN RAISE(ABORT,'Hierarchy cycle') END;
END;
CREATE TRIGGER entity_parent_update BEFORE UPDATE ON entity BEGIN
 SELECT CASE WHEN NEW.parent_id IS NOT NULL AND NOT EXISTS(SELECT 1 FROM entity p WHERE p.id=NEW.parent_id AND ((NEW.kind='group' AND p.kind IN ('organization','group')) OR (NEW.kind='hazard' AND p.kind IN ('group')) OR (NEW.kind='diagram' AND p.kind IN ('hazard')) OR (NEW.kind='top_event' AND p.kind IN ('diagram')) OR (NEW.kind='threat' AND p.kind IN ('top_event')) OR (NEW.kind='consequence' AND p.kind IN ('top_event')) OR (NEW.kind='barrier' AND p.kind IN ('threat','consequence','escalation_factor')) OR (NEW.kind='escalation_factor' AND p.kind IN ('barrier')) OR (NEW.kind='activity' AND p.kind IN ('organization','activity')) OR (NEW.kind='document' AND p.kind IN ('organization','document')) OR (NEW.kind='system' AND p.kind IN ('organization','system')) OR (NEW.kind='competency' AND p.kind IN ('organization')) OR (NEW.kind='job_title' AND p.kind IN ('organization')) OR (NEW.kind='product' AND p.kind IN ('organization')) OR (NEW.kind='action' AND p.kind IN ('hazard','top_event','threat','consequence','barrier','escalation_factor','activity')))) THEN RAISE(ABORT,'Invalid parent kind or missing parent') END;
 SELECT CASE WHEN NEW.effectiveness_id IS NOT NULL AND NOT EXISTS(SELECT 1 FROM lookup_value WHERE id=NEW.effectiveness_id AND table_key='effectiveness') THEN RAISE(ABORT,'Effectiveness must reference effectiveness taxonomy') END;
 SELECT CASE WHEN EXISTS(WITH RECURSIVE ancestors(id,parent_id) AS (SELECT id,parent_id FROM entity WHERE id=NEW.parent_id UNION SELECT p.id,p.parent_id FROM entity p JOIN ancestors a ON p.id=a.parent_id) SELECT 1 FROM ancestors WHERE id=NEW.id) THEN RAISE(ABORT,'Hierarchy cycle') END;
END;
CREATE TRIGGER entity_identity_immutable BEFORE UPDATE OF id,kind ON entity WHEN NEW.id<>OLD.id OR NEW.kind<>OLD.kind BEGIN SELECT RAISE(ABORT,'Entity identity and kind are immutable'); END;
CREATE TRIGGER entity_link_insert BEFORE INSERT ON entity_link BEGIN SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM entity s,entity t WHERE s.id=NEW.source_id AND t.id=NEW.target_id AND ((NEW.relation='activity' AND s.kind IN ('barrier') AND t.kind IN ('activity')) OR (NEW.relation='document' AND s.kind IN ('hazard','top_event','threat','consequence','barrier','escalation_factor','activity','product') AND t.kind IN ('document')) OR (NEW.relation='system' AND s.kind IN ('barrier') AND t.kind IN ('system')) OR (NEW.relation='competency' AND s.kind IN ('activity') AND t.kind IN ('competency')) OR (NEW.relation='accountable' AND s.kind IN ('hazard','barrier') AND t.kind IN ('job_title')) OR (NEW.relation='responsible' AND s.kind IN ('activity') AND t.kind IN ('job_title')) OR (NEW.relation='input' AND s.kind IN ('activity') AND t.kind IN ('product')) OR (NEW.relation='output' AND s.kind IN ('activity') AND t.kind IN ('product')) OR (NEW.relation='action_party' AND s.kind IN ('action') AND t.kind IN ('job_title')) OR (NEW.relation='sign_off_authority' AND s.kind IN ('action','activity','hazard') AND t.kind IN ('job_title')))) THEN RAISE(ABORT,'Invalid link endpoints') END; END;
CREATE TRIGGER entity_link_update BEFORE UPDATE ON entity_link BEGIN SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM entity s,entity t WHERE s.id=NEW.source_id AND t.id=NEW.target_id AND ((NEW.relation='activity' AND s.kind IN ('barrier') AND t.kind IN ('activity')) OR (NEW.relation='document' AND s.kind IN ('hazard','top_event','threat','consequence','barrier','escalation_factor','activity','product') AND t.kind IN ('document')) OR (NEW.relation='system' AND s.kind IN ('barrier') AND t.kind IN ('system')) OR (NEW.relation='competency' AND s.kind IN ('activity') AND t.kind IN ('competency')) OR (NEW.relation='accountable' AND s.kind IN ('hazard','barrier') AND t.kind IN ('job_title')) OR (NEW.relation='responsible' AND s.kind IN ('activity') AND t.kind IN ('job_title')) OR (NEW.relation='input' AND s.kind IN ('activity') AND t.kind IN ('product')) OR (NEW.relation='output' AND s.kind IN ('activity') AND t.kind IN ('product')) OR (NEW.relation='action_party' AND s.kind IN ('action') AND t.kind IN ('job_title')) OR (NEW.relation='sign_off_authority' AND s.kind IN ('action','activity','hazard') AND t.kind IN ('job_title')))) THEN RAISE(ABORT,'Invalid link endpoints') END; END;

CREATE TABLE risk_matrix (
 id TEXT PRIMARY KEY NOT NULL, title TEXT NOT NULL CHECK(length(trim(title))>0),
 coordinate_order TEXT NOT NULL DEFAULT 'xy' CHECK(coordinate_order IN ('xy','yx')),
 metadata_json TEXT NOT NULL DEFAULT '{}' CHECK(json_valid(metadata_json))
);
CREATE TRIGGER matrix_limit BEFORE INSERT ON risk_matrix WHEN (SELECT count(*) FROM risk_matrix)>=8 BEGIN SELECT RAISE(ABORT,'At most eight risk matrices'); END;
CREATE TABLE risk_axis (
 matrix_id TEXT NOT NULL REFERENCES risk_matrix(id) ON DELETE RESTRICT,
 axis TEXT NOT NULL CHECK(axis IN ('likelihood','severity')),
 position INTEGER NOT NULL CHECK(position>=0), code TEXT NOT NULL, title TEXT NOT NULL,
 PRIMARY KEY(matrix_id,axis,position)
);
CREATE TABLE risk_category (
 matrix_id TEXT NOT NULL REFERENCES risk_matrix(id) ON DELETE RESTRICT,
 id TEXT NOT NULL, title TEXT NOT NULL, color TEXT NOT NULL,
 rank INTEGER NOT NULL CHECK(rank>=0), PRIMARY KEY(matrix_id,id), UNIQUE(matrix_id,rank)
);
CREATE TABLE risk_cell (
 matrix_id TEXT NOT NULL, likelihood_position INTEGER NOT NULL, severity_position INTEGER NOT NULL,
 likelihood_axis TEXT NOT NULL DEFAULT 'likelihood' CHECK(likelihood_axis='likelihood'),
 severity_axis TEXT NOT NULL DEFAULT 'severity' CHECK(severity_axis='severity'),
 category_id TEXT NOT NULL, label TEXT NOT NULL DEFAULT '',
 PRIMARY KEY(matrix_id,likelihood_position,severity_position),
 FOREIGN KEY(matrix_id,likelihood_axis,likelihood_position) REFERENCES risk_axis(matrix_id,axis,position) ON DELETE RESTRICT,
 FOREIGN KEY(matrix_id,severity_axis,severity_position) REFERENCES risk_axis(matrix_id,axis,position) ON DELETE RESTRICT,
 FOREIGN KEY(matrix_id,category_id) REFERENCES risk_category(matrix_id,id) ON DELETE RESTRICT
);
CREATE TABLE risk_assessment (
 target_id TEXT NOT NULL REFERENCES entity(id) ON DELETE RESTRICT,
 matrix_id TEXT NOT NULL, phase TEXT NOT NULL CHECK(phase IN ('inherent','residual')),
 likelihood_position INTEGER NOT NULL, severity_position INTEGER NOT NULL,
 PRIMARY KEY(target_id,matrix_id,phase),
 FOREIGN KEY(matrix_id,likelihood_position,severity_position) REFERENCES risk_cell(matrix_id,likelihood_position,severity_position) ON DELETE RESTRICT
);
CREATE TABLE audit (
 id TEXT PRIMARY KEY NOT NULL, title TEXT NOT NULL,
 metadata_json TEXT NOT NULL DEFAULT '{}' CHECK(json_valid(metadata_json))
);
CREATE TABLE audit_question (
 id TEXT PRIMARY KEY NOT NULL, audit_id TEXT NOT NULL REFERENCES audit(id) ON DELETE RESTRICT,
 parent_id TEXT, title TEXT NOT NULL,
 answers_json TEXT NOT NULL DEFAULT '[]' CHECK(json_valid(answers_json) AND json_type(answers_json)='array'),
 metadata_json TEXT NOT NULL DEFAULT '{}' CHECK(json_valid(metadata_json)), UNIQUE(id,audit_id),
 FOREIGN KEY(parent_id,audit_id) REFERENCES audit_question(id,audit_id) ON DELETE RESTRICT
);
CREATE TABLE audit_question_target (
 question_id TEXT NOT NULL REFERENCES audit_question(id) ON DELETE RESTRICT,
 target_id TEXT NOT NULL REFERENCES entity(id) ON DELETE RESTRICT,
 PRIMARY KEY(question_id,target_id)
);
CREATE TABLE audit_survey (
 id TEXT PRIMARY KEY NOT NULL, audit_id TEXT NOT NULL REFERENCES audit(id) ON DELETE RESTRICT,
 title TEXT NOT NULL, UNIQUE(id,audit_id)
);
CREATE TABLE audit_survey_question (
 survey_id TEXT NOT NULL, audit_id TEXT NOT NULL, question_id TEXT NOT NULL, position INTEGER NOT NULL CHECK(position>=0),
 PRIMARY KEY(survey_id,question_id), UNIQUE(survey_id,position),
 FOREIGN KEY(survey_id,audit_id) REFERENCES audit_survey(id,audit_id) ON DELETE RESTRICT,
 FOREIGN KEY(question_id,audit_id) REFERENCES audit_question(id,audit_id) ON DELETE RESTRICT
);
CREATE TABLE incident (
 id TEXT PRIMARY KEY NOT NULL, title TEXT NOT NULL, method_key TEXT NOT NULL,
 metadata_json TEXT NOT NULL DEFAULT '{}' CHECK(json_valid(metadata_json))
);
CREATE TABLE incident_node (
 id TEXT PRIMARY KEY NOT NULL, incident_id TEXT NOT NULL REFERENCES incident(id) ON DELETE RESTRICT,
 kind TEXT NOT NULL CHECK(kind IN ('event','barrier','actor','evidence')),
 title TEXT NOT NULL, metadata_json TEXT NOT NULL DEFAULT '{}' CHECK(json_valid(metadata_json)), UNIQUE(id,incident_id)
);
CREATE TABLE incident_edge (
 incident_id TEXT NOT NULL, source_id TEXT NOT NULL, target_id TEXT NOT NULL,
 PRIMARY KEY(source_id,target_id), CHECK(source_id<>target_id),
 FOREIGN KEY(source_id,incident_id) REFERENCES incident_node(id,incident_id) ON DELETE RESTRICT,
 FOREIGN KEY(target_id,incident_id) REFERENCES incident_node(id,incident_id) ON DELETE RESTRICT
);
CREATE TABLE incident_barrier_link (
 incident_node_id TEXT NOT NULL REFERENCES incident_node(id) ON DELETE RESTRICT,
 barrier_id TEXT NOT NULL REFERENCES entity(id) ON DELETE RESTRICT,
 PRIMARY KEY(incident_node_id,barrier_id)
);
CREATE TRIGGER risk_target_insert BEFORE INSERT ON risk_assessment BEGIN SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM entity WHERE id=NEW.target_id AND kind IN ('top_event','consequence')) THEN RAISE(ABORT,'Only top events and consequences have risk assessments') END; END;
CREATE TRIGGER audit_target_insert BEFORE INSERT ON audit_question_target BEGIN SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM entity WHERE id=NEW.target_id AND kind IN ('barrier','activity','document')) THEN RAISE(ABORT,'Invalid audit target') END; END;
CREATE TRIGGER incident_link_insert BEFORE INSERT ON incident_barrier_link BEGIN SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM entity e,incident_node n WHERE e.id=NEW.barrier_id AND e.kind='barrier' AND n.id=NEW.incident_node_id AND n.kind='barrier') THEN RAISE(ABORT,'Links require two barriers') END; END;
CREATE TRIGGER question_cycle_insert BEFORE INSERT ON audit_question BEGIN SELECT CASE WHEN NEW.parent_id=NEW.id OR EXISTS(WITH RECURSIVE a(id,parent_id) AS (SELECT id,parent_id FROM audit_question WHERE id=NEW.parent_id UNION SELECT p.id,p.parent_id FROM audit_question p JOIN a ON p.id=a.parent_id) SELECT 1 FROM a WHERE id=NEW.id) THEN RAISE(ABORT,'Question hierarchy cycle') END; END;
CREATE TRIGGER risk_target_update BEFORE UPDATE ON risk_assessment BEGIN SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM entity WHERE id=NEW.target_id AND kind IN ('top_event','consequence')) THEN RAISE(ABORT,'Only top events and consequences have risk assessments') END; END;
CREATE TRIGGER audit_target_update BEFORE UPDATE ON audit_question_target BEGIN SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM entity WHERE id=NEW.target_id AND kind IN ('barrier','activity','document')) THEN RAISE(ABORT,'Invalid audit target') END; END;
CREATE TRIGGER incident_link_update BEFORE UPDATE ON incident_barrier_link BEGIN SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM entity e,incident_node n WHERE e.id=NEW.barrier_id AND e.kind='barrier' AND n.id=NEW.incident_node_id AND n.kind='barrier') THEN RAISE(ABORT,'Links require two barriers') END; END;
CREATE TRIGGER question_cycle_update BEFORE UPDATE ON audit_question BEGIN SELECT CASE WHEN NEW.parent_id=NEW.id OR EXISTS(WITH RECURSIVE a(id,parent_id) AS (SELECT id,parent_id FROM audit_question WHERE id=NEW.parent_id UNION SELECT p.id,p.parent_id FROM audit_question p JOIN a ON p.id=a.parent_id) SELECT 1 FROM a WHERE id=NEW.id) THEN RAISE(ABORT,'Question hierarchy cycle') END; END;
CREATE TRIGGER lookup_identity_immutable BEFORE UPDATE OF id,table_key ON lookup_value WHEN NEW.id<>OLD.id OR NEW.table_key<>OLD.table_key BEGIN SELECT RAISE(ABORT,'Lookup identity and table are immutable'); END;
CREATE TRIGGER incident_identity_immutable BEFORE UPDATE OF id,kind,incident_id ON incident_node WHEN NEW.id<>OLD.id OR NEW.kind<>OLD.kind OR NEW.incident_id<>OLD.incident_id BEGIN SELECT RAISE(ABORT,'Incident node identity, kind and incident are immutable'); END;
CREATE VIEW case_tree AS WITH RECURSIVE tree(id,parent_id,kind,title,sort_order,depth,path) AS (
 SELECT id,parent_id,kind,title,sort_order,0,id FROM entity WHERE parent_id IS NULL
 UNION ALL SELECT e.id,e.parent_id,e.kind,e.title,e.sort_order,t.depth+1,t.path||'/'||e.id FROM entity e JOIN tree t ON e.parent_id=t.id
) SELECT * FROM tree;
COMMIT;
