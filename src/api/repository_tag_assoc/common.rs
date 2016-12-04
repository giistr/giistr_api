// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use db::models::{Tag, Repo, RepositoryTagAssoc};
use std::convert::From;

#[derive(Debug, Eq, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct RepositoryTagAssocResponse {
    pub assoc: RepositoryTagAssoc,
    pub repo: Repo,
    pub tag: Tag,
}

impl From<(RepositoryTagAssoc, Repo, Tag)> for RepositoryTagAssocResponse {
    fn from(vals: (RepositoryTagAssoc, Repo, Tag)) -> RepositoryTagAssocResponse {
        RepositoryTagAssocResponse {
            assoc: vals.0,
            repo: vals.1,
            tag: vals.2,
        }
    }
}
