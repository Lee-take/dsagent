#!/usr/bin/env node

import assert from "node:assert/strict";
import { test } from "node:test";

const titleModuleUrl = new URL("../apps/desktop/src/conversationTitle.ts", import.meta.url);

const { derivePersistedConversationTitle, summarizeConversationTitleFromText } =
  await import(titleModuleUrl);

test("keeps mixed Chinese and English app names in conversation titles", () => {
  assert.equal(
    summarizeConversationTitleFromText(
      "现在测试在桌面创建一个 PowerPoint 文件，文件名“DS Agent PPT 测试.pptx”，第一页标题写“DS Agent 测试”。",
    ),
    "现在测试在桌面创建一个 PowerPoint 文件",
  );
  assert.equal(
    summarizeConversationTitleFromText(
      "现在测试在桌面创建一个 Excel 文件，文件名“DS Agent Excel 测试.xlsx”。",
    ),
    "现在测试在桌面创建一个 Excel 文件",
  );
  assert.equal(
    summarizeConversationTitleFromText(
      "现在测试在桌面创建一个 Word 文档，文件名“DS Agent 测试.docx”。",
    ),
    "现在测试在桌面创建一个 Word 文档",
  );
});

test("repairs old auto-generated truncated persisted titles", () => {
  assert.equal(
    derivePersistedConversationTitle({
      firstUserMessage:
        "现在测试在桌面创建一个 PowerPoint 文件，文件名“DS Agent PPT 测试.pptx”。",
      manualTitle: false,
      storedTitle: "现在测试在桌面创建一个 Po",
    }),
    "现在测试在桌面创建一个 PowerPoint 文件",
  );
});

test("keeps user-renamed conversation titles unchanged", () => {
  assert.equal(
    derivePersistedConversationTitle({
      firstUserMessage:
        "现在测试在桌面创建一个 PowerPoint 文件，文件名“DS Agent PPT 测试.pptx”。",
      manualTitle: true,
      storedTitle: "我的 PPT 测试",
    }),
    "我的 PPT 测试",
  );
});
