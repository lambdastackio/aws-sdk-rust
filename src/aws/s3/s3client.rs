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

#![allow(unused_variables, unused_mut)]
use std::fmt;
use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::error::Error;
use std::io::BufReader;
use std::io::Read;
use std::num::ParseIntError;
use std::str::{FromStr, ParseBoolError};
use std::str;

use hyper::client::{Client, RedirectPolicy};
use openssl::crypto::hash::Type::MD5;
use openssl::crypto::hash::hash;
use rustc_serialize::base64::{ToBase64, STANDARD};
use xml::*;

use aws::common::credentials::{AwsCredentialsProvider, AwsCredentials};
use aws::common::region::Region;
use aws::common::xmlutil::*;
use aws::common::params::{Params, ServiceParams};
use aws::common::signature::SignedRequest;
use aws::common::request::{DispatchSignedRequest, HttpResponse};
use aws::errors::s3_error::S3Error;
use aws::errors::credentials_error::CredentialsError;
use aws::s3::writeparse::*;
use aws::s3::bucket::*;

// This creates the default endpoint to be used on initial create.
fn default_endpoint(region: Region) -> String {
    let endpoint: String = match region {
        Region::UsEast1 => "s3.amazonaws.com".to_string(),
        Region::CnNorth1 => format!("s3.{}.amazonaws.com.cn", region),
        _ => format!("s3.amazonaws.com"),
    };

    endpoint
}


/// S3Client - Base client all
/// Virtual Hosting S3 docs - http://docs.aws.amazon.com/AmazonS3/latest/dev/VirtualHosting.html
///
#[derive(Debug)]
pub struct S3Client<P, D> where P: AwsCredentialsProvider, D: DispatchSignedRequest {
            credentials_provider: P,
            region: Region,
            dispatcher: D,
            endpoint: String,
            version: String
        }

impl<P> S3Client<P, Client> where P: AwsCredentialsProvider {
    pub fn new<S>(credentials_provider: P, region: Region, version: S) -> Self where S:Into<String> {
        let mut client = Client::new();
        client.set_redirect_policy(RedirectPolicy::FollowNone);
        S3Client::with_request_dispatcher(client, credentials_provider, region, version)
    }
}

// NOTE: dispatcher is the hyper client dispatcher that makes the HTTP(s) requests
impl<P, D> S3Client<P, D> where P: AwsCredentialsProvider, D: DispatchSignedRequest {
    pub fn with_request_dispatcher<S>(request_dispatcher: D, credentials_provider: P, region: Region, version: S)
        -> Self where S:Into<String> {
        S3Client {
            credentials_provider: credentials_provider,
            region: region,
            endpoint: default_endpoint(region),
            version: version.into(),
            dispatcher: request_dispatcher
        }
    }

    /// set_endpoint - Sets the correct endpoint.
    ///
    /// The default value of the endpoint is created during the 'new' method. This sets it to
    /// s3.amazon.com as the default.
    pub fn set_endpoint<S>(&mut self, endpoint: S) where S:Into<String> {
        self.endpoint = endpoint.into().to_owned();
    }

    /// Gets the endpoint value
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Creates a new bucket.
    /// All requests go to the us-east-1/us-standard endpoint, but can create buckets anywhere.
    pub fn create_bucket(&self, input: &CreateBucketRequest) -> Result<CreateBucketOutput, S3Error> {
        let region = Region::UsEast1;
        let mut create_config : Vec<u8>;
        let mut request = SignedRequest::new("PUT", "s3", region, "", &self.version);
        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        if needs_create_bucket_config(self.region) {
            create_config = create_bucket_config_xml(self.region);
            request.set_payload(Some(&create_config));
        }

        match input.acl {
            None => {},
            Some(ref canned_acl) => request.add_header("x-amz-acl", &canned_acl_in_aws_format(canned_acl)),
        }

        let result = sign_and_execute(&self.dispatcher, &mut request, try!(self.credentials_provider.credentials()));
        let status = result.status;

        match status {
            200 => {
                match result.headers.get("Location") {
                    Some(ref value) => Ok(CreateBucketOutput{ location: value.to_string() }),
                    None => Err(S3Error::new("Something went wrong when creating a bucket."))
                }
            }
            _ => {
                Err(S3Error::new("error in create_bucket"))
            }
        }
    }

