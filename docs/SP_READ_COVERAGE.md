# SP read coverage

Audit date: 2026-11-20 (client date). Source baseline: `62f3bdaab3a54c3cace4b7a0f2cd22d30bb3c1f5`.
Scope: source inspection and read-only probes of two retained project copies. This is a reader-first inventory, not a completed decoder or an authoring release.

## Finding

The current reader does **not** completely interpret an SP file. It admits the observed `6.5.5.28` header/layout, validates a 48-entry container, and exposes useful partial 3D/4D/5D data.
All 48 section names below were observed in both sampled files. Their small size does not establish that their logical collections are empty. A successful partial import does not certify the compressed contents or record schemas of opaque streams.

There are six sections retained by the metadata path, four more processed by the full geometry/texture path, two with separate narrowly guarded authoring readers, and 36 with no structured SP reader found in this audit. These counts describe code paths, **not a percentage of semantic completeness**. Even the first twelve contain limits or unknown fields.

Existing-link writer changes in the stopped authoring worktree are uncommitted and excluded from this committed-source baseline. This audit does not resume them.

## Evidence and scope

| Observation | Project A | Project B |
| --- | ---: | ---: |
| Input bytes | 57,694,388 | 8,359,930 |
| Directory entries | 48 | 48 |
| Tasks returned by metadata probe | 241 | 2 |
| Resources returned by metadata probe | 79 | 2 |
| Nonempty task collections checked | 36 | 1 |
| Lookup entries checked | 241 | 2 |
| Lookup mismatches | 0 | 0 |
| Mesh instances returned by full reader | 33,513 | 36,929 |
| Mesh definitions returned by full reader | 3,797 | 3,384 |
| Vertices returned by full reader | 3,699,006 | 362,940 |

Project A SHA-256: `50960506bc75564737a7acbc1030b1040f698f2563fca1366690d7215be0c83f`.
Project B SHA-256: `73918e7b0e9ec90c2129ce0c91a08aa356ddd52429fcc7dffa187c43de4ffa7d`.

The read-only probes used the delivered CLI with SHA-256
`6f9221e270eeaf8f07d781b3b9c7542f9237efd4bcd5e3fdd5a937b29b44668e`,
calling `formats_project_metadata_inspect` and `formats_project_open`.
This identifies the executable actually tested; it is not a fresh build attestation for every source change at the baseline.
Metadata probes explicitly returned `geometry_validated: false`.
Full-reader probes returned unverified native units, axes and native row identity.
A consistent lookup vector does not prove native row identity.

The audit also checks the container header, ordered directory spans and wrappers independently, and verifies exact zlib consumption/checksums for the ten M/G streams under the production path's size caps.
These decompression checks do not establish field meanings.
Both original input hashes must remain unchanged. No new native UI comparison, rendering comparison, external-file resolution or writer round trip is part of this audit.
Detailed local fingerprints and probe output are retained in the ignored audit log.

## All observed sections

Routes:

- **M**: metadata path retains a decoded stream; structured interpretation remains partial. Calendars are retained raw and interpreted downstream.
- **G**: full `parse` path additionally decodes geometry or texture data; metadata-only inspection does not do this.
- **A**: separate guarded authoring helper reads an observed subset; not a general viewer/metadata decoder.
- **O**: wrapper/span/hash inventory only; section content remains opaque in the audited SP reader.

The two byte columns are stored section sizes, including the wrapper and compressed payload. They are not decoded sizes, record counts or completeness weights.

