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

//! Contains both `Library` and `Client` documentation.

/// `common` contains the type, struct, enum and impls that are common accross most requests
/// such as buckets, objects etc.
pub mod common;
/// `credentials` contains the type, struct, enum and impls that are credentials related.
pub mod credentials;
/// Functions needed for percent-encoding
pub mod encode;
/// `region` contains the type, struct, enum and impls for Region related functions.
pub mod region;
/// `xmlutil` contains the type, struct, enum and impls that are common XML parsing and writing.
pub mod xmlutil;
/// `signature` contains the type, struct, enum and impls that are signature related (V2 and V4).
/// This is also where `SignedRequest` lives which is the core for requests in `S3Client`.
pub mod signature;
/// `params` contains the type, struct, enum and impls that related to URI parameters.
pub mod params;
/// `request` contains the type, struct, enum and impls that are HTTP Request related.
pub mod request;
/// `macros` contains the macros defined for the library.
pub mod macros;
