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
use serde_json;
use std::error::Error;

// get /api/v1/tags
pub fn list(ctx: Context, _: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");
    match repo_repo::list_for_user_id(db, &*ctx.user.id) {
        Ok(l) => responses::ok(serde_json::to_string(&l).unwrap()),
        Err(e) => responses::internal_error(e.description()),
    }
}