    /// Returns a list of all buckets owned by the authenticated sender of the
    /// request.
    pub fn list_buckets(&self) -> Result<ListBucketsOutput, S3Error> {
        let mut request = SignedRequest::new("GET", "s3", self.region, "/", &self.version);
        request.set_hostname(Some(self.endpoint.to_owned()));

        let mut params = Params::new();
        params.put("Action", "ListBuckets");
        request.set_params(params);
        let result = sign_and_execute(&self.dispatcher, &mut request, try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());

        stack.next(); // xml start tag

        match status {
            200 => {
                // was "ListBucketsOutput"
                Ok(try!(ListBucketsOutputParser::parse_xml("ListAllMyBucketsResult", &mut stack)))
            }
            _ => { Err(S3Error::new("error in list_buckets")) }
        }
    }

    fn hostname(&self, bucket: Option<&BucketName>) -> String {
        match bucket {
            Some(b) => format!("{}.{}", b, self.endpoint),
            None => format!("{}", self.endpoint),
        }
    }
}

/// Maps canned acl to AWS format.  EG public-read.
pub fn canned_acl_in_aws_format(canned_acl: &CannedAcl) -> String {
    match *canned_acl {
        CannedAcl::Private => "private".to_string(),
        CannedAcl::PublicRead => "public-read".to_string(),
        CannedAcl::PublicReadWrite => "public-read-write".to_string(),
        CannedAcl::AuthenticatedRead => "authenticated-read".to_string(),
        CannedAcl::BucketOwnerRead => "bucket-owner-read".to_string(),
        CannedAcl::BucketOwnerFullControl => "bucket-owner-full-control".to_string(),
    }
}

/// `extract_s3_redirect_location` takes a Hyper `Response` and attempts to pull out the temporary endpoint.
fn extract_s3_redirect_location(response: HttpResponse) -> Result<String, S3Error> {
    let mut reader = EventReader::from_str(&response.body);
    let mut stack = XmlResponse::new(reader.events().peekable());
    stack.next(); // xml start tag

    // extract and return temporary endpoint location
    extract_s3_temporary_endpoint_from_xml(&mut stack)
}

fn field_in_s3_redirect(name: &str) -> bool {
    if name == "Code" || name == "Message" || name == "Bucket" || name == "RequestId" || name == "HostId" {
        return true;
    }
    false
}


/// `extract_s3_temporary_endpoint_from_xml` takes in XML and tries to find the value of the Endpoint node.
fn extract_s3_temporary_endpoint_from_xml<T: Peek + Next>(stack: &mut T) -> Result<String, S3Error> {
    try!(start_element(&"Error".to_string(), stack));

    // now find Endpoint contents
    // This may infinite loop if there's no endpoint in the response: how can we prevent that?
    loop {
        let current_name = try!(peek_at_name(stack));
        if current_name == "Endpoint" {
            let obj = try!(string_field("Endpoint", stack));
            return Ok(obj);
        }
        if field_in_s3_redirect(&current_name){
            // <foo>bar</foo>:
            stack.next(); // skip the start tag <foo>
            stack.next(); // skip contents bar
            stack.next(); // skip close tag </foo>
            continue;
        }
        break;
    }
    Err(S3Error::new("Couldn't find redirect location for S3 bucket"))
}

// Internal method that calls the hyper dispatcher to send the URL request.
fn sign_and_execute<D>(dispatcher: &D, signed_request: &mut SignedRequest, creds: AwsCredentials) -> HttpResponse where D: DispatchSignedRequest{

    signed_request.sign(&creds);

    let response = dispatcher.dispatch(signed_request).expect("Error dispatching request");

    if response.status == 307 {
        debug!("Got a redirect response, resending request.");
        // extract location from response, modify request and re-sign and resend.
        let new_hostname = extract_s3_redirect_location(response).unwrap();
        signed_request.set_hostname(Some(new_hostname.to_string()));

        // This does a lot of appending and not clearing/creation, so we'll have to do that ourselves:
        signed_request.sign(&creds);
        return dispatcher.dispatch(signed_request).unwrap();
    }

    response
}
