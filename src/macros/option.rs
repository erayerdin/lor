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

/// A macro to log the value of an Option. You can simply use it as `olog!(o)`
/// where `o` is type of `Option` or you can provide additional formatting if
/// the value is `Some` by using `olog!(o, "The value is {v}")`.
#[macro_export]
macro_rules! olog {
    ($o:expr, $some:literal) => {{
        use $crate::macros::log;

        match $o {
            Some(v) => {
                let msg = $some.replace("{v}", &format!("{:?}", v));
                log::trace!("{msg}");
                Some(v)
            }
            None => {
                log::trace!("None");
                None
            }
        }
    }};

    ($o:expr) => {{
        olog!($o, "Some({v})")
    }};
}
