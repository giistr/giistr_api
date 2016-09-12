// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use backit::responses;
use iron::{Request, Response, IronResult};
use mount::Mount;

#[macro_use]
pub mod context;
pub mod repo;
pub mod tag;
pub mod user;
pub mod repository_tag_assoc;
pub mod v1;

pub fn not_found(_: &mut Request) -> IronResult<Response> {
    responses::not_found("url not found on this server")
}

pub fn init() -> Mount {
    let mut mount = Mount::new();
    mount.mount("/api/v1", v1::api());
    mount.mount("/*", router!(
        any "/" => not_found,
        any "/*" => not_found,
    ));
    return mount;
}
