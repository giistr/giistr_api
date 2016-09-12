// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use api::repository_tag_assoc::common::RepositoryTagAssocResponse;
use backit::responses;
use db::models::RepositoryTagAssoc;
use db::repositories::repository_tag_assoc as rt_assoc_repo;
use db::repositories::tag as tag_repo;
use db::repositories::repo as repo_repo;
use iron::{Request, Response, IronResult};
use router::Router;
use serde_json;
use std::convert::Into;
use std::error::Error;
use uuid::Uuid;

// get /api/v1/tags/repo/:id
pub fn list_for_repo(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");
    let repo_id = req.extensions.get::<Router>().unwrap().find("id").unwrap().to_string();

    // get all the tags associated to this repo
    let assocs = match rt_assoc_repo::get_tags_for_repo(db, &*repo_id) {
        Ok(t) => t,
        Err(e) => return responses::internal_error(e.description())
    };

    // make a vec wiht all the tags ids
    let tag_ids: Vec<String> = assocs.into_iter().map(|a| a.tag_id).collect();

    // get the concretes tags associated to theses ids
    match tag_repo::get_all(db, &tag_ids) {
        Ok(t) => responses::ok(serde_json::to_string(&t).unwrap()),
        Err(e) => responses::internal_error(e.description()),
    }
}
