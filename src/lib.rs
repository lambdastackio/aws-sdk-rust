// Copyright 2016 LambdaStack All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// NOTE: This attribute only needs to be set once.
#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://lambdastackio.github.io/aws-sdk-rust/aws_sdk_rust/aws/index.html")]

#[macro_use]
extern crate log;
extern crate chrono;
extern crate openssl;
extern crate regex;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate url;
extern crate xml;
extern crate httparse;

#[macro_use]
extern crate hyper;

// Only aws crate is documented, not the dependents.
pub mod aws;

// Hide the http mod since it may change or go away.
#[doc(hidden)]
pub mod http;