| # | Native section name | Route | Structured coverage / remaining boundary | A stored bytes | B stored bytes |
| --- | --- | --- | --- | ---: | ---: |
| 1 | `ProjectStateIndex` | O | Stored wrapper/span/hash only; content schema unresolved | 31 | 31 |
| 2 | `masterProjectID` | O | Stored wrapper/span/hash only; content schema unresolved | 42 | 42 |
| 3 | `Companies` | O | Stored wrapper/span/hash only; content schema unresolved | 49 | 49 |
| 4 | `Users` | O | Stored wrapper/span/hash only; content schema unresolved | 146 | 146 |
| 5 | `Roles` | O | Stored wrapper/span/hash only; content schema unresolved | 333 | 333 |
| 6 | `Tasks` | M | Framed task bodies, assignments/links/costs; opaque tails and unsupported extra lists | 34,571 | 393 |
| 7 | `TaskCatagories` | O | Stored wrapper/span/hash only; content schema unresolved | 55 | 55 |
| 8 | `Risks` | O | Stored wrapper/span/hash only; content schema unresolved | 45 | 45 |
| 9 | `Filters` | O | Stored wrapper/span/hash only; content schema unresolved | 4,763 | 2,664 |
| 10 | `Baselines` | O | Stored wrapper/span/hash only; content schema unresolved | 49 | 49 |
| 11 | `dateZero` | O | Stored wrapper/span/hash only; content schema unresolved | 44 | 41 |
| 12 | `TextureDefinitions` | G | Observed definitions, image references and path provenance; no external loading | 810 | 1,194 |
| 13 | `GraphicalImages` | G | Observed cached-image records/codecs; full pixel/sampling semantics unverified | 352,331 | 654,666 |
| 14 | `Entities` | G | Observed scene nodes, triangles and transforms; skipped/unknown attributes remain | 12,076,830 | 5,630,772 |
| 15 | `IncludeLibrary` | G | Observed definitions and instance references; other primitive/material layouts unresolved | 44,537,945 | 1,996,659 |
| 16 | `Animations` | O | Stored wrapper/span/hash only; content schema unresolved | 923 | 51 |
| 17 | `Calendars` | M | Raw decoded stream; bounded downstream calendar subset | 3,003 | 270 |
| 18 | `Activity Codes` | M | Observed definition/value trees and references; opaque suffix remains | 5,104 | 477 |
| 19 | `3DWorldSettings` | O | Stored wrapper/span/hash only; content schema unresolved | 72 | 72 |
| 20 | `Synchronizers` | O | Stored wrapper/span/hash only; content schema unresolved | 40,532 | 2,465 |
| 21 | `Use Profiles` | M | Profile definitions and phase-tail storage; general appearance semantics unresolved | 534 | 379 |
| 22 | `Clash Tests` | O | Stored wrapper/span/hash only; content schema unresolved | 51 | 51 |
| 23 | `adminIDs` | O | Stored wrapper/span/hash only; content schema unresolved | 51 | 51 |
| 24 | `User Fields` | M | Supported kind/domain definitions and selected values; other variants unresolved | 18,124 | 4,249 |
| 25 | `Positions` | O | Stored wrapper/span/hash only; content schema unresolved | 50 | 50 |
| 26 | `Task States` | A | Only guarded three-state default-task constructor profile | 194 | 194 |
| 27 | `Paths` | O | Stored wrapper/span/hash only; content schema unresolved | 45 | 45 |
| 28 | `WBSLayouts` | O | Stored wrapper/span/hash only; content schema unresolved | 50 | 50 |
| 29 | `AVIExportProperty` | O | Stored wrapper/span/hash only; content schema unresolved | 675 | 31 |
| 30 | `CADProject` | O | Stored wrapper/span/hash only; content schema unresolved | 43 | 43 |
| 31 | `CuttingPlanes` | O | Stored wrapper/span/hash only; content schema unresolved | 53 | 53 |
| 32 | `Main Grids` | O | Stored wrapper/span/hash only; content schema unresolved | 74 | 75 |
| 33 | `User Grids` | O | Stored wrapper/span/hash only; content schema unresolved | 51 | 51 |
| 34 | `Issues` | O | Stored wrapper/span/hash only; content schema unresolved | 46 | 46 |
| 35 | `Documents` | O | Stored wrapper/span/hash only; content schema unresolved | 49 | 49 |
| 36 | `Rules` | O | Stored wrapper/span/hash only; content schema unresolved | 45 | 45 |
| 37 | `SkyBoxes` | O | Stored wrapper/span/hash only; content schema unresolved | 48 | 48 |
| 38 | `Resource States` | O | Stored wrapper/span/hash only; content schema unresolved | 274 | 274 |
| 39 | `Resource Interruption States` | O | Stored wrapper/span/hash only; content schema unresolved | 69 | 69 |
| 40 | `3D Notations` | O | Stored wrapper/span/hash only; content schema unresolved | 53 | 53 |
| 41 | `ResourceGroups` | O | Stored wrapper/span/hash only; content schema unresolved | 54 | 54 |
| 42 | `BinImages` | O | Stored wrapper/span/hash only; content schema unresolved | 168,355 | 49 |
| 43 | `LastTransactionID` | O | Stored wrapper/span/hash only; content schema unresolved | 43 | 44 |
| 44 | `PhysicalLights` | O | Stored wrapper/span/hash only; content schema unresolved | 57 | 57 |
| 45 | `Resources` | M | Recursive identities, bindings and cost storage; opaque fields and semantic units | 311,404 | 991 |
| 46 | `UniFolders` | O | Stored wrapper/span/hash only; content schema unresolved | 51 | 51 |
| 47 | `Thumbnail` | O | Stored wrapper/span/hash only; content schema unresolved | 102,981 | 56,961 |
| 48 | `PresentationData` | A | Only guarded task row-map/selection subsets used by authoring | 30,734 | 2,891 |

