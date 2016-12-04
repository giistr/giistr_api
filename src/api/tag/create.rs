// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use backit::{responses, json, time};
use db::models::Tag;
use db::repositories::tag as tag_repo;
use iron::{Request, Response, IronResult};
use serde_json;
use std::convert::Into;
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct CreateTag {
    pub name: String,
}

impl Into<Tag> for CreateTag {
    fn into(self) -> Tag {
        let now = time::timestamp::now() as i32;
        Tag {
            id: Uuid::new_v4().to_string(),
            created_at: Some(now),
            updated_at: Some(now),
            user_id: Default::default(),
            name: self.name,
        }
    }
}

// post /api/v1/tag
pub fn create(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    // get the message from the body
    // it must contains exlicitly ONE CreateTag struct
    let mut ct = try_or_json_error!(json::from_body::<CreateTag, _>(&mut req.body));

    // trim tag name
    ct.name = ct.name.trim().into();
    if &*ct.name == "" {
        return responses::bad_request("tag name cannot be empty");
    }

    // convert input to db models
    let mut t: Tag = ct.into();
    // set the current user id to the tag user_id
    t.user_id = ctx.user.id;

    // first test if this tag do not exist
    match tag_repo::get_from_name_and_user_id(db, &*t.name, &*t.user_id) {
        Ok(_) => return responses::bad_request("tag already exist"),
        Err(_) => {/*nothing to do*/},
    };

    // insert tag in databse + return inserted value
    match tag_repo::create(db, t) {
        Ok(t) => responses::ok(serde_json::to_string(&t).unwrap()),
        Err(e) => responses::internal_error(e.description()),
    }
}
