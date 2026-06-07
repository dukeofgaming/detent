/**
 * NodeIdToNumericId - Convert GitHub node ID to numeric ID
 */

export function nodeIdToNumericId(nodeId: string): string {
  if (!nodeId.includes("_")) return nodeId;
  const match = nodeId.match(/^IC_kw[A-Za-z0-9]+_?(.+)$/);
  return match ? match[1] : nodeId;
}