// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::context::Context;
use backit::{responses, json};
use db::models::Repo;
use db::repositories::repo as repo_repo;
use iron::{Request, Response, IronResult};
use serde_json;
use std::convert::Into;
use std::error::Error;
use uuid::Uuid;

#[derive(Display, Debug, Eq, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct CreateRepo {
    pub github_repo_id: String,
}

impl Into<Repo> for CreateRepo {
    fn into(self) -> Repo {
        Repo {
            id: Uuid::new_v4().to_string(),
            created_at: None,
            updated_at: None,
            user_id: Default::default(),
            github_repo_id: self.github_repo_id,
        }
    }
}

// post /api/v1/repo
pub fn create(ctx: Context, req: &mut Request) -> IronResult<Response> {
    let db = &mut *ctx.db.get().expect("cannot get sqlite connection from the context");

    // get the message from the body
    // it must contains exlicitly ONE CreateRepo struct
    let cr = try_or_json_error!(json::from_body::<CreateRepo, _>(&mut req.body));

    // ensure repo id not empty
    if &*cr.github_repo_id == "" {
        return responses::bad_request("github_repo_id cannot be empty");
    }

    // convert input to db models
    let mut r: Repo = cr.into();
    // set the current user id to the tag user_id
    r.user_id = ctx.user.id;

    // first test if this tag do not exist
    match repo_repo::get_from_github_repo_id_and_user_id(db, &*r.github_repo_id, &*r.user_id) {
        Ok(_) => return responses::bad_request("repo already exist"),
        Err(_) => {/*nothing to do*/},
    };

    // insert tag in databse + return inserted value
    match repo_repo::create(db, r) {
        Ok(r) => responses::ok(serde_json::to_string(&r).unwrap()),
        Err(e) => responses::internal_error(e.description()),
    }
}
