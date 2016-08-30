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

//! Each of the modules listed below contain `Client` and `Library` documentation. At the top of
//! each file the documentation identifies if the given source file is mainly used for `Client`
//! or by the `Library`. Of course, we welcome *ALL* pull requests so digging into the `Library`
//! documentation is encouraged to get an idea how `aws-sdk-rust` works.
//!
//! However, if you're only interested in how to use it from your app then skip the docs that
//! start with `Library` and focus on those that start with `Client`.

/// `common` contains the type, struct, enum and impls that are common accross most requests
/// such as buckets, objects etc.
pub mod common;
/// `errors` contains the type, struct, enum and impls of different Error Types.
pub mod errors;
/// `s3` contains the type, struct, enum and impls of all S3 only related items. Also, it contains
/// `S3Client` which is the interface used by applications.
pub mod s3;
