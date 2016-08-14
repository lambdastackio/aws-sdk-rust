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

// Sample access code and testing ground for the library.

extern crate aws_sdk_rust;

use aws_sdk_rust::aws::common::credentials::DefaultCredentialsProvider;
use aws_sdk_rust::aws::common::region::Region;
use aws_sdk_rust::aws::s3::s3client::S3Client;

fn main() {
    let provider = DefaultCredentialsProvider::new().unwrap();
    println!("step 1");
    let mut client = S3Client::new(provider, Region::UsEast1);
    println!("step 2");

    match client.list_buckets() {
      Ok(output) => {
        println!("{:?}", output);
      }
      Err(error) => {
        println!("Error: {:?}", error);
      }
    }
}
