// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use backit::responses;
use db::repositories::repository_tag_assoc as rt_assoc_repo;
use db::repositories::repo as repo_repo;
use iron::{Request, Response, IronResult};
use router::Router;
use serde_json;
use std::error::Error;

// get /api/v1/tags/repo/:id
pub fn list_for_tag(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");
    let tag_id = req.extensions.get::<Router>().unwrap().find("id").unwrap().to_string();

    // get all the tags associated to this repo
    let assocs = match rt_assoc_repo::get_repos_for_tag(db, &*tag_id) {
        Ok(t) => t,
        Err(e) => return responses::internal_error(e.description())
    };

    // make a vec wiht all the tags ids
    let repo_ids: Vec<String> = assocs.into_iter().map(|a| a.repo_id).collect();

    // get the concretes tags associated to theses ids
    match repo_repo::get_all(db, &repo_ids) {
        Ok(t) => responses::ok(serde_json::to_string(&t).unwrap()),
        Err(e) => responses::internal_error(e.description()),
    }
}
