#!/usr/bin/env node

import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";

const checks = [
  {
    name: "live sk-style API key",
    pattern: /(^|[^A-Za-z0-9])(sk-[A-Za-z0-9]{16,})(?![A-Za-z0-9])/g,
  },
  {
    name: "non-empty DEEPSEEK_API_KEY assignment",
    pattern:
      /\bDEEPSEEK_API_KEY\s*=\s*(?:"[^"\r\n]+"|'[^'\r\n]+'|sk-[A-Za-z0-9]{16,}|[A-Za-z0-9_-]{20,})/g,
  },
];

const selfTestCount = runCheckSelfTests();
const files = execFileSync(
  "git",
  ["ls-files", "-z", "--cached", "--others", "--exclude-standard"],
  {
    encoding: "buffer",
  },
)
  .toString("utf8")
  .split("\0")
  .filter(Boolean);

const findings = [];

for (const file of files) {
  const buffer = readFileSync(file);
  if (buffer.includes(0)) {
    continue;
  }

  const content = buffer.toString("utf8");
  for (const check of checks) {
    check.pattern.lastIndex = 0;
    for (const match of content.matchAll(check.pattern)) {
      findings.push({
        file,
        line: lineNumberForIndex(content, match.index ?? 0),
        check: check.name,
      });
    }
  }
}

if (findings.length > 0) {
  console.error("Secret scan failed. Candidate secrets were found:");
  for (const finding of findings) {
    console.error(`${finding.file}:${finding.line} ${finding.check}`);
  }
  console.error("Candidate values are intentionally not printed.");
  process.exit(1);
}

console.log(
  JSON.stringify(
    {
      ok: true,
      files_scanned: files.length,
      self_tests: selfTestCount,
      checks: checks.map((check) => check.name),
    },
    null,
    2,
  ),
);

function lineNumberForIndex(value, index) {
  let line = 1;
  for (let cursor = 0; cursor < index; cursor += 1) {
    if (value[cursor] === "\n") {
      line += 1;
    }
  }
  return line;
}

function runCheckSelfTests() {
  const fakeKey = "sk-" + "1234567890abcdef";
  const envKey = "DEEPSEEK_API_KEY";
  const cases = [
    {
      content: `${envKey}=${fakeKey}`,
      expected: true,
    },
    {
      content: `$env:${envKey} = "${fakeKey}"`,
      expected: true,
    },
    {
      content: `Authorization: Bearer ${fakeKey}`,
      expected: true,
    },
    {
      content: `${envKey}=`,
      expected: false,
    },
    {
      content: '$env:DEEPSEEK_API_KEY = Read-Host "DeepSeek API key"',
      expected: false,
    },
  ];

  for (const testCase of cases) {
    const matched = checks.some((check) => matches(check, testCase.content));
    if (matched !== testCase.expected) {
      throw new Error("Secret scan self-test failed.");
    }
  }

  return cases.length;
}

function matches(check, content) {
  check.pattern.lastIndex = 0;
  return check.pattern.test(content);
}
