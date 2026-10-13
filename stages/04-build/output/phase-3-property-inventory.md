# Phase 3 typed property inventory

Run 004. Catalog keys match React controls and Rust validation; names/descriptions are native entity columns. Additional scalar/custom values use entity_property; assigned relationships use the approved typed link table. No schema migration.

| Entity kind | Field / control | Storage | Spec locator |
|---|---|---|---|
| organization | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| organization | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| organization | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| diagram | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| diagram | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| diagram | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| group | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| group | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| group | Threat acceptance criteria / textarea | entity_property | SPEC-013 Â§7.13 |
| group | General barrier acceptance criteria / textarea | entity_property | SPEC-013 Â§7.13 |
| group | Threat barrier acceptance criteria / textarea | entity_property | SPEC-013 Â§7.13 |
| group | Consequence barrier acceptance criteria / textarea | entity_property | SPEC-013 Â§7.13 |
| group | Escalation barrier acceptance criteria / textarea | entity_property | SPEC-013 Â§7.13 |
| group | Organizational units / textarea | entity_property | SPEC-013 Â§15.3 |
| group | Local attachment references / textarea | entity_property | SPEC-013 Â§Â§15.3, 10.3 |
| group | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| hazard | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| hazard | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| hazard | Affects / hazard category / lookup | entity_property | SPEC-013 Â§15.4 |
| hazard | Sign-off date / date | entity_property | SPEC-013 Â§15.4 |
| hazard | Sign-off comment / textarea | entity_property | SPEC-013 Â§15.4 |
| hazard | Revision / text | entity_property | SPEC-013 Â§Â§7.16, 15.4 |
| hazard | Revision date / date | entity_property | SPEC-013 Â§Â§7.16, 15.4 |
| hazard | Revision notes / textarea | entity_property | SPEC-013 Â§Â§7.16, 15.4 |
| hazard | Revision box alignment / select | entity_property | SPEC-013 Â§7.16 |
| hazard | Other modules / processes / textarea | entity_property | SPEC-013 Â§15.4 |
| hazard | Operations reference notes / textarea | entity_property | SPEC-013 Â§15.4 (metadata only) |
| hazard | Improvement / ALARP rationale / textarea | entity_property | SPEC-004 step 12; AR20 |
| hazard | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| top_event | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| top_event | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| top_event | Affects / category / lookup | entity_property | SPEC-013 Â§15.4 |
| top_event | Other modules / processes / textarea | entity_property | SPEC-013 Â§15.4 |
| top_event | Improvement / ALARP rationale / textarea | entity_property | SPEC-004 step 12; AR20 |
| top_event | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| threat | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| threat | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| threat | Category / lookup | entity_property | SPEC-013 Fields on Threats after Â§15.4 |
| threat | Type / lookup | entity_property | SPEC-013 Fields on Threats |
| threat | Frequency / lookup | entity_property | SPEC-013 Fields on Threats |
| threat | Equivalent top event / entity | entity_property | SPEC-013 Â§11.1; Fields on Threats |
| threat | Operations reference notes / textarea | entity_property | SPEC-013 Fields on Threats (metadata only) |
| threat | Acceptance criteria met (manual) / checkbox | entity_property | SPEC-013 Â§7.13 |
| threat | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| consequence | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| consequence | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| consequence | Category / lookup | entity_property | SPEC-013 Â§15.5 |
| consequence | Type / lookup | entity_property | SPEC-013 Â§15.5 |
| consequence | Equivalent top event / entity | entity_property | SPEC-013 Â§Â§11.1, 15.5 |
| consequence | Operations reference notes / textarea | entity_property | SPEC-013 Â§15.5 (metadata only) |
| consequence | Acceptance criteria met (manual) / checkbox | entity_property | SPEC-013 Â§7.13 |
| consequence | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| barrier | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| barrier | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| barrier | Family (metadata) / text | entity_property | SPEC-013 Â§15.6; no Advanced bundling |
| barrier | Barrier category / lookup | entity_property | SPEC-013 Â§15.6 |
| barrier | Barrier type / lookup | entity_property | SPEC-013 Â§15.6 |
| barrier | Effectiveness / lookup | entity_property + entity.effectiveness_id FK | SPEC-013 Â§Â§3.11.2, 15.6 |
| barrier | BRF code / lookup | entity_property | SPEC-013 Â§15.6 |
| barrier | Criticality / lookup | entity_property | SPEC-013 Â§15.6 |
| barrier | Equivalent top event / entity | entity_property | SPEC-013 Â§Â§11.3, 15.6 |
| barrier | Performance by organizational unit (notes) / textarea | entity_property | SPEC-013 Â§15.6 (local metadata) |
| barrier | User-system reference notes / textarea | entity_property | SPEC-013 Â§15.6 (metadata; no Advanced modeling) |
| barrier | Acceptance criteria met (manual) / checkbox | entity_property | SPEC-013 Â§7.13 |
| barrier | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| escalation_factor | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| escalation_factor | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| escalation_factor | Category / lookup | entity_property | SPEC-013 Â§15.7 |
| escalation_factor | Type / lookup | entity_property | SPEC-013 Â§15.7 |
| escalation_factor | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| activity | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| activity | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| activity | Activity category / lookup | entity_property | SPEC-013 Â§3.11.2 |
| activity | Frequency / lookup | entity_property | SPEC-013 Â§3.11.2 |
| activity | Objectives / textarea | entity_property | SPEC-013 Â§7.2 |
| activity | Performance indicators / textarea | entity_property | SPEC-013 Â§7.2 |
| activity | Timing / frequency notes / textarea | entity_property | SPEC-004 step 10 |
| activity | Instructions / textarea | entity_property | SPEC-004 step 10 |
| activity | Verification method / textarea | entity_property | SPEC-004 step 11 |
| activity | Sign-off date / date | entity_property | SPEC-013 Â§3.11.2 |
| activity | Sign-off comment / textarea | entity_property | SPEC-013 Â§3.11.2 |
| activity | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| document | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| document | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| document | Local document reference / text | entity_property | SPEC-013 Â§Â§10.1-10.3 |
| document | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| product | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| product | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| product | Local evidence reference / text | entity_property | SPEC-013 Â§7.2; SPEC-004 step 11 |
| product | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| system | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| system | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| system | Underlying systems off before this system is off / integer | entity_property | SPEC-013 Â§3.11.2 (configuration only) |
| system | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| competency | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| competency | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| competency | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| job_title | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| job_title | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| job_title | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| action | Name / text; description / multiline text | entity | SPEC-013 §§7.4, 15.2 |
| action | Code / text | entity_property | SPEC-013 Â§Â§3.11.2, 15.2 |
| action | Category / lookup | entity_property | SPEC-013 Â§3.12.4.1 |
| action | External reference / text | entity_property | SPEC-013 Â§3.12.4.1 (stored only) |
| action | Target date / date | entity_property | SPEC-013 Â§3.12.4.1 |
| action | Priority / lookup | entity_property | SPEC-013 Â§3.12.4.1 |
| action | Completed / checkbox | entity_property | SPEC-013 Â§3.12.4.1 |
| action | Sign-off date / date | entity_property | SPEC-013 Â§3.12.4.1 |
| action | Sign-off comment / textarea | entity_property | SPEC-013 Â§3.12.4.1 |
| action | Improvement / ALARP rationale / textarea | entity_property | SPEC-004 step 12; AR20 |
| action | User data / named text, number, boolean controls | entity_property; shared definitions | SPEC-013 §§7.4.1, 15.2 |
| hazard | documents, accountable, sign-off authority / typed multi-selection | entity_link; endpoint triggers | SPEC-013 §§3.11.2–3.11.4, 3.12.4.1, 7.4.2 |
| top_event | documents / typed multi-selection | entity_link; endpoint triggers | SPEC-013 §§3.11.2–3.11.4, 3.12.4.1, 7.4.2 |
| threat | documents / typed multi-selection | entity_link; endpoint triggers | SPEC-013 §§3.11.2–3.11.4, 3.12.4.1, 7.4.2 |
| consequence | documents / typed multi-selection | entity_link; endpoint triggers | SPEC-013 §§3.11.2–3.11.4, 3.12.4.1, 7.4.2 |
| barrier | activities, documents, systems, accountable / typed multi-selection | entity_link; endpoint triggers | SPEC-013 §§3.11.2–3.11.4, 3.12.4.1, 7.4.2 |
| escalation_factor | documents / typed multi-selection | entity_link; endpoint triggers | SPEC-013 §§3.11.2–3.11.4, 3.12.4.1, 7.4.2 |
| activity | documents, competencies, responsible, input/output products, sign-off authority / typed multi-selection | entity_link; endpoint triggers | SPEC-013 §§3.11.2–3.11.4, 3.12.4.1, 7.4.2 |
| product | documents / typed multi-selection | entity_link; endpoint triggers | SPEC-013 §§3.11.2–3.11.4, 3.12.4.1, 7.4.2 |
| action | action party, sign-off authority / typed multi-selection | entity_link; endpoint triggers | SPEC-013 §§3.11.2–3.11.4, 3.12.4.1, 7.4.2 |
| top_event, consequence | Inherent/residual matrix, likelihood/severity selectors; category/coordinate/rank display | risk_assessment; target and cell triggers | SPEC-013 §7.12; corrected AR45 |
| lookup values | Code/name/color/order/description; effectiveness frequency; typed custom fields | lookup_value + metadata; type-wide definitions | SPEC-013 §§3.11.2, 7.3, 7.4.1 |

Hazard sign-off/revision/accountability controls remain on the shared hazard and can be opened from the top-event panel. Questions/audit execution stay in Phase 6; Advanced/server fields appear as local reference metadata rather than enabling those modules. Attachments are local references, not server uploads. See AR52. Supporting kinds with no further documented scalar fields expose common fields and user data rather than invented defaults.
