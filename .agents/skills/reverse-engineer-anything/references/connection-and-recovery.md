# Connection and task recovery

Read this guide only for an observed connection, command, approval or provider blocker. Keep the user's target and question in view. Explain what prevented the operation and the next useful step; after repair, continue the investigation.

Commands below use an installed `rea` CLI (or `npx.cmd -y rea-agents@6.8.0` on Windows PowerShell when no global CLI is installed). Follow the connected server's schemas and the installed CLI's help. Tool-name prefixes belong to the client: find the canonical REA operation in its actual tool listing and server association before concluding that it is absent. For detailed client configuration specifics, use `read_file` to inspect `client-configuration.md` and `installation.md` in the `guides/` subfolder of this skill.

## Choose the remedy from the observed failure

| Observation | Meaning and next step |
| --- | --- |
| No REA tools in the active session | Inspect registration with `rea doctor --client antigravity --json` (or substitute the active client ID). A skill installation alone does not register MCP. |
| Missing, malformed or stale registration | Prepare a scoped setup plan; apply only authorized changes with backups. |
| Aligned registration, but no connected tools | Restart/reconnect the client. Doctor checks registration; it cannot establish the current agent's connection. Inspect the client's launch error if reconnection fails. |
| MCP connection dropped or REA could not start | If the connection drops, retry that call once only when it is safe to repeat or known not to have started. If it may already have changed the target, or the same failure returns, tell the user REA's connection failed and ask them to restart the MCP client. Run `rea doctor` when REA could not start or tools stay missing after restart. |
| `MCP tool call requires approval, but approval policy is never` | The client rejected the call before REA executed it. Do not install an engine or repeat setup to solve this client-policy decision. |
| REA returns `provider_unavailable` or an unavailable tool entry | Read that operation's provider diagnostics or `binary_session.result.tool_availability`. Use the operation's availability, reason, and remediation; a complete tool catalog does not imply every operation is available. |
| `rea` is missing in a child shell, or Node is incompatible | Inspect the actual shell PATH and launcher/runtime. A prefix installation may be absent from a shell's PATH; this does not establish a broken MCP registration. |

Static JavaScript and ordinary ZIP inventory need neither Hopper nor Ghidra. HAR inspection is offline and does not require a live browser. Native decompilation, Android analysis and firmware extraction have their own bring-your-own prerequisites. Use supported inventory/static evidence when it answers part of the question, and name the unavailable evidence separately. Do not substitute another target. Label observations from ordinary shell tools as such; only actual REA output supplies REA Evidence.

## Distinguish client and server deadlines

A host message such as `timed out awaiting tools/call` with no REA result means the client's deadline expired before a result arrived. It does not establish that REA finished, canceled successfully, or released all resources. Do not repeat the unchanged analysis. Inspect available session/provider state and retained Evidence only when useful; an idle `binary_session` is not proof that every target-free operation stopped. Never invent a missing Evidence reference.

A returned REA `provider_timeout` instead means the **server/provider** reached its deadline. Follow its `remediation.action`, reuse accessible partial Evidence, and resolve reported incomplete cleanup before retrying. Change a request field or a `REA_*` setting only when the returned remediation explicitly recommends that change. For static JavaScript, `analysis_timeout_ms` (CLI `rea analyze-javascript-application "<target-path>" --analysis-timeout-ms <milliseconds>`) controls each source's startup, analysis and transfer deadline in **milliseconds**. Increase that budget or reduce semantic work / select exact inventoried `source_paths` when the task permits. Omit `source_paths` for all sources; `[]` is invalid. `detail: "summary"` reduces delivery, not analysis work.

## Recovery ladder

Follow the returned remediation. Change a request field or a `REA_*` setting only when that text explicitly recommends the change. Restart REA when it says the value is read at startup.

- **MCP connection dropped**: retry that call once only when it is safe to repeat or known not to have started. If it may already have changed the target, or it fails again, tell the user REA's connection failed and ask them to restart the MCP client.
- **REA could not start its connection**: retry once, then tell the user and ask them to restart the MCP client. Run `rea doctor` when REA could not start or tools stay missing after that restart.
- **`cleanup_incomplete`**, or retained cleanup owners in the error details: retry that exact cleanup owner. Do not start new work that needs those resources until that owner confirms release.
- **Capacity**, when the result is `resource_constraint` and `retryable` is true: wait for the active admission to finish, then retry within the reported limits. Do not restart REA to free the slot.
- **`configuration_invalid`**: correct the named setting in the environment that starts REA (the shell or the MCP registration), then restart REA.
- **`invalid_request`**: correct the reported fields from the current schema. Do not repeat the unchanged call.

## Repair registration when needed

```powershell
npx.cmd -y rea-agents@6.8.0 doctor --client antigravity --json
npx.cmd -y rea-agents@6.8.0 setup --client antigravity --skill=false --dry-run --json
```

Use the current supported client's ID: `antigravity`, `claude_code`, `claude_desktop`, `codex`, `cursor`, `gemini_cli`, `windsurf`, `devin`, `opencode`, `copilot_cli`, `commandcode`, `qwen_code`, `vscode`, `trae`, `grok_build`, `omp`, `pi`, `hermes`, `qoder`, `droid`, `zcode`, `dsh`, `freebuff`, or `grok_bot`. If unknown, inspect `doctor --json` before choosing a scope.

Show the plan's exact paths, backups, and changes. Obtain approval for writes when that concrete scope is not already authorized, then apply the same scope with `--yes`. Restart/reconnect, verify tools actually appear, and resume the original task.

## Repair an editable REA limit

If `details.limit` names an editable setting and its bounds, call `update_analysis_settings` directly with a `changes` entry containing `setting`, `action: "set"` and `value`. `inspect_analysis_settings` is optional when those facts are already present. Retry the same input after work and cleanup settle. Runtime defaults affect new work only; explicit per-call arguments take precedence and must be corrected directly when selected. For full limit specifications, read `limits-and-recovery.md` in the `guides/` subfolder of this skill.

## Continue through an installed CLI

For a user-supplied JavaScript tree or ASAR, store output files in the conversation's `scratch/` directory (or `$env:TEMP`) and save complete Evidence before reading a view:

```powershell
npx.cmd -y rea-agents@6.8.0 analyze-javascript-application "<target-app-path>" --json > "$env:TEMP\app-evidence.json"
```

Then inspect the saved Evidence file using `rea inspect-analysis-view` with `{"source": {"kind": "evidence-file", "path": "<path-to-app-evidence.json>"}, "view": {"kind": "summary"}}` (see `cli.md` in `guides/` and `javascript-applications.md` in `references/`). Preserve graph coverage, limitations and unknowns.
