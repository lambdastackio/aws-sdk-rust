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
#![allow(unused_variables)]  // Mainly got tired of looking at warnings so added this :)

extern crate aws_sdk_rust;
#[macro_use]
extern crate lsio;
extern crate url;
//extern crate hyper;
extern crate rustc_serialize;
extern crate term;
extern crate md5;

use std::io;
use std::io::{Read, Seek, SeekFrom, BufReader};
use std::path::Path;
use std::fs::File;
use std::str;
use std::str::FromStr;
// NOTE: Have to add `use std::iter;` if using repeat macros
use std::iter;

use rustc_serialize::json;
use rustc_serialize::base64::{ToBase64, STANDARD};

use lsio::commands::run_cli;

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
    let bucket_name: &str = "cm2test6";
    let width: usize = 120;

    // NOTE: repeat_color and println_color are macros from the lsio library

    repeat_color!(term::color::GREEN, "=", "Start", width);


    repeat_color_with_ends!(term::color::WHITE, "-", "create_bucket", "", "", width);

    let mut bucket = CreateBucketRequest::default();
    bucket.bucket = bucket_name.to_string();

    match client.create_bucket(&bucket) {
        Ok(bucket) => println_color!(term::color::GREEN, "{:#?}", bucket),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    repeat_color!(term::color::WHITE, "-", "get_bucket_notification_configuration", width);

    let mut bucket_notify = GetBucketNotificationConfigurationRequest::default();
    bucket_notify.bucket = bucket_name.to_string();

    match client.get_bucket_notification_configuration(&bucket_notify) {
        Ok(bucket) => println_color!(term::color::GREEN, "{:#?}", bucket),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    repeat_color!(term::color::WHITE, "-", "get_bucket_logging", width);

    let mut bucket_logging = GetBucketLoggingRequest::default();
    bucket_logging.bucket = bucket_name.to_string();

    match client.get_bucket_logging(&bucket_logging) {
        Ok(bucket) => println_color!(term::color::GREEN, "{:#?}", bucket),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    repeat_color!(term::color::WHITE, "-", "put_bucket_acl", width);

    let mut put_bucket_acl = PutBucketAclRequest::default();
    put_bucket_acl.bucket = bucket_name.to_string();
    put_bucket_acl.acl = Some(CannedAcl::PublicRead);

    match client.put_bucket_acl(&put_bucket_acl) {
        Ok(bucket) => println_color!(term::color::GREEN, "{:#?}", bucket),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    repeat_color!(term::color::WHITE, "-", "get_bucket_acl", width);

    let mut get_bucket_acl = GetBucketAclRequest::default();
    get_bucket_acl.bucket = bucket_name.to_string();

    match client.get_bucket_acl(&get_bucket_acl) {
        Ok(bucket) => println_color!(term::color::GREEN, "{:#?}", bucket),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    repeat_color!(term::color::WHITE, "-", "put_object", width);

    let mut put_object = PutObjectRequest::default();
    put_object.bucket = bucket_name.to_string();
    put_object.key = "mytest.txt".to_string();
    put_object.body = Some(b"this is a test.");

    match client.put_object(&put_object) {
        Ok(output) => println_color!(term::color::GREEN, "{:#?}", output),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    repeat_color!(term::color::WHITE, "-", "get_object", width);

    let mut get_object = GetObjectRequest::default();
    get_object.bucket = bucket_name.to_string();
    get_object.key = "mytest.txt".to_string();

    match client.get_object(&get_object) {
        Ok(output) => println_color!(term::color::GREEN, "\n\n{:#?}\n\n", str::from_utf8(&output.body).unwrap()),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    repeat_color!(term::color::WHITE, "-", "create_multipart_upload", width);

    // This is the first thing that needs to be done. Initiate the request and get the uploadid.
    // Generate a test file of 8MB in size...
    let test_abort: bool = false;
    let file_size: u16 = 8;
    // NOTE: .gitignore has this file name as an ignore. If you change this file name then change .gitignore if you also want to issue a PR.
    let file_name: &str = "test.multipart.upload.file";
    // NOTE: The temp file will be removed after the test. If you want to keep the file then set this to false.
    let file_remove: bool = true;
    let file_create: bool = true;

    let mut create_multipart_upload_output: Option<MultipartUploadCreateOutput> = None;
    let mut create_multipart_upload = MultipartUploadCreateRequest::default();
    create_multipart_upload.bucket = bucket_name.to_string();
    create_multipart_upload.key = file_name.to_string();

    match client.multipart_upload_create(&create_multipart_upload) {
        Ok(output) => {
            println_color!(term::color::GREEN, "{:#?}", output);
            create_multipart_upload_output = Some(output);
            // Only for *nix based systems - the following command
            if file_create {
                let result = run_cli(format!("dd if=/dev/zero ibs={}m count=1 of={}", file_size, file_name.to_string()));
            }
        },
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }

    repeat_color!(term::color::WHITE, "-", "upload_part", width);

    // You will need to decide on your upload part sizes. The minimum size is 5MB per chunk exept
    // the last one with a maximum size of 5GB per part. The total file size can't exceed 5TB.
    // Also, you can break the parts up into at most 10,000 parts. You want to split your file
    // into the size that works for your use case, bandwidth, machine etc.
    //
    // This of course should go in a loop OR create threads for the different parts upload.
    // IMPORTANT: The *final* (complete_multipart_upload) method will need *ALL* of the parts
    // ETag and number since AWS uses that to assemble the object and to stop charging for parts.
    // The abort_multipart_upload can be called to tell AWS to abort the upload process and remove
    // all of the parts. This is imporant because 'complete' and 'abort' both close the process,
    // remove the chunks and iether stitch up the completed object or abort it so that you're
    // no longer charged for the parts. A bucket policy can also be added to say, abort after
    // X days if an abort or complete is not processed. This will cause AWS to automatically
    // remove the incomplete parts.

    if create_multipart_upload_output.is_some() {
        let create_multipart_upload = create_multipart_upload_output.unwrap();
        let upload_id: &str = &create_multipart_upload.upload_id;
        let mut parts_list: Vec<String> = Vec::new();

        repeat_color!(term::color::WHITE, "-", "part-1", width);

        // Create read buffer for bytes and read in first part.
        //let path = Path::new(file_name);
        // NOTE: Used 2 file objects because it's not in a loop and it's using a seek to show
        // what would happen if done in different threads.
        let f1 = File::open(file_name).unwrap();
        let mut f2 = File::open(file_name).unwrap();
        let metadata = f1.metadata().unwrap();
        let min_size: u64 = 5242880;
        let len: usize = (metadata.len() - min_size) as usize;
        // NOTE: Don't do this in a dynamic envrionment since the metadata.len() is u64 and Vec can't handle that size.
        let mut part1_buffer: Vec<u8> = Vec::with_capacity(min_size as usize); // 5MB
        let mut part2_buffer: Vec<u8> = Vec::with_capacity(if len > min_size as usize {min_size as usize} else {len});

        let mut upload_part = MultipartUploadPartRequest::default();
        upload_part.bucket = bucket_name.to_string();
        upload_part.upload_id = upload_id.to_string();
        upload_part.key = file_name.to_string();

        // read file
        match f1.take(min_size).read_to_end(&mut part1_buffer) {
            Ok(_) => println_color!(term::color::YELLOW, "Read in buffer 1 - {}", part1_buffer.len()),
            Err(e) => println_color!(term::color::RED, "Error reading file {}", e),
        }

        upload_part.body = Some(&part1_buffer);
        upload_part.part_number = 1;
        // Compute hash - Hash is slow
        //let hash = md5::compute(upload_part.body.unwrap()).to_base64(STANDARD);
        //upload_part.content_md5 = Some(hash);

        match client.multipart_upload_part(&upload_part) {
            Ok(output) => {
                // Collecting the partid in a list.
                let new_out = output.clone();
                parts_list.push(output);
                println_color!(term::color::GREEN, "Part 1 - {:#?}", new_out);
            },
            Err(e) => println_color!(term::color::RED, "{:#?}", e),
        }

        // NOTE: Keeping the test simple to begin with. The created file is ~8MB in size so we
        // can break it up into 5MB and 3MB for two parts. Could put in a loop and make it more
        // useful.

        repeat_color!(term::color::WHITE, "-", "list_multipart_uploads", width);

        let mut list_multipart_uploads = MultipartUploadListRequest::default();
        list_multipart_uploads.bucket = bucket_name.to_string();

        match client.multipart_upload_list(&list_multipart_uploads) {
            Ok(output) => println_color!(term::color::GREEN, "{:#?}", output),
            Err(e) => println_color!(term::color::RED, "{:#?}", e),
        }

        repeat_color!(term::color::WHITE, "-", "list_parts (#1)", width);

        let mut list_parts = MultipartUploadListPartsRequest::default();
        list_parts.bucket = bucket_name.to_string();
        list_parts.upload_id = upload_id.to_string();
        list_parts.key = file_name.to_string();

        match client.multipart_upload_list_parts(&list_parts) {
            Ok(output) => println_color!(term::color::GREEN, "{:#?}", output),
            Err(e) => println_color!(term::color::RED, "{:#?}", e),
        }

        // If test_abort is true then don't upload the last part so that we can test abort.
        if !test_abort {
            repeat_color!(term::color::WHITE, "-", "part-2", width);

            let seek_result = f2.seek(SeekFrom::Start(min_size + 1)).unwrap();

            match f2.take(part2_buffer.capacity() as u64).read_to_end(&mut part2_buffer) {
                Ok(_) => println_color!(term::color::YELLOW, "Read in buffer 2"),
                Err(e) => println_color!(term::color::RED, "Error reading file {}", e),
            }

            upload_part.body = Some(&part2_buffer);
            upload_part.part_number = 2;
            // Compute hash - Hash is slow
            //let hash = md5::compute(upload_part.body.unwrap()).to_base64(STANDARD);
            //upload_part.content_md5 = Some(hash);

            match client.multipart_upload_part(&upload_part) {
                Ok(output) => {
                    let new_out = output.clone();
                    parts_list.push(output);
                    println_color!(term::color::GREEN, "Part 2 - {:#?}", new_out);
                },
                Err(e) => println_color!(term::color::RED, "{:#?}", e),
            }

            // Just to show both parts now.
            repeat_color!(term::color::WHITE, "-", "list_parts (#2)", width);

            let mut list_parts = MultipartUploadListPartsRequest::default();
            list_parts.bucket = bucket_name.to_string();
            list_parts.upload_id = upload_id.to_string();
            list_parts.key = file_name.to_string();

            match client.multipart_upload_list_parts(&list_parts) {
                Ok(output) => println_color!(term::color::GREEN, "{:#?}", output),
                Err(e) => println_color!(term::color::RED, "{:#?}", e),
            }
        }

        // If the test_abort is true then abort the process.
        if test_abort {
            repeat_color!(term::color::WHITE, "-", "abort_upload", width);

            let mut abort_multipart_upload = MultipartUploadAbortRequest::default();
            abort_multipart_upload.bucket = bucket_name.to_string();
            abort_multipart_upload.upload_id = upload_id.to_string();
            abort_multipart_upload.key = file_name.to_string();

            match client.multipart_upload_abort(&abort_multipart_upload) {
                Ok(output) => println_color!(term::color::GREEN, "{:#?}", output),
                Err(e) => println_color!(term::color::RED, "{:#?}", e),
            }
        } else {
            // Test complete if not testing abort
            repeat_color!(term::color::WHITE, "-", "complete_multipart_upload", width);

            let item_list : Vec<u8>;

            let mut complete_multipart_upload = MultipartUploadCompleteRequest::default();
            complete_multipart_upload.bucket = bucket_name.to_string();
            complete_multipart_upload.upload_id = upload_id.to_string();
            complete_multipart_upload.key = file_name.to_string();

            // parts_list gets converted to XML and sets the item_list.
            match multipart_upload_finish_xml(&parts_list) {
                Ok(parts_in_xml) => item_list = parts_in_xml,
                Err(e) => {
                    item_list = Vec::new(); // Created the list here so it will fail in the call below
                    println_color!(term::color::RED, "{:#?}", e);
                },
            }

            complete_multipart_upload.multipart_upload = Some(&item_list);

            match client.multipart_upload_complete(&complete_multipart_upload) {
                Ok(output) => println_color!(term::color::GREEN, "{:#?}", output),
                Err(e) => println_color!(term::color::RED, "{:#?}", e),
            }
        }

        // Remove temp file - ignore result
        if file_remove {
            let result = run_cli(format!("rm -f {}", file_name.to_string()));
        }
    }


    repeat_color!(term::color::WHITE, "-", "put_object_acl", width);

    let mut put_object_acl = PutObjectAclRequest::default();
    put_object_acl.bucket = bucket_name.to_string();
    put_object_acl.key = "mytest.txt".to_string();
    put_object_acl.acl = Some(CannedAcl::Private);

    match client.put_object_acl(&put_object_acl) {
        Ok(acl) => println_color!(term::color::GREEN, "{:#?}", acl),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    repeat_color!(term::color::WHITE, "-", "get_object_acl", width);

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


    repeat_color!(term::color::WHITE, "-", "head_bucket", width);

    let bucket_head = HeadBucketRequest { bucket: bucket_name.to_string() };

    match client.head_bucket(&bucket_head) {
        Ok(head) => println_color!(term::color::GREEN, "{:#?}", head),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    repeat_color!(term::color::WHITE, "-", "put_bucket_versioning", width);

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


    repeat_color!(term::color::WHITE, "-", "get_bucket_versioning", width);

    let bucket_versioning = GetBucketVersioningRequest { bucket: bucket_name.to_string() };

    match client.get_bucket_versioning(&bucket_versioning) {
        Ok(version) => println_color!(term::color::GREEN, "{:#?}", version),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    repeat_color!(term::color::WHITE, "-", "delete_object", width);

    let mut del_object = DeleteObjectRequest::default();
    del_object.bucket = bucket_name.to_string();
    del_object.key = "mytest.txt".to_string();

    match client.delete_object(&del_object) {
        Ok(output) => println_color!(term::color::GREEN, "{:#?}", output),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    repeat_color!(term::color::WHITE, "-", "list_object_versions", width);

    let mut bucket_versioning = ListObjectVersionsRequest::default();
    bucket_versioning.bucket = bucket_name.to_string();

    match client.list_object_versions(&bucket_versioning) {
        Ok(version) => println_color!(term::color::GREEN, "{:#?}", version),
        Err(e) => println_color!(term::color::RED, "{:#?}", e),
    }


    repeat_color!(term::color::WHITE, "-", "list_objects (again)", width);

    let mut list_objects = ListObjectsRequest::default();
    list_objects.bucket = bucket_name.to_string();
    // NOTE: The default is version 1 listing. You must set it to version 2.
    list_objects.version = Some(2);

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


    repeat_color!(term::color::WHITE, "-", "delete_bucket", width);

    let bucket = DeleteBucketRequest { bucket: bucket_name.to_string() };

    match client.delete_bucket(&bucket) {
        Ok(bucket) => println_color!(term::color::GREEN, "{:#?}", bucket),
        Err(e) => {
            println_color!(term::color::RED, "{:#?}", e);

            repeat_color!(term::color::WHITE, "-", "list_object_versions", width);

            let mut bucket_versioning = ListObjectVersionsRequest::default();
            bucket_versioning.bucket = bucket_name.to_string();

            match client.list_object_versions(&bucket_versioning) {
                Ok(version) => println_color!(term::color::GREEN, "{:#?}", version),
                Err(e) => println_color!(term::color::RED, "{:#?}", e),
            }
        },
    }

    repeat_color!(term::color::WHITE, "-", "list_buckets", width);

    match client.list_buckets() {
        Ok(output) => {
            println_color!(term::color::GREEN, "{:#?}", output);
        },
        Err(error) => {
            println_color!(term::color::RED, "Error: {:#?}", error);
        },
    }

    repeat_color!(term::color::GREEN, "=", "Finished", width);
}
