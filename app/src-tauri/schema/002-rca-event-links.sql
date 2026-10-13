PRAGMA user_version = 2;
DROP TRIGGER incident_link_insert;
DROP TRIGGER incident_link_update;
CREATE TRIGGER incident_link_insert BEFORE INSERT ON incident_barrier_link BEGIN SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM entity e,incident_node n JOIN incident i ON i.id=n.incident_id WHERE e.id=NEW.barrier_id AND e.kind='barrier' AND n.id=NEW.incident_node_id AND (n.kind='barrier' OR (n.kind='event' AND i.method_key IN ('rca','categorized_rca')))) THEN RAISE(ABORT,'Invalid incident-to-barrier link for this method') END; END;
CREATE TRIGGER incident_link_update BEFORE UPDATE ON incident_barrier_link BEGIN SELECT CASE WHEN NOT EXISTS(SELECT 1 FROM entity e,incident_node n JOIN incident i ON i.id=n.incident_id WHERE e.id=NEW.barrier_id AND e.kind='barrier' AND n.id=NEW.incident_node_id AND (n.kind='barrier' OR (n.kind='event' AND i.method_key IN ('rca','categorized_rca')))) THEN RAISE(ABORT,'Invalid incident-to-barrier link for this method') END; END;
