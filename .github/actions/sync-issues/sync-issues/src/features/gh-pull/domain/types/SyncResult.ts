/**
 * SyncResult - Result of a sync operation
 */

export interface SyncResult {
  action: "created" | "updated" | "found";
  issueNumber: number | null;
  commentCount: number;
}