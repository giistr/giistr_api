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
pub mod repo;
pub mod tag;
pub mod user;
pub mod repository_tag_assoc;

pub fn not_found(_: &mut Request) -> IronResult<Response> {
    responses::not_found("url not found on this server")
}

pub fn init() -> Router {
    router!(
        // tag crud
        get "/api/v1/tag/:id" => wrap_ctx!(tag::get),
        get "/api/v1/tags" => wrap_ctx!(tag::list),
        put "/api/v1/tag" => wrap_ctx!(tag::update),
        post "/api/v1/tag" => wrap_ctx!(tag::create),
        delete "/api/v1/tag/:id" => wrap_ctx!(tag::delete),

        // user listing (for debuging)
        get "/api/v1/user/:id" => wrap_ctx!(user::get),
        get "/api/v1/users" => wrap_ctx!(user::list),

        // repositories crud
        get "/api/v1/repo/:id" => wrap_ctx!(repo::get),
        get "/api/v1/repos" => wrap_ctx!(repo::list),
        post "/api/v1/repo" => wrap_ctx!(repo::create),
        delete "/api/v1/repo" => wrap_ctx!(repo::delete),

        // tags associations
        // get the list of tags for a given repository
        get "/api/v1/tags/repo/:id" => wrap_ctx!(repository_tag_assoc::list_for_repo),

        // get the list of repos for a tag
        get "/api/v1/repos/tag/:id" => wrap_ctx!(repository_tag_assoc::list_for_tag),
        
        // add a tag for a repository
        post "/api/v1/tag/:tag_id/repo/:repo_id" => wrap_ctx!(repository_tag_assoc::create),

        // delete a tag for a repository
        delete "/api/v1/repo-tag-assoc/:id" => wrap_ctx!(repository_tag_assoc::delete),

        any "/" => not_found,
        any "/*" => not_found,
    )
}
