# JavaScript and Electron application artifacts

> **Guide Resolution**: For deeper JavaScript and Electron workflows, use `read_file` to inspect `javascript-artifact-reconstruction.md`, `javascript-application-graph.md`, `javascript-application-workflows.md`, `javascript-route-inventory.md`, `javascript-recovery.md`, `javascript-runtime-observation.md`, `javascript-runtime-reconciliation.md`, `electron-observation.md`, `electron-fuses.md`, or `node-sea.md` in the `guides/` subfolder of this skill.

For captured or bundled JavaScript that needs readable source recovery, use `recover_javascript_sources` with the target path and absent `output_directory`. Supply related runtime/shared/lazy files in `additional_paths` when advertised, then analyze the returned `analysis_input` directory together. Each input retains its own snapshot, digest and byte-range provenance; inputs and recovered code are never executed. Recovery requires a caller-supplied Wakaru on Linux x64 (see `javascript-recovery.md` in `guides/`).

Use `analyze_javascript_application` directly on the operator-supplied ASAR or extracted tree. When the question concerns known source paths and the connected schema advertises `source_paths`, select exact inventoried paths such as `["package.json", "main.js", "preload.js"]`. Include supporting manifests or source maps explicitly; dependencies are not implicitly analyzed. REA retains the complete artifact inventory and identity but reads source text and analyzes only selected files. The result and summary record `source_selection`; files outside scope and their relationships remain unknown, so partial application coverage does not mean the selected source failed. Omit the selection when the question requires complete application analysis; `source_paths: []` is invalid. `detail: "summary"` reduces delivery and does not by itself reduce analysis work.

ASAR header/index admission defaults to 16 MiB / 16777216 bytes; an admission error names `asar.max_header_bytes` and its bounds (at most 256 MiB). Call `update_analysis_settings` with that setting, `action: "set"` and enough bytes, then retry the same full-archive request.

When size is unknown or large and the connected schema advertises `detail`, request `detail: "summary"`. This returns an `inspect_analysis_view` Evidence record with a distinct ID and a `normalized_result.parent_evidence_id` for the complete retained application analysis. Source-map contents are part of that static analysis. Read findings, coverage, limitations and unknowns, then follow a specific unanswered question.

## Inventory route candidates without returning the application graph

When the advertised schema supports `view`, use `detail: "summary"` with `view: {kind: "javascript-routes", collection: "routes", fields: ["path", "confidence"], limit: 20}`. The same view works with `analyze_web_bundle` on already selected captured sources. The complete source Evidence is retained; use its `parent_evidence_id` with `inspect_analysis_view` for `summary`, `routes`, `occurrences`, `sources`, or `unknowns`. Filter and batch IDs before requesting more fields, and follow `coverage.next_offset`.

## Read a module without repeating analysis

Use the actual returned parent ID, on the same connection, as `source.evidence_id`. When the module's node ID is not already known, page identities before selecting an item. Pages return only node IDs, paths and roles:

```json
{
  "source": {
    "kind": "retained-evidence",
    "evidence_id": "ev_ACTUAL_PARENT_ID"
  },
  "view": { "kind": "page", "collection": "modules", "offset": 0, "limit": 16 }
}
```

Select the relevant page entry by its exact `node_id` and recorded role:

```json
{
  "source": {
    "kind": "retained-evidence",
    "evidence_id": "ev_ACTUAL_PARENT_ID"
  },
  "view": {
    "kind": "item",
    "collection": "modules",
    "selector": { "node_id": "jag_node_ACTUAL_NODE_ID" }
  }
}
```

For canonical identities and retained source-range counts, use a typed reference page instead of reading every full item:

```json
{
  "source": {
    "kind": "retained-evidence",
    "evidence_id": "ev_ACTUAL_PARENT_ID"
  },
  "view": {
    "kind": "javascript",
    "collection": "modules",
    "projection": "references",
    "where": { "path": "main.js", "kind": "javascript-module" },
    "offset": 0,
    "limit": 16
  }
}
```

## Trace only the remaining question

Use `trace_application_feature` on the parent application Evidence for one literal node ID, route, string, API, IPC channel, module, or native export. Select `incoming` for relationships leading to the seed, `outgoing` for those leaving it, and `both` when the question needs both. Trace traversal includes containment relationships, so even a narrow seed can expand many nodes; a module item or page is preferable when its observations already answer the question.

## Version & export shape comparison

For version comparison, analyze each version once, then call `compare_application_versions`. Accept only its digest, source-map, structural fingerprint, or non-module semantic matches.

When the question asks how one exact exported callable's returned object shape changed, analyze each version once and then call `compare_javascript_export_shapes` with explicit module paths and export names.

## Reusing application Evidence

For exact file/native metadata or processing gaps, use `view: {kind: "javascript-facts", where: {paths: ["package.json"]}, property_keys: ["name", "operation", "analysis_status"], limit: 32}`. Rows retain canonical node/observation identity and original Evidence. `close_binary` clears retained references; export a bundle before closing, or supply portable inline Evidence in another connection.
