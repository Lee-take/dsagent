#!/usr/bin/env node

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

const [app, types, commands, lifecycle] = await Promise.all([
  readFile("apps/desktop/src/App.tsx", "utf8"),
  readFile("apps/desktop/src/types.ts", "utf8"),
  readFile("apps/desktop/src-tauri/src/commands.rs", "utf8"),
  readFile("apps/desktop/src-tauri/src/kernel/goal_lifecycle.rs", "utf8"),
]);

for (const status of [
  "proposed",
  "blocked",
  "validated",
  "frozen",
  "verification_blocked",
  "complete",
]) {
  assert.match(types, new RegExp(`\\|? \\"${status}\\"`));
}

assert.match(app, /goal_envelope: response\.goal_envelope/);
assert.match(app, /goal_projection: response\.goal_projection/);
assert.match(app, /message\.goal_projection\?\.status/);
assert.match(app, /message\.goal_projection\.reason_codes\.join/);
assert.match(app, /message\.goal_projection\.revision/);
assert.match(app, /message\.goal_projection\.fingerprint/);
assert.doesNotMatch(
  app,
  /invoke[^\n]*(complete_goal|record_goal|completion_receipt|validation_receipt|authority_fingerprint)/i,
);

assert.match(commands, /#\[serde\(default, skip_deserializing\)\]\s+pub goal_projection/);
assert.match(commands, /reconcile_agent_goal_projection/);
assert.match(commands, /record_goal_completion_for_tool_invocation/);
assert.doesNotMatch(commands, /pub fn (complete_goal|record_goal_completion_evidence)/);

const uiProjectionStart = lifecycle.indexOf("pub struct GoalEnvelopeUiProjection");
const uiProjectionEnd = lifecycle.indexOf("}\n", uiProjectionStart);
assert.ok(uiProjectionStart >= 0 && uiProjectionEnd > uiProjectionStart);
const uiProjection = lifecycle.slice(uiProjectionStart, uiProjectionEnd);
assert.doesNotMatch(
  uiProjection,
  /authority|context_fingerprint|target_bindings|provider|claim|secret/i,
);

console.log("GoalEnvelope UI and completion authority boundary checks passed");
