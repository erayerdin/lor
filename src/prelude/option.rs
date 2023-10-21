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

use std::fmt::Debug;

/// A trait for `Option` to log the value of it.
/// It always uses trace level.
pub trait OptionLog<T> {
    /// Logs the value of the option with custom
    /// format. `{v}` in `some` parameter will be
    /// replaced with the value of `Some`.
    fn log_format(self, some: &str) -> Option<T>;

    fn log(self) -> Option<T>;
}

impl<T> OptionLog<T> for Option<T>
where
    T: Debug,
{
    fn log_format(self, some: &str) -> Option<T> {
        match self {
            Some(v) => {
                let msg = some.replace("{v}", &format!("{v:?}"));
                log::trace!("{msg}");
                Some(v)
            }
            None => None,
        }
    }

    fn log(self) -> Option<T> {
        self.log_format("Some({v})")
    }
}
