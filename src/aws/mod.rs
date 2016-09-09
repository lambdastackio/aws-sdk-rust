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
//!
//! NB: ####Endpoint is the only non-unit structure that is not JSON encodable/decodable due to
//! third party Url struct. A custom to_json trait would need to be implemented. Not real impactful
//! since this struct is mainly used for initial endpoint connections.
//!
//! NB: ####CompleteMultipartUploadRequest is *not* JSON decodable without implementing a custom to_json trait
//! because of Option<&'a [u8]>. You can still encode to JSON.
//!
//! NB: ####PutObjectRequest is *not* JSON decodable without implementing a custom to_json trait
//! because of Option<&'a [u8]>. You can still encode to JSON.
//!
//! NB: ####UploadPartRequest is *not* JSON decodable without implementing a custom to_json trait
//! because of Option<&'a [u8]>. You can still encode to JSON.
//!
//! Example JSON output: (see /src/main.rs for full examples)
//!    use rustc_serialize::json;
//!
//!    let mut list_objects = ListObjectsRequest::default();
//!    list_objects.bucket = bucket_name.to_string();
//!
//!    match client.list_objects(&list_objects) {
//!        Ok(objects) => {
//!            // Example of Serializing Rust struct
//!            println!("{:#?}", objects);
//!            // Example of converting to JSON. Can also call json::encode(&objects).unwrap();
//!            println!("{}", json::as_pretty_json(&objects));
//!        },
//!        Err(e) => println!("{:#?}", e)
//!    }
//!

/// `common` contains the type, struct, enum and impls that are common accross most requests
/// such as buckets, objects etc.
pub mod common;
/// `errors` contains the type, struct, enum and impls of different Error Types.
pub mod errors;
/// `s3` contains the type, struct, enum and impls of all S3 only related items. Also, it contains
/// `S3Client` which is the interface used by applications.
pub mod s3;