The [container and import paths](../crates/formats/src/sp_project.rs) distinguish M from G.
The metadata `sections.encoding` and section-hash fields retain the container's M/O classification even when the full reader processes geometry separately. An O-labelled geometry entry in that map is therefore not proof that the full import skipped it.
Hashes are of decoded bytes for M entries and stored wrapper/payload bytes for opaque entries; compare like representations.

## Field and behavior coverage

| Area | Present in audited source | Remaining work before full reading |
| --- | --- | --- |
| Container | Magic, admitted header string, 48 ordered entries, wrappers, spans, file/stream budgets and hashes | The four header bytes at offsets 16..20 are not interpreted; other layouts/versions, unknown-section schemas and external-dependency completeness |
| Task bodies | GUID, parent, name, Display ID, calendar reference, raw start/duration/status/date windows; framed assignments, links, direct costs and selected progress | Opaque header/dynamic/tail fields; additional nonempty lists are refused; complete constraints, milestones, history and task-kind semantics |
| Task collection lookup | Raw-GUID-sorted collection index validation and typed mismatch report | Lookup validity is independent of framing and native row identity |
| Assignments | Framed 235-byte records; resource/profile references; planned, remaining and actual quantity windows; utilization/rate windows; raw record hex | Confirm assignment identity; decode remaining opaque fields; distinguish utilization from monetary price and derived fallbacks from stored quantities |
| Relationships | Predecessor GUID, raw type and lag; endpoint existence checks | Public Link DTO omits full record identity/opaque bytes; signed/type semantics live in declared scheduling adapters, not a general native scheduler |
| Resources | Recursive identities, parents, kind/unit bytes, geometry bindings, user values and framed cost groups/items with raw windows | The two nonbinding GUID lists emit counts rather than individual entries; opaque identity/text meanings, semantic units, full cost families, currency identifiers and external catalog references |
| Appearance profiles | Framed definitions; counted names; 247..259-byte phase tails, action/growth/opacity/flags and raw tail | Unknown flag/inter-phase semantics, precedence, interpolation and unsupported profile families; successful decoding is not successful evaluation |
| Calendars | Decoded raw stream in metadata; downstream bounded calendar records, references, day types and selected exceptions | Unsupported inherited weekday/ancestor-exception behavior, overnight/other layouts, constraints, project time settings and timezone/DST meaning |
| Direct budgets / expenses | Observed singleton direct Fixed framing; observed type-5 reports with Fixed items and nil definition references | Other item/report families and nonnil definition references; currency/credit semantics and general actual/remaining attribution |
| Activity codes / user fields | Framed definition identities, hierarchy/value references, supported kinds/domains; task/resource values | Activity-code suffix and unused parameters remain opaque; unsupported variants/domains need controls |
| Geometry graph | Nodes, hierarchy, references, transforms, meshes, triangle indices, instance expansion, resource bindings and checked bounds | Other primitive/normal/attribute layouts, semantic units/axes and full native material/visibility rules |
| Geometry details | Supported normals are framed then skipped; geometry user values are parsed/type-domain checked but only a count is emitted | Retain and export normals and individual geometry user values with provenance; validation alone does not make them available to consumers |
| Textures / cached images | Definition/cache identity joins, paths as provenance, supported planar UV candidates, raw-channel candidates and JPEG framing | External assets are never opened by this parser; channel order, colour space, wrapping, orientation and complete native sampling remain unverified |
| Task States / PresentationData | Three-state constructor profile; constrained task row map/selection validation in authoring helpers | General catalog/layout/selection decoding and a common read-only view; do not promote constructor/deletion acceptance to complete section support |
| Other catalogs and project data | Original stored sections can be fingerprinted/preserved | O routes above require actual structured decoders and populated controls |

