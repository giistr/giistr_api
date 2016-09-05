// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use backit::responses;
use db::repositories::tag as tag_repo;
use iron::{Request, Response, IronResult};
use router::Router;
use serde_json;
use std::error::Error;

// get /api/v1/tag/:id
pub fn get(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");
    let id = req.extensions.get::<Router>()
        .unwrap().find("id").unwrap().to_string();

    match tag_repo::get(db, &*id) {
        Ok(t) => responses::ok(serde_json::to_string(&t).unwrap()),
        Err(e) => responses::not_found(format!("id do not exist in database {}", e.description())),
    }
}
