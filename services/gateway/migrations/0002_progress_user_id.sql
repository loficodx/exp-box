-- Replace anonymous session_id with authenticated user_id.
-- Local dev only — existing anonymous progress rows are not preserved.
CREATE TABLE IF NOT EXISTS progress_new (
  user_id  TEXT    NOT NULL,
  room_id  INTEGER NOT NULL REFERENCES rooms(id),
  solved_at TEXT,
  PRIMARY KEY (user_id, room_id)
);

DROP TABLE IF EXISTS progress;

ALTER TABLE progress_new RENAME TO progress;
