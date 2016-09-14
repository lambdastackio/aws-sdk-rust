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

// Sample access code and testing ground for the library.

// Allow unused_imports file wide because it allows you to comment out parts of the code without
// seeing warnings.

#![allow(unused_imports)]

extern crate aws_sdk_rust;
#[macro_use] extern crate lsio;
extern crate url;
//extern crate hyper;
extern crate rustc_serialize;
extern crate term;

use std::str;
use std::str::FromStr;

use rustc_serialize::json;

use aws_sdk_rust::aws::common::credentials::DefaultCredentialsProvider;
// NOTE: The bucket and obect use is using * but you may want to use specific items instead of everything
use aws_sdk_rust::aws::s3::bucket::*;
use aws_sdk_rust::aws::s3::object::*;
use aws_sdk_rust::aws::s3::acl::*;

use aws_sdk_rust::aws::common::region::Region;
use aws_sdk_rust::aws::s3::endpoint::{Endpoint, Signature};
use aws_sdk_rust::aws::s3::s3client::S3Client;

fn main() {
    // DefaultCredentialsProvider will end up cycling through the credentials provider list in
    // the following order:
    // 1. Environment - Checks the envrionment variables:
    //      AWS_ACCESS_KEY_ID
    //      AWS_SECRET_ACCESS_KEY
    //      AWS_SESSION_TOKEN
    // 2. Parameters passed in via ParametersProvider (see example below)
    // 3. Profile provider - ~/.aws/credentials
    // 4. IAM Provider

    // Option to initialize the ParametersProvider
    // Example of using parameters for passing the credentials.
    // use aws_sdk_rust::aws::common::credentials::{DefaultCredentialsProvider, ParametersProvider};
    //
    // let param_provider: Option<ParametersProvider>;
    // param_provider = Some(
    // ParametersProvider::with_params(
    // "<whatever your access_key_id>",
    // "<whatever your secret_access_key>",
    // None).unwrap()
    // );
    //
    // let provider = DefaultCredentialsProvider::new(param_provider).unwrap();
    //


    // Allow the defaults w/o ParametersProvider - pass in 'None' in ::new(None)
    let provider = DefaultCredentialsProvider::new(None).unwrap();

    // V4 is the default signature for AWS. However, other systems also use V2.
    let endpoint = Endpoint::new(Region::UsEast1, Signature::V2, None, None, None);
    let client = S3Client::new(provider, endpoint);

    // For cli version see s3lsio cli
    let bucket_name: &str = "cm2test6.some.test2";

    // repeat_color!(term::color::GREEN, "=", 80);

    println!("------------------------create_bucket----------------------------------------");
    let mut bucket = CreateBucketRequest::default();
    bucket.bucket = bucket_name.to_string();

    match client.create_bucket(&bucket) {
        Ok(bucket) => println_color!(term::color::GREEN, "{:#?}", bucket),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    println!("------------------------get_bucket_notification_configuration----------------");
    let mut bucket_notify = GetBucketNotificationConfigurationRequest::default();
    bucket_notify.bucket = bucket_name.to_string();

    match client.get_bucket_notification_configuration(&bucket_notify) {
        Ok(bucket) => println_color!(term::color::GREEN, "{:#?}", bucket),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    println!("------------------------get_bucket_logging----------------------------------");
    let mut bucket_logging = GetBucketLoggingRequest::default();
    bucket_logging.bucket = bucket_name.to_string();

    match client.get_bucket_logging(&bucket_logging) {
        Ok(bucket) => println_color!(term::color::GREEN, "{:#?}", bucket),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    println!("------------------------put_bucket_acl-------------------------------------");
    let mut put_bucket_acl = PutBucketAclRequest::default();
    put_bucket_acl.bucket = bucket_name.to_string();
    put_bucket_acl.acl = Some(CannedAcl::PublicRead);

    match client.put_bucket_acl(&put_bucket_acl) {
        Ok(bucket) => println_color!(term::color::GREEN, "{:#?}", bucket),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    println!("------------------------get_bucket_acl------------------------------------");
    let mut get_bucket_acl = GetBucketAclRequest::default();
    get_bucket_acl.bucket = bucket_name.to_string();

    match client.get_bucket_acl(&get_bucket_acl) {
        Ok(bucket) => println_color!(term::color::GREEN, "{:#?}", bucket),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    println!("------------------------put_object---------------------------------------");
    let mut put_object = PutObjectRequest::default();
    put_object.bucket = bucket_name.to_string();
    put_object.key = "mytest.txt".to_string();
    put_object.body = Some(b"this is a test.");

    match client.put_object(&put_object) {
        Ok(output) => println_color!(term::color::GREEN, "{:#?}", output),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    println!("------------------------get_object---------------------------------------");
    let mut get_object = GetObjectRequest::default();
    get_object.bucket = bucket_name.to_string();
    get_object.key = "mytest.txt".to_string();

    match client.get_object(&get_object) {
        Ok(output) => println_color!(term::color::GREEN, "\n\n{:#?}\n\n", str::from_utf8(&output.body).unwrap()),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    println!("------------------------put_object_acl-----------------------------------");
    let mut put_object_acl = PutObjectAclRequest::default();
    put_object_acl.bucket = bucket_name.to_string();
    put_object_acl.key = "mytest.txt".to_string();
    put_object_acl.acl = Some(CannedAcl::Private);

    match client.put_object_acl(&put_object_acl) {
        Ok(acl) => println_color!(term::color::GREEN, "{:#?}", acl),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    println!("------------------------get_object_acl-----------------------------------");
    let mut get_object_acl = GetObjectAclRequest::default();
    get_object_acl.bucket = bucket_name.to_string();
    get_object_acl.key = "mytest.txt".to_string();

    match client.get_object_acl(&get_object_acl) {
        Ok(acl) => {
            println_color!(term::color::GREEN, "{:#?}", acl);
            println!("----------JSON-----------");
            println_color!(term::color::GREEN, "{}", json::as_pretty_json(&acl));
        },
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    println!("------------------------head_bucket--------------------------------------");
    let bucket_head = HeadBucketRequest { bucket: bucket_name.to_string() };

    match client.head_bucket(&bucket_head) {
        Ok(head) => println_color!(term::color::GREEN, "{:#?}", head),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    println!("------------------------put_bucket_versioning----------------------------");
    let bucket_versioning = PutBucketVersioningRequest {
        bucket: bucket_name.to_string(),
        versioning_configuration: VersioningConfiguration {
            status: "Enabled".to_string(),
            mfa_delete: "".to_string(),
        },
        mfa: None,
        content_md5: None,
    };

    match client.put_bucket_versioning(&bucket_versioning) {
        Ok(version) => println_color!(term::color::GREEN, "{:#?}", version),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    println!("------------------------get_bucket_versioning--------------------------");
    let bucket_versioning = GetBucketVersioningRequest { bucket: bucket_name.to_string() };

    match client.get_bucket_versioning(&bucket_versioning) {
        Ok(version) => println_color!(term::color::GREEN, "{:#?}", version),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    println!("------------------------delete_object---------------------------------");
    let mut del_object = DeleteObjectRequest::default();
    del_object.bucket = bucket_name.to_string();
    del_object.key = "mytest.txt".to_string();

    match client.delete_object(&del_object) {
        Ok(output) => println_color!(term::color::GREEN, "{:#?}", output),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    println!("------------------------list_object_versions-------------------------");
    let mut bucket_versioning = ListObjectVersionsRequest::default();
    bucket_versioning.bucket = bucket_name.to_string();

    match client.list_object_versions(&bucket_versioning) {
        Ok(version) => println_color!(term::color::GREEN, "{:#?}", version),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    println!("------------------------list_objects--------------------------------");
    let mut list_objects = ListObjectsRequest::default();
    list_objects.bucket = bucket_name.to_string();
    // list_objects.version = Some(2);

    match client.list_objects(&list_objects) {
        Ok(objects) => {
            println_color!(term::color::GREEN, "{:#?}", objects);
            println!("----------JSON (serial)--");
            let encoded = json::encode(&objects).unwrap();
            println_color!(term::color::GREEN, "{:#?}", encoded);
            println!("----------JSON-----------");
            println_color!(term::color::GREEN, "{}", json::as_pretty_json(&objects));
        },
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    println!("------------------------delete_bucket-------------------------------");
    let bucket = DeleteBucketRequest { bucket: bucket_name.to_string() };

    match client.delete_bucket(&bucket) {
        Ok(bucket) => println_color!(term::color::GREEN, "{:#?}", bucket),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }

    println!("------------------------list_buckets-------------------------------");
    match client.list_buckets() {
        Ok(output) => {
            println_color!(term::color::GREEN, "{:#?}", output);
        },
        Err(error) => {
            println_color!(term::color::RED, "Error: {:#?}", error);
        },
    }

    // repeat_color!(term::color::GREEN, "=", 80);
}
