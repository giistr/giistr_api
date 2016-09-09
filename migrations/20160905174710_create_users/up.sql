CREATE TABLE users (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  created_at INT DEFAULT 0,
  updated_at INT DEFAULT 0,

  token_id VARCHAR(256) NOT NULL,
  github_user_id VARCHAR(256) NOT NULL
)
