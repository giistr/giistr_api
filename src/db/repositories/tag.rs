// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::{json, time};
use db::models::Tag;
use diesel::result::Error as DieselError;
use std::error::Error;
use uuid::Uuid;
//use diesel::sqlite::SqliteConnection;
use diesel::pg::PgConnection;

pub fn create(db: &mut PgConnection, mut t: Tag) -> Result<Tag, json::Error> {
    use diesel::{self, ExecuteDsl};
    use db::schemas::tags;

    // create some mandatory fields
    t.id = Uuid::new_v4().to_string();
    t.created_at = Some(time::timestamp::now() as i32);
    t.updated_at = Some(time::timestamp::now() as i32);

    match diesel::insert(&t).into(tags::table).execute(db) {
        Ok(_) => Ok(t),
        Err(e) => Err(json::Error::internal_error(e.description())),
    }
}

pub fn update(db: &mut PgConnection, mut t: Tag) -> Result<Tag, json::Error> {
    use diesel::SaveChangesDsl;
    t.updated_at = Some(time::timestamp::now() as i32);

    match t.save_changes::<Tag>(db) {
        Ok(_) => Ok(t),
        Err(e) => Err(json::Error::internal_error(e.description())),
    }
}

pub fn get(db: &mut PgConnection, get_id: &str) -> Result<Tag, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::tags::dsl::{tags, id};
    tags.filter(id.eq(get_id)).first::<Tag>(db)
}

pub fn get_from_name_and_user_id(db: &mut PgConnection, get_name: &str, get_user_id: &str) -> Result<Tag, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::tags::dsl::{tags, name, user_id};
    tags.filter(user_id.eq(get_user_id)).filter(name.eq(get_name)).first::<Tag>(db)
}

pub fn list(db: &mut PgConnection) -> Result<Vec<Tag>, DieselError> {
    use diesel::LoadDsl;
    use db::schemas::tags::dsl::tags;
    tags.load::<Tag>(db)
}

pub fn list_for_user_id(db: &mut PgConnection, list_user_id: &str) -> Result<Vec<Tag>, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::tags::dsl::{tags, user_id};
    tags.filter(user_id.eq(list_user_id)).load::<Tag>(db)
}

pub fn delete(db: &mut PgConnection, delete_id: &str)
              -> Result<usize, DieselError> {
    use diesel::{self, ExecuteDsl, FilterDsl, ExpressionMethods};
    use db::schemas::tags::dsl::{tags, id};
    diesel::delete(tags.filter(id.eq(delete_id))).execute(db)
}

pub fn get_all(db: &mut PgConnection, ids: &[String]) -> Result<Vec<Tag>, DieselError> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use db::schemas::tags::dsl::{tags, id};
    use diesel::pg::expression::dsl::any;
    tags.filter(id.eq(any(ids)))
        .load::<Tag>(db)
}
