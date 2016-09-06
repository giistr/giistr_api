// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*
use api::context::Context;
use backit::{responses, json, time};
use db::models::RepositoryTagAssoc;
use db::repositories::repository_tag_assoc as rt_assoc_repo;
use iron::{Request, Response, IronResult};
use serde_json;
use std::convert::Into;
use std::error::Error;
use uuid::Uuid;

#[derive(Display, Debug, Eq, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct CreateRepositoryTagAssoc {
    pub repo_id: String,
    pub tag_id: String,
}

impl Into<RepositoryTagAssoc> for CreateRepositoryTagAssoc {
    fn into(self) -> Tag {
        let now = time::timestamp::now() as i32;
        Tag {
            id: Uuid::new_v4().to_string(),
            created_at: Some(now),
            updated_at: Some(now),
            repo_id: self.repo_id,
            tag_id: self.tag_id,
        }
    }
}

// post /api/v1/tag
pub fn create(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    // get the message from the body
    // it must contains exlicitly ONE CreateTag struct
    let mut crta = try_or_json_error!(json::from_body::<CreateRepositoryTagAssoc, _>(&mut req.body));
 
    // convert input to db models
    let mut rta: RepositoryTagAssoc = crta.into();

    // first test if this tag do not exist
    match rt_assoc_repo::get(db, &*rta.repo_id, &*rta.tag_id) {
        Ok(_) => return responses::bad_request("tag association already exist"),
        Err(_) => {/*nothing to do*/},
    };

    // insert tag in databse + return inserted value
    match tag_repo::create(db, t) {
        Ok(t) => responses::ok(serde_json::to_string(&t).unwrap()),
        Err(e) => responses::internal_error(e.description()),
    }
}

*/
