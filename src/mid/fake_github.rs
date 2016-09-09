// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::json::Error as JsonError;
use backit::middlewares::extract_postgres_from_request;
use db::models::User;
use db::repositories::user as user_repo;
//use diesel::sqlite::SqliteConnection;
use diesel::pg::PgConnection;
use iron::{BeforeMiddleware, Request, IronResult};
use iron::error::IronError;
use mid::GithubMid;
use uuid::Uuid;

pub struct FakeGithubMid;

fn make_github_info(db: &mut PgConnection, token: &str) -> Result<User, JsonError> {
    // first check if the token is in database
    // if the token exist, just consider it's still valid
    match user_repo::get_from_token(db, token) {
        // the user exist already with the same token,
        // let say it's safe enough for now
        Some(u) => Ok(u),
        // the user do not exist, first validate the token with slack api
        None => {
            let u = User::from_github_ids(&*Uuid::new_v4().to_string(), token);
            user_repo::create(db, u)
        }
    }
}

impl BeforeMiddleware for FakeGithubMid {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        // first extract the db
        let conn_wrapper = extract_postgres_from_request(req);
        let db = &mut *conn_wrapper.get().expect("cannot get sqlite connection from the context");

        match req.headers.get_raw("X-Github-Token") {
            Some(h) => {
                if h.len() != 1 {
                    let err = JsonError::bad_request("invalid X-Github-Token");
                    return Err(IronError::new(err.clone(), (err.status(), err.as_json())));
                }
                let token = match String::from_utf8(h[0].clone()) {
                    Ok(t) => t,
                    Err(_) => {
                        let err = JsonError::bad_request("invalid X-Github-Token");
                        return Err(IronError::new(err.clone(), (err.status(), err.as_json())));
                    }
                };
                let u = match make_github_info(db, &*token) {
                    Ok(u) => u,
                    Err(e) => return Err(IronError::new(e.clone(), (e.status(), e.as_json())))
                };
                req.extensions.insert::<GithubMid>(u);
                Ok(())
            },
            None => {
                let err = JsonError::bad_request("missing X-Github-Token");
                Err(IronError::new(err.clone(), (err.status(), err.as_json())))
            }
        }

    }
}
