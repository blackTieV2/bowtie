# Barrier Atlas case table v1

This is an original interchange layout, independent of any historical application format. UTF-8 CSV and an XLSX worksheet named `Case data` share the same eight columns in this exact order:

`record_type,record_id,parent_id,kind,title,description,sort_order,payload_json`

CSV uses commas, quoted fields, doubled embedded quotes, and CRLF record delimiters. Quoted values may contain commas, quotes, Unicode and line breaks. A UTF-8 BOM is accepted. Diagnostics identify the physical starting line of a CSV record. XLSX imports select a worksheet and convert its rows to this CSV representation; diagnostics refer to the corresponding converted CSV line. XLSX values are imported without formatting. Inline and shared strings are supported; formulas must first be replaced with values. No network links or external worksheet relationships are read. Input/output and expanded workbooks are limited to 20 MiB.

XLSX export also observes the 32,767-character cell limit; use CSV when a property's complete JSON payload exceeds it. Entity `properties.links` is not accepted: links use their own records so the dry-run summary covers every created relationship.

| Record type | Identity and columns | JSON payload |
|---|---|---|
| `case` | `record_id=case`; title is the case name; other text columns empty; order 0 | `author`, `message` and `metadata` (application `barrier-atlas`, format_version 1) |
| `entity` | Stable entity ID; `@root` represents the organization; parent ID; supported entity kind; title, description, nonnegative order | `properties`, nullable `effectivenessId`, `typeKey`, object `metadata` |
| `lookup` | Stable choice ID; kind is table key; title and order; parent/description empty | Full choice: `id`, `tableKey`, `code`, `title`, `color`, `sortOrder`, object `metadata` |
| `link` | record_id is JSON array `[sourceId,targetId,relation]`; parent is source ID; kind is relation; title/description empty; order 0 | `targetId` |
| `matrix` | Stable matrix ID; title; parent/kind/description empty; order 0 | `matrix` containing axes/categories/cells and coordinate order, plus persisted `metadata` (including hiding state) |
| `assessment` | record_id is JSON array `[targetId,matrixId,phase]`; parent is target ID; kind is inherent/residual; title/description empty; order 0 | `targetId`, `matrixId`, `phase`, `likelihoodPosition`, `severityPosition` |

Actions are entities with kind `action`, preserving their parents, descriptions, properties and assignments. Supporting activities/documents/systems/competencies/job titles/products, lookups, typed custom field definitions/values, saved positions, links, matrices and assessments are included. Future audit/incident records are outside Phase 5 exchange.

Every package includes one case header and one `@root` organization. Root IDs and parent references are normalized only in interchange, allowing an export/import/export round trip into a new case without rewriting other identities. A pristine target (organization only, with no properties, lookups or matrices) adopts the package case header and organization details. For a populated target, organization details/properties/custom definitions must match and imported identities must all be new; its case header is retained. Existing records are never silently updated, deduplicated or merged. The preview states whether the empty case header will be updated and lists the record counts that would be created. To import a full export of an existing case, create a new empty case first.

Hierarchy rows may be reordered; parents are resolved before children. Unresolved parents, cycles, unsupported kinds, duplicate IDs and malformed CSV/JSON are rejected with line diagnostics. Typed properties, references, dates, custom values, ratings, geometry and matrices are checked through the same service validators used by normal editing. The authoritative schema and triggers additionally enforce references, hierarchy, matrix limits and assessment constraints. Validation runs on an isolated copy; the active session and saved file are unchanged. Apply rechecks the exact table and current-case preview signature, then writes all records in one transaction. No case file is saved automatically.

The last successful replace/import can be undone as a whole while its resulting session is unchanged. A later edit or save/open/new/close invalidates that undo checkpoint. This is a one-operation session checkpoint, not a general editing history.

Example for a group (payload abbreviated here; production files include complete objects):

```csv
"record_type","record_id","parent_id","kind","title","description","sort_order","payload_json"
"entity","group-1","@root","group","Transfer station","","0","{""properties"":{},""effectivenessId"":null,""typeKey"":"""",""metadata"":{}}"
```

This illustrative row is not a complete package; a valid package also needs its case and root records.
