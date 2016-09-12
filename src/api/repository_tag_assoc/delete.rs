// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use backit::{responses};
use db::repositories::repository_tag_assoc as rt_assoc_repo;
use iron::{Request, Response, IronResult};
use router::Router;
use serde_json;
use std::error::Error;

// post /api/v1/tag
pub fn delete(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");
    let delete_id = req.extensions.get::<Router>().unwrap().find("id").unwrap().to_string();
    
    // first check if the assoc exist
    let assoc = match rt_assoc_repo::_get(db, &*delete_id) {
        // FIXME(jeremy): need to check the owner of this association through the owner of the repo.
        Ok(a) => a,
        Err(_) => return responses::bad_request("cannot delete a non existing association"),
    };
    
    // insert tag in databse + return inserted value
    match rt_assoc_repo::delete(db, &*assoc.id) {
        Ok(_) => responses::ok(serde_json::to_string(&assoc).unwrap()),
        Err(e) => responses::internal_error(e.description()),
    }
}
