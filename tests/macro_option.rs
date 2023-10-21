// Copyright 2023 Eray Erdin
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rstest::{fixture, rstest};
#[cfg(feature = "macro")]
use tracer::olog;

#[cfg(feature = "macro")]
#[fixture]
fn o_some<'a>() -> Option<&'a str> {
    Some("foo")
}

#[cfg(feature = "macro")]
#[fixture]
fn o_none<'a>() -> Option<&'a str> {
    None
}

#[cfg(feature = "macro")]
#[rstest]
fn log_format_with_some(o_some: Option<&str>) {
    let o = olog!(o_some, "The value is {v}");
    assert_eq!(o, Some("foo"));
}

#[cfg(feature = "macro")]
#[rstest]
fn log_format_with_none(o_none: Option<&str>) {
    let o = olog!(o_none, "The value is {v}");
    assert_eq!(o, None);
}

#[cfg(feature = "macro")]
#[rstest]
fn log_with_some(o_some: Option<&str>) {
    let o = olog!(o_some);
    assert_eq!(o, Some("foo"));
}

#[cfg(feature = "macro")]
#[rstest]
fn log_with_none(o_none: Option<&str>) {
    let o = olog!(o_none);
    assert_eq!(o, None);
}
