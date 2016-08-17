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
extern crate url;
extern crate hyper;

//use std::str::FromStr;

//use url::Url;

//use aws_sdk_rust::aws::common::credentials::{DefaultCredentialsProvider, ParametersProvider};
use aws_sdk_rust::aws::common::credentials::DefaultCredentialsProvider;
use aws_sdk_rust::aws::common::region::Region;
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
    /* Example of using parameters for passing the credentials.
    let param_provider: Option<ParametersProvider>;
    param_provider = Some(
        ParametersProvider::with_params(
            "<whatever your access_key_id>",
            "<whatever your secret_access_key>",
            None).unwrap()
    );

    let provider = DefaultCredentialsProvider::new(param_provider).unwrap();
    */

    // Allow the defaults w/o ParametersProvider - pass in 'None' in ::new(None)
    let provider = DefaultCredentialsProvider::new(None).unwrap();

    // V4 is the default signature for AWS. However, other systems also use V2.
    let client = S3Client::new(provider, Region::UsEast1, "V4", None);

    // If you wish to override the defaults of AWS then you can call the method below before
    // making any requests.
    // client.set_endpoint("<whatever url you want>");
    println!("Endpoint: {}", client.endpoint());

    match client.list_buckets() {
      Ok(output) => {
        println!("{:?}", output);
      }
      Err(error) => {
        println!("Error: {:?}", error);
      }
    }
}
