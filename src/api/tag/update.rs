// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use backit::{responses, json};
use db::repositories::tag as tag_repo;
use iron::{Request, Response, IronResult};
use serde_json;
use std::error::Error;

#[derive(Display, Debug, Eq, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct UpdateTag {
    pub id: String,
    pub name: String,
}

// update /api/v1/tag
pub fn update(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    // one match only
    let mut ut = try_or_json_error!(json::from_body::<UpdateTag, _>(&mut req.body));

    // trim name
    ut.name = ut.name.trim().into();
    // ensure not empty
    if &*ut.name == "" {
        return responses::bad_request("tag name cannot be empty");
    }

    // get the message
    let mut t = match tag_repo::get(db, &*ut.id) {
        Ok(old) => {
            if &*ctx.user.id != &*old.user_id {
                return responses::bad_request("cannot update a tag owned by another user");
            } else {
                old
            }
        },
        Err(e) => return responses::bad_request(format!("tag do not exist, {}", e.description())),
    };

    // update the tag
    t.name = ut.name.clone();

    match tag_repo::update(db, t) {
        Ok(t) => responses::ok(serde_json::to_string(&t).unwrap()),
        Err(e) => responses::internal_error(e.description()),
    }
}
