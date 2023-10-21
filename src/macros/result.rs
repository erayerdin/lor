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

#[macro_export]
macro_rules! rlog {
    ($r:expr, ok=$ok:literal, err=$err:literal) => {{
        use $crate::macros::log;

        match $r {
            Ok(v) => {
                let msg = $ok.replace("{v}", &format!("{:?}", v));
                log::trace!("{}", msg);
                Ok(v)
            }
            Err(e) => {
                let msg = $err.replace("{e}", &format!("{:?}", e));
                log::error!("{}", msg);
                Err(e)
            }
        }
    }};

    ($r:expr, ok=$ok:literal) => {{
        $crate::rlog!($r, ok = $ok, err = "Err({e})")
    }};

    ($r:expr, err=$err:literal) => {{
        $crate::rlog!($r, ok = "Ok({v})", err = $err)
    }};
}
