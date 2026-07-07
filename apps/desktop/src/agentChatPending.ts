const DEFAULT_PENDING_STAGE_DELAYS_MS = [0, 2500, 6500, 12000];

export function agentChatPendingStageIndex(
  elapsedMs: number,
  stageCount: number,
  stageDelaysMs: number[] = DEFAULT_PENDING_STAGE_DELAYS_MS,
): number {
  if (stageCount <= 0) {
    return 0;
  }

  const boundedElapsedMs = Math.max(0, elapsedMs);
  let selectedIndex = 0;
  for (let index = 0; index < Math.min(stageCount, stageDelaysMs.length); index += 1) {
    if (boundedElapsedMs >= stageDelaysMs[index]) {
      selectedIndex = index;
    }
  }

  return Math.min(selectedIndex, stageCount - 1);
}

export function agentChatPendingStageDelaysMs(stageCount: number): number[] {
  return DEFAULT_PENDING_STAGE_DELAYS_MS.slice(0, Math.max(0, stageCount));
}
