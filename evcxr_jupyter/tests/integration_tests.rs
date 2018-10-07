// Copyright 2018 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::process;

// Checks that our binary can be executed. Due to
// https://github.com/rust-lang/rust/issues/45601 it's way too easy for us to
// end up with a binary that can't be executed (without LD_LIBRARY PATH or
// similar). Currently any use of custom derive, even in dev dependencies gives
// us a bad binary.
#[test]
fn test_binary_execution() {
    let output = process::Command::new(
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("evcxr_jupyter"),
    ).arg("--help")
    .env_remove("LD_LIBRARY_PATH")
    .output()
    .unwrap();
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    assert_eq!(stderr, "");
    if !stdout.contains("To install, run") {
        panic!("Unexpected output:\n{:?}", stdout);
    }
}