Source anchors:
[definitions](../crates/formats/src/sp_project/definitions.rs),
[resource tree](../crates/formats/src/sp_project/resource_tree.rs),
[task lookup](../crates/formats/src/sp_project/task_lookup.rs),
[actual expenses](../crates/formats/src/sp_project/actual_expenses.rs),
[geometry](../crates/formats/src/sp_project/scene_geometry.rs),
[scene graph](../crates/formats/src/sp_project/scene_graph.rs),
[appearance storage](../crates/formats/src/project_appearance.rs),
[calendar storage](../crates/formats/src/project_calendar.rs),
[textures](../crates/formats/src/project_textures.rs),
[constructor states](../crates/formats/src/sp_project/creation.rs) and
[presentation rows](../crates/formats/src/sp_project/deletion/presentation.rs).

## Coverage contract to implement in R0

This contract is proposed, not an API already shipped. Each reported section/record/field should carry source SHA, layout, reader identity, section name, stored span and decoded span where applicable, plus raw-value retention and linked evidence.

| Independent axis | Required distinction |
| --- | --- |
| Storage | Opaque bytes, validated framing, decoded fields, or explicit refusal |
| Meaning | Unresolved storage meaning, caller-declared interpretation, or scoped independent native control |
| References | Unchecked, valid within declared scope, missing, or ambiguous |
| Evaluation | Not requested, resolved, partial, or refused under the named policy |
| Comparison | Not compared, scoped match, mismatch, or unavailable oracle |

Do not collapse these axes into one `supported` boolean. A decoded profile can fail evaluation; a valid reference can still have unverified native meaning; a matching control covers only its admitted fields and conditions. Unknown ranges and over-limit/cancelled work must remain explicit.

## Decode status must be separate from evaluation status

The [analysis model](../crates/analysis/src/project_model.rs) has Strict, interval-preview and profile-effects-preview policies.
Strict accepts only scoped Install/Maintain signatures; a decoded custom or modified profile can still become `unsupported_profile`.
The [simulation](../crates/analysis/src/project_simulation/mod.rs) also distinguishes missing intervals, unsupported profile families and coincident mixed boundaries.
The [declared scheduler](../crates/analysis/src/project_schedule.rs) is not general native rescheduling or CPM.

A resource diagnostic should retain:

- Source SHA, reader binary/source identity, policy, focus and date mode.
- Task GUID + resource GUID + profile GUID + source assignment ordinal/offset; profile GUID alone is not an assignment ID.
- Definition presence, framing/decode status, raw signature/action and opaque ranges.
- Evaluation status and precise unresolved reason, separate from decode status.
- Native/export comparison status and any source conflict, without overwriting raw SP values.

