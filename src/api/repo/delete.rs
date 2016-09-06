// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use backit::responses;
use db::repositories::repo as repo_repo;
use iron::{Request, Response, IronResult};
use router::Router;
use serde_json;
use std::error::Error;

// delete /api/v1/repo/:id
pub fn delete(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");
    let id = req.extensions.get::<Router>()
        .unwrap().find("id").unwrap().to_string();

    let r = match repo_repo::get(db, &*id) {
        Ok(t) => {
            if &*t.user_id != &*ctx.user.id {
                return responses::bad_request("you cannot delete a tag from another user");
            }
            t
        },
        Err(e) => return responses::not_found(format!("id do not exist in database {}", e.description())),
    };

    match repo_repo::delete(db, &*id) {
        Ok(_) => responses::ok(serde_json::to_string(&r).unwrap()),
        Err(e) => responses::internal_error(format!("unable to delete tag, {}", e.description()))
    }
}
