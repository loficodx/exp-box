CREATE TABLE IF NOT EXISTS rooms (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  slug TEXT UNIQUE NOT NULL,
  title TEXT NOT NULL,
  category TEXT NOT NULL,
  difficulty TEXT NOT NULL,
  position INTEGER NOT NULL,
  description TEXT NOT NULL,
  flag_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS progress (
  session_id TEXT NOT NULL,
  room_id INTEGER NOT NULL REFERENCES rooms(id),
  solved_at TEXT,
  PRIMARY KEY (session_id, room_id)
);

INSERT INTO rooms (slug, title, category, difficulty, position, description, flag_hash)
VALUES (
  'rce',
  'Remote Code Execution',
  'rce',
  'easy',
  1,
  'A diagnostics utility exposes a dangerous endpoint. Exploit it to read the flag from the server filesystem.',
  'c3d14e1ee7e22b6de4ac1e6b98c98b71d9a15f487abe66738dc85684540d244b'
)
ON CONFLICT (slug) DO NOTHING;
