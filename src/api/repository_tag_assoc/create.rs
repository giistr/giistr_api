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

#[derive(Display, Debug, Eq, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct CreateRepositoryTagAssoc {
    pub repo_id: String,
    pub tag_id: String,
}

impl Into<RepositoryTagAssoc> for CreateRepositoryTagAssoc {
    fn into(self) -> RepositoryTagAssoc {
        RepositoryTagAssoc {
            id: Uuid::new_v4().to_string(),
            repo_id: self.repo_id,
            tag_id: self.tag_id,
        }
    }
}

// post /api/v1/tag/:tag_id/repo/:repo_id
pub fn create(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");
    let crta = CreateRepositoryTagAssoc {
        repo_id: req.extensions.get::<Router>().unwrap().find("repo_id").unwrap().to_string(),
        tag_id: req.extensions.get::<Router>().unwrap().find("tag_id").unwrap().to_string(),
    };
    
    // convert input to db models
    let rta: RepositoryTagAssoc = crta.into();

    // first test if this tag do not exist
    match rt_assoc_repo::get(db, &*rta.repo_id, &*rta.tag_id) {
        Ok(_) => return responses::bad_request("tag association already exist"),
        Err(_) => {/*nothing to do*/},
    };

    // get the tag for this id
    let t = match tag_repo::get(db, &*rta.tag_id) {
        Ok(t) => {
            println!("{}, {}", &*t.user_id, &*ctx.user.id);
            if &*t.user_id == &*ctx.user.id { t }
            else { return responses::bad_request("cannot create association with another user tag") }
        },
        Err(e) => return responses::internal_error(e.description()),
    };

    let r = match repo_repo::get(db, &*rta.repo_id) {
        Ok(r) => {
            println!("{}, {}", &*r.user_id, &*ctx.user.id);
            if &*r.user_id == &*ctx.user.id { r }
            else { return responses::bad_request("cannot create association with another user repo") }
        },
        Err(e) => return responses::internal_error(e.description()),
    };

    
    // insert tag in databse + return inserted value
    match rt_assoc_repo::create(db, rta) {
        Ok(rta) => {
            responses::ok(
                serde_json::to_string(
                    &RepositoryTagAssocResponse::from((rta, r, t))
                ).unwrap()
            )
        },
        Err(e) => responses::internal_error(e.description()),
    }
}
