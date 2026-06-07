/**
 * SyncResult - Result of a sync operation
 */

export interface SyncResult {
  action: "created" | "updated" | "skipped" | "found";
  issueNumber: number | null;
  commentCount: number;
}