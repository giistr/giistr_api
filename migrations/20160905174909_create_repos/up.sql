CREATE TABLE repos (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  created_at INT DEFAULT CURRENT_TIMESTAMP,
  updated_at INT DEFAULT CURRENT_TIMESTAMP,

  user_id VARCHAR(36) NOT NULL,
  github_repo_id VARCHAR(256) NOT NULL,
  FOREIGN KEY(user_id) REFERENCES users(id)
)
