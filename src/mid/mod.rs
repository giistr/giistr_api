// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use self::fake_github::FakeGithubMid;
pub use self::github::GithubMid;
pub use self::simple_cors::SimpleCors;

pub mod simple_cors;
pub mod fake_github;
pub mod github;
