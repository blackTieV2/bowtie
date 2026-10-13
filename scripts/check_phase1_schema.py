"""Schema-level acceptance against the twelve-step relationships, not GUI parity."""
import json
from contextlib import ExitStack
from pathlib import Path
import sqlite3
import tempfile

ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "app/src-tauri/schema/001-initial.sql"


def check():
    negatives = []
    with tempfile.TemporaryDirectory(prefix="case-schema-") as folder, ExitStack() as resources:
        filename = Path(folder) / "case.sqlite"
        db = sqlite3.connect(filename)
        resources.callback(db.close)
        db.executescript(SCHEMA.read_text(encoding="utf-8"))

        def node(identity, kind, parent, title, **metadata):
            db.execute(
                "INSERT INTO entity(id,kind,parent_id,title,metadata_json) VALUES(?,?,?,?,?)",
                (identity, kind, parent, title, json.dumps(metadata)),
            )

        def link(source, target, relation):
            db.execute("INSERT INTO entity_link VALUES(?,?,?)", (source, target, relation))

        def rejects(name, sql, parameters=()):
            db.execute("SAVEPOINT negative_check")
            try:
                db.execute(sql, parameters)
            except sqlite3.IntegrityError:
                negatives.append(name)
            else:
                raise AssertionError(f"Expected integrity rejection: {name}")
            finally:
                db.execute("ROLLBACK TO negative_check")
                db.execute("RELEASE negative_check")

        db.execute("INSERT INTO case_info(id,title) VALUES(1,'Training case')")
        node("org", "organization", None, "Example workshop")
        node("group", "group", "org", "Transfer station")
        # Steps 1-8: one hazard/top-event combination, paths and factor controls.
        node("hazard", "hazard", "group", "Liquid transfer")
        node("diagram", "diagram", "hazard", "Transfer containment")
        node("top", "top_event", "diagram", "Loss of containment")
        node("threat", "threat", "top", "Hose damage")
        node("consequence", "consequence", "top", "Work area affected")
        node("prevention", "barrier", "threat", "Pre-use inspection")
        node("recovery", "barrier", "consequence", "Isolation and collection")
        node("factor", "escalation_factor", "prevention", "Inspection omitted")
        node("factor-control", "barrier", "factor", "Independent start check")
        # Steps 9-11: recurring work, ownership/instructions and verification output.
        node("task", "activity", "org", "Inspect transfer equipment", frequency="each shift")
        node("owner", "job_title", "org", "Shift coordinator")
        node("instruction", "document", "org", "Inspection instructions", path="inspection.txt")
        node("record", "product", "org", "Signed inspection record")
        link("prevention", "task", "activity")
        link("task", "owner", "responsible")
        link("task", "instruction", "document")
        link("task", "record", "output")
        # Step 12 records a human improvement judgment, not an ALARP calculation.
        node("improvement", "action", "prevention", "Review inspection prompts",
             completed=False, rationale="Assess whether clearer prompts improve reliability")
        link("improvement", "owner", "action_party")
        db.execute("INSERT INTO risk_matrix(id,title) VALUES('matrix','Practice matrix')")
        db.execute("INSERT INTO risk_axis VALUES('matrix','likelihood',0,'L0','Example frequency')")
        db.execute("INSERT INTO risk_axis VALUES('matrix','severity',0,'S0','Example severity')")
        db.execute("INSERT INTO risk_category VALUES('matrix','review','Review','#8899aa',0)")
        db.execute("INSERT INTO risk_cell(matrix_id,likelihood_position,severity_position,category_id) VALUES('matrix',0,0,'review')")
        for phase in ("inherent", "residual"):
            db.execute("INSERT INTO risk_assessment VALUES('top','matrix',?,0,0)", (phase,))
        # Future reference contracts only; no module execution/scoring implemented.
        db.execute("INSERT INTO audit VALUES('audit','Barrier review','{}')")
        db.execute("INSERT INTO audit_question(id,audit_id,title,answers_json) VALUES(?,?,?,?)",
                   ("question", "audit", "Inspection evidence", json.dumps(["Absent", "Present"])))
        db.execute("INSERT INTO audit_question_target VALUES('question','prevention')")
        db.execute("INSERT INTO audit_survey VALUES('survey','audit','Shift review')")
        db.execute("INSERT INTO audit_survey_question VALUES('survey','audit','question',0)")
        db.execute("INSERT INTO incident VALUES('incident','Transfer review','event-barrier','{}')")
        db.execute("INSERT INTO incident_node VALUES('incident-barrier','incident','barrier','Inspection failed','{}')")
        db.execute("INSERT INTO incident_barrier_link VALUES('incident-barrier','prevention')")

        rejects("orphan parent", "INSERT INTO entity(id,kind,parent_id,title) VALUES('bad','group','missing','Bad')")
        rejects("invalid parent type", "UPDATE entity SET parent_id='threat' WHERE id='group'")
        rejects("hierarchy cycle", "UPDATE entity SET parent_id='group' WHERE id='group'")
        rejects("empty required title", "UPDATE entity SET title=' ' WHERE id='group'")
        rejects("invalid metadata", "UPDATE entity SET metadata_json='not-json' WHERE id='group'")
        rejects("entity kind mutation", "UPDATE entity SET kind='consequence' WHERE id='threat'")
        rejects("duplicate top event", "INSERT INTO entity(id,kind,parent_id,title) VALUES('another','top_event','diagram','Other')")
        rejects("used entity deletion", "DELETE FROM entity WHERE id='prevention'")
        rejects("wrong link endpoints", "INSERT INTO entity_link VALUES('task','prevention','activity')")
        rejects("risk on wrong entity", "INSERT INTO risk_assessment VALUES('threat','matrix','inherent',0,0)")
        rejects("missing matrix cell", "INSERT INTO risk_assessment VALUES('consequence','matrix','inherent',2,3)")
        rejects("wrong audit target", "INSERT INTO audit_question_target VALUES('question','top')")
        rejects("question hierarchy cycle", "UPDATE audit_question SET parent_id='question' WHERE id='question'")
        rejects("wrong incident target", "INSERT INTO incident_barrier_link VALUES('incident-barrier','top')")
        rejects("linked incident kind mutation", "UPDATE incident_node SET kind='event' WHERE id='incident-barrier'")
        db.execute("INSERT INTO lookup_value(id,table_key,title) VALUES('category','category','Example category')")
        rejects("wrong effectiveness taxonomy", "UPDATE entity SET effectiveness_id='category' WHERE id='prevention'")
        db.execute("SAVEPOINT limits")
        for i in range(1, 8):
            db.execute("INSERT INTO risk_matrix(id,title) VALUES(?,?)", (f"m{i}", f"Matrix {i}"))
        rejects("ninth matrix", "INSERT INTO risk_matrix(id,title) VALUES('ninth','Ninth')")
        db.execute("ROLLBACK TO limits")
        db.execute("RELEASE limits")
        db.execute("INSERT INTO audit VALUES('other-audit','Other review','{}')")
        rejects("cross-audit survey question", "INSERT INTO audit_survey_question VALUES('survey','other-audit','question',1)")
        expected = db.execute("SELECT id,kind,parent_id,title,metadata_json FROM entity ORDER BY id").fetchall()
        assert len(expected) == 16
        assert db.execute("SELECT count(*) FROM case_tree").fetchone()[0] == 16
        assert db.execute("PRAGMA foreign_key_check").fetchall() == []
        assert db.execute("PRAGMA integrity_check").fetchone()[0] == "ok"
        db.commit()
        db.close()
        reopened = sqlite3.connect(filename)
        resources.callback(reopened.close)
        reopened.execute("PRAGMA foreign_keys=ON")
        assert reopened.execute("PRAGMA user_version").fetchone()[0] == 1
        assert reopened.execute("SELECT id,kind,parent_id,title,metadata_json FROM entity ORDER BY id").fetchall() == expected
        assert reopened.execute("SELECT count(*) FROM entity_link").fetchone()[0] == 5
        assert reopened.execute("SELECT count(*) FROM risk_assessment").fetchone()[0] == 2
        assert reopened.execute("SELECT barrier_id FROM incident_barrier_link").fetchone()[0] == "prevention"
        assert reopened.execute("SELECT target_id FROM audit_question_target").fetchone()[0] == "prevention"
        assert reopened.execute("PRAGMA foreign_key_check").fetchall() == []
        reopened.close()
    return {"result": "PASS", "scope": "schema-only", "sqlite_version": sqlite3.sqlite_version,
            "walkthrough_steps": 12, "persisted_entities": 16,
            "negative_integrity_checks": negatives, "gui_workflow_tested": False}


if __name__ == "__main__":
    print(json.dumps(check(), indent=2))
