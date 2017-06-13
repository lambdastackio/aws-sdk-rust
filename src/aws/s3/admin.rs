// Copyright 2017 LambdaStack All rights reserved.
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

use aws::common::params::*;
use aws::s3::endpoint::*;

/// Enum of output format (JSON or XML)
#[derive(Debug, Clone, PartialEq, RustcDecodable, RustcEncodable)]
pub enum AdminOutputType {
    Json,
    Xml,
}

/// Admin request for Ceph RGW Admin
#[derive(Debug, Clone, Default)]
pub struct AdminRequest {
    /// Defaults to GET
    pub method: Option<String>,
    /// Specify the path using the default 'admin'. For example, admin/bucket etc.
    pub admin_path: Option<String>,
    /// Allows for speacial path option that does NOT have a value but only a key like
    /// ```admin/user?quota``` where you would admin/user in admin_path and quota in path_options.
    pub path_options: Option<String>,
    /// User's ID
    pub uid: Option<String>,
    /// Name of bucket
    pub bucket: Option<String>,
    /// Name of Object
    pub object: Option<String>,
    /// Parameters used in the query string
    pub params: Params,
    /// Endpoint that can override the default
    pub endpoint: Option<Endpoint>,
    /// Access key ID
    pub access_key: Option<String>,
    /// Secret key ID
    pub secret_key: Option<String>,
    /// Output format (JSON or XML). Default is JSON
    pub format: Option<AdminOutputType>,
}

/// Default output of all admin functions
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct AdminOutput {
    /// Status code from the restful server
    pub code: u16,
    /// Payload of output
    pub payload: String,
    /// Output format (JSON or XML). Default is JSON
    pub format: AdminOutputType,
}

/// Ceph RGW Users
#[derive(Debug, Default, Clone, RustcDecodable, RustcEncodable)]
pub struct AdminUsers {
    /// Vec of users
    pub users: Vec<String>,
}

/// Ceph RGW Buckets
#[derive(Debug, Default, Clone, RustcDecodable, RustcEncodable)]
pub struct AdminBuckets {
    /// Vec of buckets
    pub buckets: Vec<String>,
}

/// Ceph RGW Usage
#[derive(Debug, Default, Clone, RustcDecodable, RustcEncodable)]
pub struct AdminUsage {
    /// Usage
    pub usage: String,
}

/// Ceph RGW Bucket Quota
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct AdminQuota {
    /// Quota enabled
    pub enabled: bool,
    /// Default -1 which indicates no quota for bucket sizes
    pub max_size_kb: i64,
    /// Default -1 which indicates no quota for number of total objects
    pub max_objects: i64,
}

impl Default for AdminQuota {
    fn default() -> AdminQuota {
        AdminQuota{ enabled: false, max_size_kb: -1, max_objects: -1 }
    }
}

/// Ceph RGW Bucket Metadata
#[derive(Debug, Default, Clone, RustcDecodable, RustcEncodable)]
pub struct AdminBucketMetadata {
    pub bucket: String,
    pub pool: String,
    pub index_pool: String,
    pub id: String,
    pub marker: String,
    pub owner: String,
    pub ver: String,
    pub master_ver: String,
    pub mtime: String, // Could be time
    pub max_marker: String,
    pub usage: AdminUsage,
    pub bucket_quota: AdminQuota,
}
