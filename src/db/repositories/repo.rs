// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::{json, time};
use db::models::Repo;
use diesel::result::Error as DieselError;
use std::error::Error;
use uuid::Uuid;
use diesel::pg::PgConnection;

pub fn create(db: &mut PgConnection, mut r: Repo) -> Result<Repo, json::Error> {
    use diesel::{self, ExecuteDsl};
    use db::schemas::repos;

    // create some mandatory fields
    r.id = Uuid::new_v4().to_string();
    r.created_at = Some(time::timestamp::now() as i32);
    r.updated_at = Some(time::timestamp::now() as i32);

    match diesel::insert(&r).into(repos::table).execute(db) {
        Ok(_) => Ok(r),
        Err(e) => Err(json::Error::internal_error(e.description())),
    }
}

pub fn get(db: &mut PgConnection, get_id: &str) -> Result<Repo, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::repos::dsl::{repos, id};
    repos.filter(id.eq(get_id)).first::<Repo>(db)
}

pub fn get_from_github_repo_id_and_user_id(db: &mut PgConnection, get_github_repo_id: &str, get_user_id: &str) -> Result<Repo, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::repos::dsl::{repos, github_repo_id, user_id};
    repos
        .filter(user_id.eq(get_user_id))
        .filter(github_repo_id.eq(get_github_repo_id))
        .first::<Repo>(db)
}

pub fn list_for_user_id(db: &mut PgConnection, list_user_id: &str) -> Result<Vec<Repo>, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::repos::dsl::{repos, user_id};
    repos.filter(user_id.eq(list_user_id)).load::<Repo>(db)
}

pub fn delete(db: &mut PgConnection, delete_id: &str)
              -> Result<usize, DieselError> {
    use diesel::{self, ExecuteDsl, FilterDsl, ExpressionMethods};
    use db::schemas::repos::dsl::{repos, id};
    diesel::delete(repos.filter(id.eq(delete_id))).execute(db)
}
