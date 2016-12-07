// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use iron::{BeforeMiddleware, Request, IronResult};
use iron::status::Status;
use iron::error::IronError;
use std::error::Error;
use std::fmt;

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
    fn before(&self, _: &mut Request) -> IronResult<()> {
        Err(IronError::new(FakeError, (Status::Ok, "")))
    }
}
