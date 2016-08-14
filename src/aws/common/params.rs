/*
 Copyright 2016 LambdaStack All rights reserved.

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

 http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
*/

/*
 Portions borrowed from the rusoto project. See README.md
*/

//! Parameters for talking to query-based AWS services.
//!
//! Key-value pairs for AWS query requests.
//!
//! Supports optional parameters for calling SQS and ETS.

use std::collections::BTreeMap;

pub type Params = BTreeMap<String, String>;

/// Key:value pair for an service parameter.
pub trait ServiceParams {
    fn put(&mut self, key: &str, val: &str);
}

impl ServiceParams for Params {
    fn put(&mut self, key: &str, val: &str) {
        self.insert(key.into(), val.into());
    }
}
