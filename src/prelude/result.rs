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

/// A trait for `Result` to log the value and error of it.
/// If `Result` is `Ok`, the log level is trace while
/// if `Result` is `Err`, the log level is error.
pub trait ResultLog<T, E> {
    /// Simply logs the `Ok` and `Err`.
    fn log(self) -> Result<T, E>;

    /// Logs the `Ok` and `Err` with custom format.
    /// `{v}` in `ok` parameter will be replaced with
    /// the value of `Ok` and `{e}` in `err` parameter
    /// will be replaced with the value of `Err`.
    fn log_format(self, ok: &str, err: &str) -> Result<T, E>;

    // Logs the `Ok` with custom format. `{v}` in `ok`
    // parameter will be replaced with the value of `Ok`.
    fn log_format_ok(self, ok: &str) -> Result<T, E>;

    /// Logs the `Err` with custom format. `{e}` is `err`
    /// parameter will be replaced with the value of `Err`.
    fn log_format_err(self, err: &str) -> Result<T, E>;
}

impl<T, E> ResultLog<T, E> for Result<T, E>
where
    T: Debug,
    E: Debug,
{
    fn log(self) -> Result<T, E> {
        self.log_format("Ok({v:?})", "Err({e:?})")
    }

    fn log_format(self, ok: &str, err: &str) -> Result<T, E> {
        match self {
            Ok(v) => {
                let msg = ok.replace("{v}", &format!("{v:?}"));
                log::trace!("{msg}");
                Ok(v)
            }
            Err(e) => {
                let msg = err.replace("{e}", &format!("{e:?}"));
                log::error!("{msg}");
                Err(e)
            }
        }
    }

    fn log_format_ok(self, ok: &str) -> Result<T, E> {
        self.log_format(ok, "Err({e})")
    }

    fn log_format_err(self, err: &str) -> Result<T, E> {
        self.log_format("Ok({v})", err)
    }
}
