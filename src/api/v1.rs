// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use api::{repo, tag, user, repository_tag_assoc, not_found};
use backit::responses;
use iron::Request;
use router::Router;

pub fn api() -> Router {
    router!(
        // tag crud
        get "/tag/:id" => wrap_ctx!(tag::get),
        get "/tags" => wrap_ctx!(tag::list),
        put "/tag" => wrap_ctx!(tag::update),
        post "/tag" => wrap_ctx!(tag::create),
        delete "/tag/:id" => wrap_ctx!(tag::delete),

        // user listing (for debuging)
        get "/user/:id" => wrap_ctx!(user::get),
        get "/users" => wrap_ctx!(user::list),

        // repositories crud
        get "/repo/:id" => wrap_ctx!(repo::get),
        get "/repos" => wrap_ctx!(repo::list),
        post "/repo" => wrap_ctx!(repo::create),
        delete "/repo" => wrap_ctx!(repo::delete),

        // tags associations
        // get the list of tags for a given repository
        get "/tags/repo/:id" => wrap_ctx!(repository_tag_assoc::list_for_repo),

        // get the list of repos for a tag
        get "/repos/tag/:id" => wrap_ctx!(repository_tag_assoc::list_for_tag),

        // add a tag for a repository
        post "/tag/:tag_id/repo/:repo_id" => wrap_ctx!(repository_tag_assoc::create),

        // delete a tag for a repository
        delete "/repo-tag-assoc/:id" => wrap_ctx!(repository_tag_assoc::delete),

        // options "/*" => |_: &mut Request| responses::ok(""),

        any "/" => not_found,
        any "/*" => not_found,
    )
}
