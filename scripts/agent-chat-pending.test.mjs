#!/usr/bin/env node

import assert from "node:assert/strict";
import { test } from "node:test";

const pendingModuleUrl = new URL("../apps/desktop/src/agentChatPending.ts", import.meta.url);

const { agentChatPendingStageIndex } = await import(pendingModuleUrl);

test("selects staged chat pending messages by elapsed time", () => {
  assert.equal(agentChatPendingStageIndex(0, 4), 0);
  assert.equal(agentChatPendingStageIndex(2499, 4), 0);
  assert.equal(agentChatPendingStageIndex(2500, 4), 1);
  assert.equal(agentChatPendingStageIndex(6499, 4), 1);
  assert.equal(agentChatPendingStageIndex(6500, 4), 2);
  assert.equal(agentChatPendingStageIndex(11999, 4), 2);
  assert.equal(agentChatPendingStageIndex(12000, 4), 3);
  assert.equal(agentChatPendingStageIndex(90000, 4), 3);
});

test("handles empty stage lists defensively", () => {
  assert.equal(agentChatPendingStageIndex(5000, 0), 0);
});
