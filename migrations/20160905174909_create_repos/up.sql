CREATE TABLE repos (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  created_at INT DEFAULT 0,
  updated_at INT DEFAULT 0,

  user_id VARCHAR(36) NOT NULL,
  github_repo_id VARCHAR(256) NOT NULL,
  repository_name VARCHAR(256) NOT NULL,
  user_login VARCHAR(256) NOT NULL,
  FOREIGN KEY(user_id) REFERENCES users(id)
)
