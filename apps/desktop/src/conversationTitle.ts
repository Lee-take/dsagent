const CJK_TITLE_MAX_CHARS = 32;
const LATIN_TITLE_MAX_CHARS = 64;

export function summarizeConversationTitleFromText(value: string): string {
  const normalized = value
    .replace(/```[\s\S]*?```/g, " ")
    .replace(/https?:\/\/\S+/gi, " ")
    .replace(/\s+/g, " ")
    .trim();
  const firstClause =
    normalized
      .split(/[。！？!?；;，,\n]/)
      .find((part) => part.trim().length > 0)
      ?.trim() || normalized;
  const withoutLeadIn = firstClause
    .replace(/^(请你|请帮我|帮我|麻烦你|能不能|可以帮我|我想要|我想|我要|请|帮忙)\s*/u, "")
    .replace(/^(please|help me|can you|could you|i want to|i need to)\s+/i, "")
    .trim();
  const semanticTitle =
    withoutLeadIn
      .split(/(?:并且|并|然后|同时|以及|and then|and also|then)/iu)
      .find((part) => part.trim().length > 0)
      ?.trim() || withoutLeadIn;
  const title = semanticTitle || firstClause;
  const hasCjk = /[\u3400-\u9fff]/u.test(title);
  if (hasCjk) {
    return Array.from(title).slice(0, CJK_TITLE_MAX_CHARS).join("");
  }
  return title
    .split(/\s+/)
    .slice(0, 10)
    .join(" ")
    .slice(0, LATIN_TITLE_MAX_CHARS);
}

export function derivePersistedConversationTitle({
  firstUserMessage,
  manualTitle,
  storedTitle,
}: {
  firstUserMessage: string;
  manualTitle: boolean;
  storedTitle: string;
}): string {
  const normalizedStoredTitle = storedTitle.trim();
  if (manualTitle && normalizedStoredTitle) {
    return normalizedStoredTitle;
  }

  const sourceText = firstUserMessage.trim() || normalizedStoredTitle;
  return summarizeConversationTitleFromText(sourceText);
}
