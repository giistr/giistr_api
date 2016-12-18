// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::schemas::*;
use diesel::ExpressionMethods;

#[derive(Debug, Eq, PartialEq, Default, Clone, AsChangeset, Identifiable, Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
#[changeset_for(users)]
pub struct User {
    pub id: String,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,

    pub token_id: String,
    pub github_user_id: String,
}

impl User {
    pub fn from_github_ids<S1, S2>(user_id: S1, token_id: S2) -> User
        where S1: Into<String>, S2: Into<String> {
        User {
            id: Default::default(),
            created_at: None,
            updated_at: None,
            github_user_id: user_id.into(),
            token_id: token_id.into(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Default, Clone, AsChangeset, Identifiable, Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "repos"]
#[changeset_for(repos)]
pub struct Repo {
    pub id: String,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,

    pub user_id: String,
    pub github_repo_id: i32,
    pub repository_name: String,
    pub user_login: String,
}

#[derive(Debug, Eq, PartialEq, Default, Clone, AsChangeset, Identifiable, Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "tags"]
#[changeset_for(tags)]
pub struct Tag {
    pub id: String,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,

    pub user_id: String,
    pub name: String,
}

#[derive(Debug, Eq, PartialEq, Default, Clone, AsChangeset, Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "repository_tag_assocs"]
#[changeset_for(repository_tag_assocs)]
pub struct RepositoryTagAssoc {
    pub id: String,
    pub repo_id: String,
    pub tag_id: String,
}
