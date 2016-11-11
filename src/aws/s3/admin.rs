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

// Portions borrowed from the rusoto project. See README.md
//

#![allow(unused_variables)]
#![allow(unused_mut)]

use aws::s3::endpoint::*;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub enum AdminOutputType {
    Json,
    Plain,
    Xml,
    None,
}

#[derive(Debug, Clone, Default)]
pub struct AdminRequest {
    pub admin: Option<String>,
    pub bucket: Option<String>,
    pub object: Option<String>,
    pub params: Option<String>,
    pub endpoint: Option<Endpoint>,
    pub uid: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub format: Option<AdminOutputType>,
}

#[derive(Debug, Clone, Default, RustcDecodable, RustcEncodable)]
pub struct AdminOutput {
    pub payload: Option<String>,
    pub format: Option<AdminOutputType>,
}
