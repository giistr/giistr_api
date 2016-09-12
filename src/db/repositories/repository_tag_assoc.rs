// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::json;
use db::models::RepositoryTagAssoc;
use diesel::result::Error as DieselError;
use std::error::Error;
use uuid::Uuid;
use diesel::pg::PgConnection;

pub fn create(db: &mut PgConnection, mut rta: RepositoryTagAssoc)
              -> Result<RepositoryTagAssoc, json::Error> {
    use diesel::{self, ExecuteDsl};
    use db::schemas::repository_tag_assocs;
    
    // create some mandatory fields
    rta.id = Uuid::new_v4().to_string();

    match diesel::insert(&rta).into(repository_tag_assocs::table).execute(db) {
        Ok(_) => Ok(rta),
        Err(e) => Err(json::Error::internal_error(e.description())),
    }
}

pub fn get(db: &mut PgConnection, get_repo_id: &str, get_tag_id: &str)
           -> Result<RepositoryTagAssoc, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::repository_tag_assocs::dsl::{repository_tag_assocs, repo_id, tag_id};
    repository_tag_assocs
        .filter(repo_id.eq(get_repo_id))
        .filter(tag_id.eq(get_tag_id)).first::<RepositoryTagAssoc>(db)
}

pub fn delete(db: &mut PgConnection, delete_repo_id: &str, delete_tag_id: &str)
              -> Result<usize, DieselError> {
    use diesel::{self, ExecuteDsl, FilterDsl, ExpressionMethods};
    use db::schemas::repository_tag_assocs::dsl::{repository_tag_assocs, repo_id, tag_id};
    diesel::delete(
        repository_tag_assocs
            .filter(tag_id.eq(delete_tag_id))
            .filter(repo_id.eq(delete_repo_id))
    ).execute(db)
}

pub fn get_tags_for_repo(db: &mut PgConnection, get_repo_id: &str)
           -> Result<Vec<RepositoryTagAssoc>, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::repository_tag_assocs::dsl::{repository_tag_assocs, repo_id};
    repository_tag_assocs
        .filter(repo_id.eq(get_repo_id))
        .load::<RepositoryTagAssoc>(db)
}

pub fn get_repos_for_tag(db: &mut PgConnection, get_tag_id: &str)
           -> Result<Vec<RepositoryTagAssoc>, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::repository_tag_assocs::dsl::{repository_tag_assocs, tag_id};
    repository_tag_assocs
        .filter(tag_id.eq(get_tag_id))
        .load::<RepositoryTagAssoc>(db)
}
