CREATE TABLE repos (
  id TEXT NOT NULL PRIMARY KEY,
  created_at INTEGER DEFAULT CURRENT_TIMESTAMP,
  updated_at INTEGER DEFAULT CURRENT_TIMESTAMP,

  user_id TEXT NOT NULL,
  github_repo_id TEXT NOT NULL
)
