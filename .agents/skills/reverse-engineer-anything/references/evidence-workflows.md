# Evidence, comparison, and verification workflows

Evidence-producing tools return the complete Evidence record directly in MCP `structuredContent` and CLI JSON. Read `normalized_result` for operation data, and cite `evidence_id` when another tool or your explanation needs a stable reference. Preserve authority, limitations, coverage, and residual unknowns. If a result is incomplete or paginated, continue only when the remaining data matters to the task; do not fetch a bundle or resource merely to read a result already returned inline. When complete `inspect_binary_layout` or `analyze_javascript_application` Evidence is already retained, call `inspect_analysis_view` for a summary, one section/symbol/module, or a stable page instead of repeating the producer.

Completed `inspect_artifact` inventory also supports a summary and `view: {kind: "inventory", collection: "occurrences", logical_paths: ["package.json", "main.js"]}`. Omit `logical_paths` to page all available occurrences; `[]` selects none. Other inventory collections are `nodes`, `edges` and `integrity_contradictions`. Keep `source_complete` separate from page exhaustion: a missing match in partial source Evidence does not establish absence. Use the returned parent Evidence ID for subsequent views and export before closing the session when the source must survive it.

For native procedure/name/string inventories, select `detail: "summary"` when the connected tool advertises it and the task needs counts or selected rows. Use its parent Evidence ID with `view: {kind: "native-inventory", offset: 0, limit: 32, fields: ["value"]}`. Search tools retain the existing pattern/mode/case/document selection. Omit `fields` for complete records; `record.address` and `source_index` remain on selected rows. Parent Evidence ID plus source index preserves same-address alternatives. Follow `coverage.next_offset` only as far as the question needs; exhausting retained rows does not establish complete program discovery.

Use `record_unknown` to track unresolved questions in the user's investigation and name the authority or environment still required. Supply supporting and contradicting evidence IDs. It updates the session registry directly. Use `update_unknown` with the current revision; reread after a stale revision instead of retrying blindly. Only qualifying observed evidence can verify a resolution.

Use comparisons with complete, compatible page sets when claiming equivalence or absence:

- `compare_artifacts` compares inventory pages by occurrence path, content, metadata, and graph relations.
- `compare_functions` compares explicit function Evidence, not fuzzy whole-binary matches. It rejects an `analyze_function` summary reply; pass the summary's `parent_evidence_id` as a `retained-evidence` reference or the complete dossier.
- `compare_bundles` compares canonical bundle membership and unknown history.
- `find_changed_behavior` combines existing comparisons; static differences remain candidates, not causal proof.
- `build_call_path` needs explicit function dossiers from one artifact/provider; an incomplete frontier makes absence unknown.
- `correlate_static_and_runtime` uses explicit mappings; matching patterns are hypotheses, not causality.
- `verify_reconstruction` evaluates a finite typed specification. A pass covers only declared comparable claims, not global equivalence (for details, read `reconstruction-coverage.md` and `reconstruction-obligation-ledgers.md` in `guides/`).

Use `inspect_analysis_view` with `view: {kind: "api-surface"}` on retained `inspect_web_network_capture` HAR Evidence to group observed request lines by method, origin and path template. Format matches are inferences, not handler identity or schemas. Group pages cite parent record ordinals; consult record coverage and exclusions before claiming completeness. No requests are replayed. When the connected schema advertises `include_query_parameter_names`, select it in the view for distinct observed names with original record ordinals.

Use `inspect_analysis_view` with `view: {kind: "environment-manifest"}` on the same retained HAR Evidence to project the primary origin's request lines into named tools with path templates, parameters, observed query names, value-free body shapes and record pointers for an agent environment.

Use `view: {kind: "environment-state"}` with the manifest's `template_rules` to extract the data those reads observed instead: entity types and identity fields, entities with recorded values, documents, and per-tool render plans checked against the recordings.

Use `view: {kind: "environment-coverage"}` with the same rules to see what an environment built from a capture would serve: read tools answered from the state or replayed with their reason, and each write tool served or withheld with a hint about what to record next.

For a PE/ELF export question, `inspect_native_exports` accepts `detail: "summary"` and retains the complete table. Query `inspect_analysis_view` with `view: {kind: "native-exports", collection: "exports", names: ["selected_name"], fields: ["name"], offset: 0, limit: 32}` and the reported parent Evidence ID. All same-name versions and aliases survive; use `source_index` with the parent ID for stable identity. `collection: "issues"` pages interpretation failures.
