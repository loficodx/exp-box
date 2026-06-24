CREATE TABLE IF NOT EXISTS room_accounts (
  user_id TEXT PRIMARY KEY,
  display_password_value TEXT NOT NULL,
  updated_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP)
);
