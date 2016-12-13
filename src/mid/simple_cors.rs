// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use iron::{BeforeMiddleware, Request, IronResult};
use iron::status::Status;
use iron::error::IronError;
use iron::headers::ContentType;
use iron::headers::AccessControlAllowOrigin;
use iron::headers::AccessControlAllowMethods;
use iron::headers::AccessControlAllowHeaders;
use iron::modifiers::Header;
use std::error::Error;
use std::fmt;
use unicase::UniCase;

pub struct SimpleCors;

#[derive(Debug)]
pub struct FakeError;

impl Error for FakeError {
    fn description(&self) -> &str { "" }

    fn cause(&self) -> Option<&Error> { None }
}

impl fmt::Display for FakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl BeforeMiddleware for SimpleCors {
    fn before(&self, req : &mut Request) -> IronResult<()> {
        use iron::method::Method::*;
        match req.method {
            Options => {
                let content_type = Header(ContentType::json());
                let allow_origin = Header(AccessControlAllowOrigin::Any);
                let allow_method = Header(AccessControlAllowMethods(vec![Get, Post, Put, Delete, Options]));
                let allow_headers = Header(AccessControlAllowHeaders(vec![
                    UniCase("x-github-token".to_owned()),
                    UniCase("content-type".to_owned())
                ]));
                let modifiers = (Status::Ok, content_type, allow_origin, allow_method, allow_headers);
                Err(IronError::new(FakeError, modifiers))
            },
            _ => Ok(()),
        }
    }
}
