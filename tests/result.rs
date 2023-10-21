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
use tracer::prelude::ResultLog;

#[fixture]
fn r_ok<'a>() -> Result<&'a str, &'a str> {
    Result::Ok("ok")
}

#[fixture]
fn r_err<'a>() -> Result<&'a str, &'a str> {
    Result::Err("err")
}

#[rstest]
fn log_with_ok(r_ok: Result<&str, &str>) {
    let r = r_ok.log();
    assert_eq!(r, Ok("ok"));
}

#[rstest]
fn log_with_err(r_err: Result<&str, &str>) {
    let r = r_err.log();
    assert_eq!(r, Err("err"));
}

#[rstest]
fn log_format_with_ok(r_ok: Result<&str, &str>) {
    let r = r_ok.log_format(
        "Result is ok and value is: {v}",
        "Result is err and error is: {e}",
    );
    assert_eq!(r, Ok("ok"));
}

#[rstest]
fn log_format_with_err(r_err: Result<&str, &str>) {
    let r = r_err.log_format(
        "Result is ok and value is: {v}",
        "Result is err and error is: {e}",
    );
    assert_eq!(r, Err("err"));
}

#[rstest]
fn log_format_ok_with_ok(r_ok: Result<&str, &str>) {
    let r = r_ok.log_format_ok("Result is ok and value is: {v}");
    assert_eq!(r, Ok("ok"));
}

#[rstest]
fn log_format_ok_with_err(r_err: Result<&str, &str>) {
    let r = r_err.log_format_ok("Result is ok and value is: {v}");
    assert_eq!(r, Err("err"));
}
