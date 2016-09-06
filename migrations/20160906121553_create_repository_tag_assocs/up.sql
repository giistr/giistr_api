CREATE TABLE repository_tag_assocs (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  repo_id VARCHAR(36) NOT NULL,
  tag_id VARCHAR(36) NOT NULL,
  FOREIGN KEY(repo_id) REFERENCES repos(id),
  FOREIGN KEY(tag_id) REFERENCES tags(id)
)