The earlier report of 13 unsupported resources within a 31-resource comparison is **not** a newly verified count here. Its subset identities, reference exports, focus times and options must be recovered before diagnosing it.
The 79 resource records in Project A and that 31-resource comparison are different populations.
Likewise, mesh-instance counts and external scene-node counts are different measures; a numerical difference alone does not establish missing geometry.

## Reader-first backlog and dependencies

| Order | Work | Depends on | Acceptance evidence |
| --- | --- | --- | --- |
| R0 | Central per-section/per-field coverage and unknown-range registry shared by metadata/full readers | This inventory | Tested route-specific statuses; stored versus decoded offsets/hashes labelled; no invented overall percentage |
| R1 | Task/resource/profile diagnostic inventory; compare Strict and effects preview on identical inputs | R0 + retained reference inputs | Every selected resource accounted for; assignment identity retained; decode failures separated from policy refusals |
| R2 | Retain skipped geometry values, nonbinding resource GUID-list entries and opaque record spans; expose typed/raw provenance | R0 | Controls for normals/user values/GUID lists; values survive extraction; no consumer silently receives only a count |
| R3 | Complete state/time/calendar/constraint reading needed by 4D | R0, then R1 for affected assignments | Populated native controls, nondefault/inherited calendars and boundary cases; raw clocks preserved; unknown timezone policies explicit |
| R4 | Complete quantity/rate/currency/expense reference reading needed by 5D | R0 + R3 for time allocation | Per-field controlled values, multiple effective rates and supported units; unresolved money never presented as complete totals |
| R5 | Decode remaining project/catalog/reference sections, starting with baselines, animations and external-reference discovery | R0; R3/R4 for dependent semantics | Nonempty samples and record/reference inventories for each admitted family; names alone are not schema evidence |
| R6 | Complete remaining geometry/material/image families and reconcile external scene identities | R0 + R2; R5 for discovered assets/settings | GUID/instance/bounds/topology correspondence, populated codec/primitive/material controls and explicit missing-asset reports |
| R7 | Versioned corpus and hostile-input compatibility matrix | Each decoder's fixtures | Fresh per-field comparisons, truncation/corruption/limit/cancel checks and stable supported/unsupported outcomes |

R4 must first locate and validate the actual storage of referenced expense definitions. The current error mentions `GlobalExpenseDefs`; no section with that name was present in either 48-entry directory. Do not invent a new section or infer an empty catalog from that error.

Discover external references before adding any resolver. Reading a single SP file cannot supply assets that were never embedded. A future resolver must use explicit permitted locations, retain provenance and report missing/ambiguous targets.

Do not widen file/decoded-stream budgets just to make an unknown file pass. Current relevant caps include 256 MiB input; 16 MiB per retained metadata stream and 64 MiB combined; 192 MiB per geometry stream and 320 MiB combined; 64 MiB per texture stream in the full import route; bounded records and depth 64. Unknown formats, corrupt streams, missing dependencies, over-budget data and cancellation must remain distinguishable.

## Definition of completion

A full-reading claim must name the layout/version, populated feature families, corpus and required external assets.
For that declared scope:

1. Every section and record is accounted for with typed fields or explicitly unresolved spans. Unresolved semantic fields prevent a claim of complete semantic reading.
2. Counts, identities, units, dates, costs, geometry and reference joins match independent source-application/export evidence, with stated numeric tolerances.
3. Empty and populated controls are tested; an empty collection does not validate its populated layout.
4. Every refusal or partial result explains what is missing. Missing/unsupported values are never replaced by fabricated defaults or zero totals.
5. Original source hashes remain unchanged; source/binary/policy identities accompany comparisons.
6. Corrupt, truncated, oversized and cancelled inputs return bounded errors rather than a crash or hang.

This is a reader acceptance gate. General editing, rescheduling and native SP writing need separate acceptance; existing write controls do not prove that all file content is understood.

## Immediate next step

Implement R0's explicit coverage contract, then run R1's source-bound profile inventory on the actual reported subset and identical Strict/effects settings.
Group failures by raw profile family before selecting the next decoder. Keep unknown source values visible and preserve the original files.
