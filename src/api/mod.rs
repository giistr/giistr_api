// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::responses;
use iron::{Request, Response, IronResult};
use router::Router;

#[macro_use]
pub mod context;
pub mod tag;
pub mod user;

pub fn not_found(_: &mut Request) -> IronResult<Response> {
    responses::not_found("url not found on this server")
}

pub fn init() -> Router {
    router!(
        get "/api/v1/tag/:id" => wrap_ctx!(tag::get),
        get "/api/v1/tags" => wrap_ctx!(tag::list),
        put "/api/v1/tag" => wrap_ctx!(tag::update),
        post "/api/v1/tag" => wrap_ctx!(tag::create),
        delete "/api/v1/tag/:id" => wrap_ctx!(tag::delete),

        get "/api/v1/user/:id" => wrap_ctx!(user::get),
        get "/api/v1/users" => wrap_ctx!(user::list),

        any "/" => not_found,
        any "/*" => not_found,
    )
}
