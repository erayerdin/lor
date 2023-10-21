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

#[cfg(feature = "macro")]
use lor::rlog;
use rstest::{fixture, rstest};

#[cfg(feature = "macro")]
#[fixture]
fn r_ok<'a>() -> Result<&'a str, &'a str> {
    Ok("ok")
}

#[cfg(feature = "macro")]
#[fixture]
fn r_err<'a>() -> Result<&'a str, &'a str> {
    Err("err")
}

#[cfg(feature = "macro")]
#[rstest]
fn log_format_with_ok(r_ok: Result<&str, &str>) {
    let r = rlog!(r_ok, ok = "Result is ok: {v}", err = "Result is err: {e}");
    assert_eq!(r, Ok("ok"));
}

#[cfg(feature = "macro")]
#[rstest]
fn log_format_with_err(r_err: Result<&str, &str>) {
    let r = rlog!(r_err, ok = "Result is ok: {v}", err = "Result is err: {e}");
    assert_eq!(r, Err("err"));
}

#[cfg(feature = "macro")]
#[rstest]
fn log_format_ok_with_ok(r_ok: Result<&str, &str>) {
    let r = rlog!(r_ok, ok = "Result is ok: {v}");
    assert_eq!(r, Ok("ok"));
}

#[cfg(feature = "macro")]
#[rstest]
fn log_format_ok_with_err(r_err: Result<&str, &str>) {
    let r = rlog!(r_err, ok = "Result is ok: {v}");
    assert_eq!(r, Err("err"));
}

#[cfg(feature = "macro")]
#[rstest]
fn log_format_err_with_ok(r_ok: Result<&str, &str>) {
    let r = rlog!(r_ok, err = "Result is error: {e}");
    assert_eq!(r, Ok("ok"));
}

#[cfg(feature = "macro")]
#[rstest]
fn log_format_err_with_err(r_err: Result<&str, &str>) {
    let r = rlog!(r_err, err = "Result is error: {e}");
    assert_eq!(r, Err("err"));
}

#[cfg(feature = "macro")]
#[rstest]
fn log_with_ok(r_ok: Result<&str, &str>) {
    let r = rlog!(r_ok);
    assert_eq!(r, Ok("ok"));
}

#[cfg(feature = "macro")]
#[rstest]
fn log_with_err(r_err: Result<&str, &str>) {
    let r = rlog!(r_err);
    assert_eq!(r, Err("err"));
}
