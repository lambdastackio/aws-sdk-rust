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

// Portions borrowed from the rusoto project. See README.md

#![allow(unused_variables, unused_mut)]
use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::str::FromStr;
use std::str;
use std::env;
use std::time::Instant; //, SystemTime};

use hyper::client::{Client, RedirectPolicy};
use url::Url;
use xml::reader::EventReader;
use chrono::{self, UTC};

use aws::common::credentials::{AwsCredentials, AwsCredentialsProvider};
use aws::common::region::Region;
use aws::common::xmlutil::*;
use aws::common::params::{Params, ServiceParams};
use aws::common::signature::SignedRequest;
use aws::common::request::{DispatchSignedRequest, HttpResponse};
use aws::common::common::Operation;
use aws::errors::s3::*;
use aws::errors::aws::*;
use aws::s3::endpoint::*;
use aws::s3::writeparse::*;
use aws::s3::bucket::*;
use aws::s3::object::*;
use aws::s3::acl::*;

/// Returns a valid hyper client. If proxies are passed in then a proxy version of the client is returned.
/// If None is passed then in then the default Client is returned.
///
pub fn http_client(proxy: Option<Url>, endpoint: Url) -> Client {
    let mut proxy_url: String = String::new();
    let mut proxy_port: u16 = 0;
    let endpoint_domain = endpoint.host_str().unwrap();

    let is_proxy = match proxy {
        Some(url) => {
            proxy_url = url.host_str().unwrap_or("").to_string();
            proxy_port = url.port_or_known_default().unwrap();
            true
        },
        None => {
            // Check envrionment var http_proxy
            match env::var("http_proxy") {
                Ok(url) => {
                    let url = Url::parse(&url).unwrap();
                    proxy_url = url.host_str().unwrap_or("").to_string();
                    proxy_port = url.port_or_known_default().unwrap();
                    true
                },
                _ => {
                    match env::var("HTTP_PROXY") {
                        Ok(url) => {
                            let url = Url::parse(&url).unwrap();
                            proxy_url = url.host_str().unwrap_or("").to_string();
                            proxy_port = url.port_or_known_default().unwrap();
                            true
                        },
                        _ => false,
                    }
                },
            }

        },
    };

    match is_proxy {
        true => {
            let mut is_proxy = true;

            match env::var("no_proxy") {
                Ok(domain_list) => {
                    if domain_list.contains(endpoint_domain) {
                        is_proxy = false;
                    }
                },
                _ => {},
            }

            match is_proxy {
                true => Client::with_http_proxy(proxy_url, proxy_port),
                _ => Client::new(),
            }
        },
        _ => Client::new(),
    }
}

/// S3Client - Base client all
/// Virtual Hosting S3 docs - http://docs.aws.amazon.com/AmazonS3/latest/dev/VirtualHosting.html
///
#[derive(Debug)]
pub struct S3Client<P, D>
    where P: AwsCredentialsProvider,
          D: DispatchSignedRequest,
{
    credentials_provider: P,
    dispatcher: D,
    region: Region,
    endpoint: Endpoint,
}

impl<P> S3Client<P, Client>
    where P: AwsCredentialsProvider,
{
    /// Entry point for S3Client. Must provide a Provider to the S3Client. Example:
    ///
    /// ```
    /// let provider = DefaultCredentialsProvider::new(None).unwrap();
    /// ```
    ///
    pub fn new(credentials_provider: P, endpoint: Endpoint) -> Self {
        // Hyper client
        let mut client = http_client(endpoint.proxy.clone(), endpoint.endpoint.clone().unwrap());

        client.set_redirect_policy(RedirectPolicy::FollowNone);
        S3Client::with_request_dispatcher(client, credentials_provider, endpoint)
    }
}

// NOTE: dispatcher is the hyper client dispatcher that makes the HTTP(s) requests
impl<P, D> S3Client<P, D>
    where P: AwsCredentialsProvider,
          D: DispatchSignedRequest,
{
    /// Creator of the S3Client object.
    pub fn with_request_dispatcher(request_dispatcher: D, credentials_provider: P, endpoint: Endpoint) -> Self {
        S3Client {
            credentials_provider: credentials_provider,
            region: endpoint.region.clone(),
            endpoint: endpoint,
            dispatcher: request_dispatcher,
        }
    }

    /// Returns the current Endpoint of the S3Client.
    pub fn endpoint(&self) -> &Endpoint {
        &self.endpoint
    }

    /// Creates a new bucket.
    /// All requests go to the us-east-1/us-standard endpoint, but can create buckets anywhere.
    pub fn create_bucket(&self, input: &CreateBucketRequest) -> Result<CreateBucketOutput, S3Error> {
        // let region = Region::UsEast1;
        let mut create_config: Vec<u8>;
        let mut request = SignedRequest::new("PUT",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             "/",
                                             &self.endpoint);
                                             //&&self.endpoint);

        // If location is not 'us-east-1' create bucket location config.
        if needs_create_bucket_config(self.region) {
            create_config = create_bucket_config_xml(self.region);
            request.set_payload(Some(&create_config));
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        match input.acl {
            None => {},
            Some(ref canned_acl) => request.add_header("x-amz-acl", &canned_acl_in_aws_format(canned_acl)),
        }

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;

        match status {
            200 => {
                match result.headers.get("Location") {
                    Some(ref value) => Ok(CreateBucketOutput { location: value.to_string() }),
                    None => Err(S3Error::new("Something went wrong when creating a bucket.")),
                }
            },
            _ => {
                let mut reader = EventReader::from_str(&result.body);
                let mut stack = XmlResponse::new(reader.events().peekable());
                stack.next(); // xml start tag

                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error creating bucket", aws))
            },
        }
    }

    /// head_bucket is good for seeing if a bucket exists and you have permission to access it.
    /// AWS will return 200 if it found it and you have permission. It will return 404 or 403
    /// if the bucket is not found or you don't have permission.
    ///
    /// head_bucket returns Ok(()) if found and you have permission else error.
    pub fn head_bucket(&self, input: &HeadBucketRequest) -> Result<(), S3Error> {
        let mut request = SignedRequest::new("HEAD",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             "/",
                                             &self.endpoint);

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        //let mut reader = EventReader::from_str(&result.body);
        //let mut stack = XmlResponse::new(reader.events().peekable());
        //stack.next(); // xml start tag
        //stack.next();

        match status {
            200 => {
                Ok(())
            },
            _ => {
                Err(S3Error::new("Error bucket does not exists or error in retrieving"))
            },
        }
    }

    /// Returns a list of all buckets owned by the authenticated sender of the
    /// request.
    pub fn list_buckets(&self) -> Result<ListBucketsOutput, S3Error> {
        let mut request = SignedRequest::new(
                                        "GET",
                                        "s3",
                                        self.region,
                                        "",
                                        "/",
                                        &self.endpoint);

        request.set_hostname(self.endpoint.hostname());

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());

        stack.next(); // xml start tag

        match status {
            200 => {
                Ok(try!(ListBucketsOutputParser::parse_xml("ListAllMyBucketsResult", &mut stack)))
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error listing buckets", aws))
            },
        }
    }

    /// Sets lifecycle configuration for your bucket. If a lifecycle configuration
    /// exists, it replaces it.
    pub fn put_bucket_lifecycle(&self, input: &PutBucketLifecycleRequest) -> Result<(), S3Error> {
        let mut request = SignedRequest::new("PUT",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?lifecycle"} else {"/"},
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("lifecycle", "");
            //params.put("Action", "PutBucketLifecycle");
            //PutBucketLifecycleRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();

        match status {
            200 => {
                Ok(())
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error putting bucket lifecycle", aws))
            },
        }
    }

    /// Sets the bucket ACLs
    pub fn put_bucket_acl(&self, input: &PutBucketAclRequest) -> Result<(), S3Error> {
        let mut request = SignedRequest::new("PUT",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?acl"} else {"/"},
                                             &self.endpoint);

        // Not doing anything but allow unused_variables is set above to kill warning.
        let acls = build_bucket_acls(&mut request, &input);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("acl", "");
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag

        match status {
            200 => {
                Ok(())
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error putting bucket acl", aws))
            },
        }
    }

    /// Replaces a policy on a bucket. If the bucket already has a policy, the one in
    /// this request completely replaces it.
    pub fn put_bucket_policy(&self, input: &PutBucketPolicyRequest) -> Result<(), S3Error> {
        let mut request = SignedRequest::new("PUT",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?policy"} else {"/"},
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("policy", "");
            // params.put("Action", "PutBucketPolicy");
            // PutBucketPolicyRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag

        match status {
            200 => {
                Ok(())
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error putting bucket policy", aws))
            },
        }
    }

    /// Set the website configuration for a bucket.
    pub fn put_bucket_website(&self, input: &PutBucketWebsiteRequest) -> Result<(), S3Error> {
        let mut request = SignedRequest::new("PUT",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?website"} else {"/"},
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("website", "");
            // params.put("Action", "PutBucketWebsite");
            // PutBucketWebsiteRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag

        match status {
            200 => {
                Ok(())
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error putting bucket website", aws))
            },
        }
    }

    /// Set the logging parameters for a bucket and to specify permissions for who can
    /// view and modify the logging parameters. To set the logging status of a bucket,
    /// you must be the bucket owner.
    pub fn put_bucket_logging(&self, input: &PutBucketLoggingRequest) -> Result<(), S3Error> {
        let mut request = SignedRequest::new("PUT",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?logging"} else {"/"},
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("logging", "");
            // params.put("Action", "PutBucketLogging");
            // PutBucketLoggingRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();

        match status {
            200 => {
                Ok(())
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error putting bucket logging", aws))
            },
        }
    }

    /// Creates a new replication configuration (or replaces an existing one, if
    /// present).
    pub fn put_bucket_replication(&self, input: &PutBucketReplicationRequest) -> Result<(), S3Error> {
        let mut request = SignedRequest::new("PUT",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?replication"} else {""},
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("replication", "");
            // params.put("Action", "PutBucketReplication");
            // PutBucketReplicationRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();

        match status {
            200 => {
                Ok(())
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error putting bucket replication", aws))
            },
        }
    }

    /// Sets the versioning state of an existing bucket. To set the versioning state,
    /// you must be the bucket owner.
    pub fn put_bucket_versioning(&self, input: &PutBucketVersioningRequest) -> Result<(), S3Error> {
        let mut payload: Vec<u8>;
        let mut request = SignedRequest::new("PUT",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?versioning"} else {"/"},
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("versioning", "");
            // params.put("Action", "PutBucketVersioning");
            // PutBucketVersioningRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let xml = format!("<VersioningConfiguration xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">
                                \
                           <Status>{}</Status>
                            </VersioningConfiguration>",
                          input.versioning_configuration.status);
        payload = xml.into_bytes();
        request.set_payload(Some(&payload));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();

        match status {
            200 => {
                Ok(())
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error putting bucket versioning", aws))
            },
        }
    }

    /// Deletes the bucket. All objects (including all object versions and Delete
    /// Markers) in the bucket must be deleted before the bucket itself can be
    /// deleted.
    pub fn delete_bucket(&self, input: &DeleteBucketRequest) -> Result<(), S3Error> {
        let mut request = SignedRequest::new("DELETE",
                                             "s3",
                                             self.endpoint.region,
                                             &input.bucket,
                                             "/",
                                             &self.endpoint);

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let mut result = sign_and_execute(&self.dispatcher,
                                          &mut request,
                                          try!(self.credentials_provider.credentials()));
        let status = result.status;

        match status {
            204 => {
                Ok(())
            },
            _ => {
                let mut reader = EventReader::from_str(&result.body);
                let mut stack = XmlResponse::new(reader.events().peekable());
                stack.next(); // xml start tag

                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error deleting bucket", aws))
            },
        }
    }

    /// Deletes the tags from the bucket.
    pub fn delete_bucket_tagging(&self, input: &DeleteBucketTaggingRequest) -> Result<(), S3Error> {
        let mut request = SignedRequest::new("DELETE",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?tagging"} else {""},
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("tagging", "");
            // params.put("Action", "DeleteBucketTagging");
            // DeleteBucketTaggingRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();

        match status {
            200 => {
                Ok(())
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error deleting bucket tagging", aws))
            },
        }
    }

    /// Deletes the cors configuration information set for the bucket.
    pub fn delete_bucket_cors(&self, input: &DeleteBucketCorsRequest) -> Result<(), S3Error> {
        let mut request = SignedRequest::new("DELETE",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?cors"} else {"/"},
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("cors", "");
            // params.put("Action", "DeleteBucketCors");
            // DeleteBucketCorsRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();

        match status {
            200 => {
                Ok(())
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error deleting bucket cors", aws))
            },
        }
    }

    /// This operation removes the website configuration from the bucket.
    pub fn delete_bucket_website(&self, input: &DeleteBucketWebsiteRequest) -> Result<(), S3Error> {
        let mut request = SignedRequest::new("DELETE",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?website"} else {"/"},
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("website", "");
            // params.put("Action", "DeleteBucketWebsite");
            // DeleteBucketWebsiteRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();

        match status {
            200 => {
                Ok(())
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error deleting bucket website", aws))
            },
        }
    }

    /// Deletes the policy from the bucket.
    pub fn delete_bucket_policy(&self, input: &DeleteBucketPolicyRequest) -> Result<(), S3Error> {
        let mut request = SignedRequest::new("DELETE",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?policy"} else {"/"},
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("policy", "");
            // params.put("Action", "DeleteBucketPolicy");
            // DeleteBucketPolicyRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();

        match status {
            200 => {
                Ok(())
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error deleting bucket policy", aws))
            },
        }
    }

    /// Deletes bucket replication.
    pub fn delete_bucket_replication(&self, input: &DeleteBucketReplicationRequest) -> Result<(), S3Error> {
        let mut request = SignedRequest::new("DELETE",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?replication"} else {"/"},
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("replication", "");
            // params.put("Action", "DeleteBucketReplication");
            // DeleteBucketReplicationRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();

        match status {
            200 => {
                Ok(())
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error deleting bucket replication", aws))
            },
        }
    }

    /// Returns the cors configuration for the bucket.
    pub fn get_bucket_cors(&self, input: &GetBucketCorsRequest) -> Result<GetBucketCorsOutput, S3Error> {
        let mut request = SignedRequest::new("GET",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?cors"} else {"/"},
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("cors", "");
            // params.put("Action", "GetBucketCors");
            // GetBucketCorsRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();

        match status {
            200 => {
                Ok(try!(GetBucketCorsOutputParser::parse_xml("GetBucketCorsOutput", &mut stack)))
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error getting bucket cors", aws))
            },
        }
    }

    /// Gets the access control policy for the bucket.
    pub fn get_bucket_acl(&self, input: &GetBucketAclRequest) -> Result<AccessControlPolicy, S3Error> {
        let mut request = SignedRequest::new("GET",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?acl"} else {"/"},
                                             &self.endpoint);

        // NOTE: V4 - For sub-resources add them as params and not part of the path
        // NOTE: V2 - For sub-resources add then as part of the path and not as params

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("acl", "");
            //params.put("Action", "GetBucketAcl");
            //GetBucketAclRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag

        match status {
            200 => {
                Ok(try!(AccessControlPolicyParser::parse_xml("AccessControlPolicy", &mut stack)))
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error getting bucket acl", aws))
            },
        }
    }

    /// Returns the logging status of a bucket and the permissions users have to view
    /// and modify that status. To use GET, you must be the bucket owner.
    pub fn get_bucket_logging(&self, input: &GetBucketLoggingRequest)
                -> Result<GetBucketLoggingOutput, S3Error> {
        let mut request = SignedRequest::new("GET",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?logging"} else {"/"},
                                             &self.endpoint);

        // NOTE: V4 - For sub-resources add them as params and not part of the path
        // NOTE: V2 - For sub-resources add then as part of the path and not as params

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("logging", "");
            // params.put("Action", "GetBucketLogging");
            // GetBucketLoggingRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag

        match status {
            200 => {
                Ok(try!(GetBucketLoggingOutputParser::parse_xml("BucketLoggingStatus", &mut stack)))
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error getting bucket logging", aws))
            },
        }
    }

    /// Returns the notification configuration of a bucket.
    pub fn get_bucket_notification_configuration(&self,
                                                 input: &GetBucketNotificationConfigurationRequest)
                                                 -> Result<NotificationConfiguration, S3Error> {
        let mut request = SignedRequest::new(
                        "GET",
                        "s3",
                        self.region,
                        &input.bucket,
                        if self.endpoint.signature == Signature::V2 {"/?notification"} else {"/"},
                        &self.endpoint);

         // NOTE: V4 - For sub-resources add them as params and not part of the path
         // NOTE: V2 - For sub-resources add then as part of the path and not as params

         if self.endpoint.signature == Signature::V4 {
             let mut params = Params::new();
             params.put("notification", "");
             // params.put("Action", "GetBucketNotificationConfiguration");
             // GetBucketNotificationConfigurationRequestWriter::write_params(&mut params, "", input);
             request.set_params(params);
         }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag

        match status {
            200 => {
                Ok(try!(NotificationConfigurationParser::parse_xml("NotificationConfiguration", &mut stack)))
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error getting bucket notification", aws))
            },
        }
    }

    /// Returns the bucket versioning output object if versioning is enabled.
    pub fn get_bucket_versioning(&self,
                                 input: &GetBucketVersioningRequest)
                                 -> Result<GetBucketVersioningOutput, S3Error> {
        let mut request = SignedRequest::new("GET",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?versioning"} else {"/"},
                                             &self.endpoint);

         // NOTE: V4 - For sub-resources add them as params and not part of the path
         // NOTE: V2 - For sub-resources add then as part of the path and not as params

         if self.endpoint.signature == Signature::V4 {
             let mut params = Params::new();
             params.put("versioning", "");
             // params.put("Action", "GetBucketVersioning");
             // GetBucketVersioningRequestWriter::write_params(&mut params, "", input);
             request.set_params(params);
         }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());

        stack.next(); // xml start tag
        match status {
            200 => {
                // AWS Returns VersioningConfiguration XML so parse and create
                // GetBucketVersioningOutput
                Ok(try!(GetBucketVersioningOutputParser::parse_xml("VersioningConfiguration", &mut stack)))
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error getting bucket versioning", aws))
            },
        }
    }

    // Object Section

    /// Returns some or all (up to 1000) of the objects in a bucket. You can use the
    /// request parameters as selection criteria to return a subset of the objects in
    /// a bucket.
    pub fn list_objects(&self, input:
                        &ListObjectsRequest)
                        -> Result<ListObjectsOutput, S3Error> {
        let mut request = SignedRequest::new("GET",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             //&format,
                                             "/",
                                             &self.endpoint);

        if input.version != None {
            let mut params = Params::new();
            params.put("list-type", "2");
            // params.put("Action", "ListObjects");
            // ListObjectsRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag
        match status {
            200 => {
                Ok(try!(ListObjectsOutputParser::parse_xml("ListBucketResult", &mut stack)))
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error listing bucket objects", aws))
            },
        }
    }

    /// The HEAD operation retrieves metadata from an object without returning the
    /// object itself. This operation is useful if you're only interested in an
    /// object's metadata. To use HEAD, you must have READ access to the object.
    pub fn head_object(&self, input: &HeadObjectRequest) -> Result<HeadObjectOutput, S3Error> {
        let mut request = SignedRequest::new("HEAD",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             &format!("/{}", input.key),
                                             &self.endpoint);

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        // let mut params = Params::new();
        // params.put("Action", "HeadObject");
        // GetObjectRequestWriter::write_params(&mut params, "", input);
        // request.set_params(params);

        let mut result = sign_and_execute(&self.dispatcher,
                                          &mut request,
                                          try!(self.credentials_provider.credentials()));
        let status = result.status;

        match status {
            200 => {
                let head_object = try!(S3Client::<P,D>::head_object_from_response(&mut result));

                Ok(head_object)
            }
            _ => {
              let format = format!("Error getting object head with response: {} - {}", status, if status == 404 {"not found"} else {""});
              Err(S3Error::new(format))
            }
        }
    }

    /// Retrieves objects from Amazon S3.
    ///
    /// Keep in mind way to increase performance:
    /// http://docs.aws.amazon.com/AmazonS3/latest/dev/request-rate-perf-considerations.html
    ///
    /// AWS S3 recommends any GET operations that exceed 300 per second should open
    /// a support ticket to increase the rate. See the link above more details.
    ///
    pub fn get_object(&self,
                      input: &GetObjectRequest,
                      operation: Option<&mut Operation>)
                      -> Result<GetObjectOutput, S3Error> {
        let mut request = SignedRequest::new("GET",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             &format!("/{}", input.key),
                                             &self.endpoint);

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        // let mut params = Params::new();
        // params.put("Action", "GetObject");
        // GetObjectRequestWriter::write_params(&mut params, "", input);
        // request.set_params(params);
        if let Some(ref range) = input.range {
            request.add_header("Range", range);
        }

        let mut result = new_sign_and_execute(&self.dispatcher,
                                          &mut request,
                                          operation,
                                          try!(self.credentials_provider.credentials()));

        let status = result.status;

        match status {
            200...206 => {
                let s3_object = try!(S3Client::<P, D>::get_object_from_response(&mut result));

                Ok(s3_object)
            },
            _ => {
                let mut reader = EventReader::from_str(&result.body);
                let mut stack = XmlResponse::new(reader.events().peekable());
                stack.next(); // xml start tag

                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error getting object", aws))
            },
        }
    }

    /// Returns the access control list (ACL) of an object.
    pub fn get_object_acl(&self, input: &GetObjectAclRequest) -> Result<AccessControlPolicy, S3Error> {
        let mut path: String;
        if self.endpoint.signature == Signature::V2 {
            path = format!("/{}?acl", input.key);
        } else {
            path = format!("/{}", input.key);
        }

        let mut request = SignedRequest::new("GET",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             &path,
                                             &self.endpoint);

         let hostname = self.hostname(Some(&input.bucket));
         request.set_hostname(Some(hostname));

         // NOTE: V4 - For sub-resources add them as params and not part of the path
         // NOTE: V2 - For sub-resources add then as part of the path and not as params

         if self.endpoint.signature == Signature::V4 {
             let mut params = Params::new();
             params.put("acl", "");
             //params.put("Action", "GetObjectAcl");
             //GetObjectAclRequestWriter::write_params(&mut params, "", input);
             request.set_params(params);
         }

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag
        match status {
            200 => {
                Ok(try!(AccessControlPolicyParser::parse_xml("AccessControlPolicy", &mut stack)))
            }
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error getting object acl", aws))
            }
        }
    }

    /// Returns a value for a requested header.
    pub fn get_value_for_header(header_name: String, response: &HttpResponse) -> Result<String, S3Error> {
        match response.headers.get(&header_name) {
            Some(ref value) => Ok(value.to_string()),
            _ => Ok(String::new()),
        }
    }

    pub fn head_object_from_response(response: &mut HttpResponse) -> Result<HeadObjectOutput, S3Error> {
        // get all the goodies for HeadObjectOutput
        let delete_marker_string = try!(S3Client::<P,D>::get_value_for_header("x-amz-delete-marker".to_string(), &response));
        let delete_marker : bool;
        if delete_marker_string.is_empty() {
            delete_marker = false;
        } else {
            delete_marker = try!(bool::from_str(&delete_marker_string));
        }
        let accept_ranges = try!(S3Client::<P,D>::get_value_for_header("accept-ranges".to_string(), response));
        let last_modified = try!(S3Client::<P,D>::get_value_for_header("Last-Modified".to_string(), response));
        let request_charged = try!(S3Client::<P,D>::get_value_for_header("x-amz-request-charged".to_string(), response));
        let content_encoding = try!(S3Client::<P,D>::get_value_for_header("Content-Encoding".to_string(), response));
        let replication_status = try!(S3Client::<P,D>::get_value_for_header("x-amz-replication-status".to_string(), response));
        let storage_class = try!(S3Client::<P,D>::get_value_for_header("x-amz-storage-class".to_string(), response));
        let server_side_encryption = try!(S3Client::<P,D>::get_value_for_header("x-amz-server-side-encryption".to_string(), response));
        let ssekms_key_id = try!(S3Client::<P,D>::get_value_for_header("x-amz-server-side-encryption-aws-kms-key-id".to_string(), response));
        let content_disposition = try!(S3Client::<P,D>::get_value_for_header("Content-Disposition".to_string(), response));
        let metadata = try!(S3Client::<P,D>::get_value_for_header("x-amz-meta-".to_string(), response));
        let website_redirect_location = try!(S3Client::<P,D>::get_value_for_header("x-amz-website-redirect-location".to_string(), response));
        let expires = try!(S3Client::<P,D>::get_value_for_header("Expires".to_string(), response));
        let cache_control = try!(S3Client::<P,D>::get_value_for_header("Cache-Control".to_string(), response));

        let content_length_string = try!(S3Client::<P,D>::get_value_for_header("Content-Length".to_string(), response));
        let content_length = try!(content_length_string.parse::<i32>());

        let expiration = try!(S3Client::<P,D>::get_value_for_header("x-amz-expiration".to_string(), response));
        let missing_meta_string = try!(S3Client::<P,D>::get_value_for_header("x-amz-missing-meta".to_string(), response));
        let missing_meta : i32;
        if missing_meta_string.is_empty() {
            missing_meta = 0;
        } else {
            missing_meta = try!(missing_meta_string.parse::<i32>());
        }
        let restore = try!(S3Client::<P,D>::get_value_for_header("x-amz-restore".to_string(), response));
        let sse_customer_algorithm = try!(S3Client::<P,D>::get_value_for_header("x-amz-server-side-encryption-customer-algorithm".to_string(), response));
        let content_type = try!(S3Client::<P,D>::get_value_for_header("Content-Type".to_string(), response));
        let content_language = try!(S3Client::<P,D>::get_value_for_header("Content-Language".to_string(), response));
        let version_id = try!(S3Client::<P,D>::get_value_for_header("x-amz-version-id".to_string(), response));
        let e_tag = try!(S3Client::<P,D>::get_value_for_header("ETag".to_string(), response));
        let sse_customer_key_md5 = try!(S3Client::<P,D>::get_value_for_header("x-amz-server-side-encryption-customer-key-MD5".to_string(), response));
        // make the object to return
        let head_object = HeadObjectOutput {
            delete_marker: delete_marker,
            accept_ranges: accept_ranges,
            last_modified: last_modified,
            request_charged: request_charged,
            content_encoding: content_encoding,
            replication_status: replication_status,
            storage_class: storage_class,
            server_side_encryption: server_side_encryption,
            ssekms_key_id: ssekms_key_id,
            content_disposition: content_disposition,
            metadata: HashMap::new(),
            website_redirect_location: website_redirect_location,
            expires: expires,
            cache_control: cache_control,
            content_length: content_length,
            expiration: expiration,
            missing_meta: missing_meta,
            restore: restore,
            sse_customer_algorithm: sse_customer_algorithm,
            content_type: content_type,
            content_language: content_language,
            version_id: version_id,
            e_tag: e_tag,
            sse_customer_key_md5: sse_customer_key_md5,
        };
        Ok(head_object)
    }

    /// Use the Hyper response to populate the GetObjectOutput
    // This may be a great candidate for some codegen magicks.
    pub fn get_object_from_response(response: &mut HttpResponse) -> Result<GetObjectOutput, S3Error> {
        // get all the goodies for GetObjectOutput
        let delete_marker_string = try!(S3Client::<P, D>::get_value_for_header("x-amz-delete-marker".to_string(),
                                                                               &response));
        let delete_marker: bool;
        if delete_marker_string.is_empty() {
            delete_marker = false;
        } else {
            delete_marker = try!(bool::from_str(&delete_marker_string));
        }
        let accept_ranges = try!(S3Client::<P, D>::get_value_for_header("accept-ranges".to_string(), response));
        let last_modified = try!(S3Client::<P, D>::get_value_for_header("Last-Modified".to_string(), response));
        let content_range = try!(S3Client::<P, D>::get_value_for_header("Content-Range".to_string(), response));
        let request_charged = try!(S3Client::<P, D>::get_value_for_header("x-amz-request-charged".to_string(),
                                                                          response));
        let content_encoding = try!(S3Client::<P, D>::get_value_for_header("Content-Encoding".to_string(), response));
        let replication_status = try!(S3Client::<P, D>::get_value_for_header("x-amz-replication-status".to_string(),
                                                                             response));
        let storage_class = try!(S3Client::<P, D>::get_value_for_header("x-amz-storage-class".to_string(), response));
        let server_side_encryption =
            try!(S3Client::<P, D>::get_value_for_header("x-amz-server-side-encryption".to_string(), response));
        let ssekms_key_id = try!(S3Client::<P, D>::get_value_for_header("x-amz-server-side-encryption-aws-kms-key-id"
                                                                            .to_string(),
                                                                        response));
        let content_disposition = try!(S3Client::<P, D>::get_value_for_header("Content-Disposition".to_string(),
                                                                              response));
        let metadata = try!(S3Client::<P, D>::get_value_for_header("x-amz-meta-".to_string(), response));
        let website_redirect_location =
            try!(S3Client::<P, D>::get_value_for_header("x-amz-website-redirect-location".to_string(), response));
        let expires = try!(S3Client::<P, D>::get_value_for_header("Expires".to_string(), response));
        let cache_control = try!(S3Client::<P, D>::get_value_for_header("Cache-Control".to_string(), response));
        let content_length_string = try!(S3Client::<P, D>::get_value_for_header("Content-Length".to_string(),
                                                                                response));
        let content_length = try!(content_length_string.parse::<i32>());
        let expiration = try!(S3Client::<P, D>::get_value_for_header("x-amz-expiration".to_string(), response));
        let missing_meta_string = try!(S3Client::<P, D>::get_value_for_header("x-amz-missing-meta".to_string(),
                                                                              response));
        let missing_meta: i32;
        if missing_meta_string.is_empty() {
            missing_meta = 0;
        } else {
            missing_meta = try!(missing_meta_string.parse::<i32>());
        }
        let restore = try!(S3Client::<P, D>::get_value_for_header("x-amz-restore".to_string(), response));
        let sse_customer_algorithm =
            try!(S3Client::<P, D>::get_value_for_header("x-amz-server-side-encryption-customer-algorithm".to_string(),
                                                        response));
        let content_type = try!(S3Client::<P, D>::get_value_for_header("Content-Type".to_string(), response));
        let content_language = try!(S3Client::<P, D>::get_value_for_header("Content-Language".to_string(), response));
        let version_id = try!(S3Client::<P, D>::get_value_for_header("x-amz-version-id".to_string(), response));
        let e_tag = try!(S3Client::<P, D>::get_value_for_header("ETag".to_string(), response));
        let sse_customer_key_md5 =
            try!(S3Client::<P, D>::get_value_for_header("x-amz-server-side-encryption-customer-key-MD5".to_string(),
                                                        response));
        // make the object to return
        let s3_object = GetObjectOutput {
            delete_marker: delete_marker,
            accept_ranges: accept_ranges,
            last_modified: last_modified,
            content_range: content_range,
            request_charged: request_charged,
            content_encoding: content_encoding,
            replication_status: replication_status,
            storage_class: storage_class,
            server_side_encryption: server_side_encryption,
            ssekms_key_id: ssekms_key_id,
            content_disposition: content_disposition,
            metadata: HashMap::new(),
            body: response.body.clone().into_bytes(),
            website_redirect_location: website_redirect_location,
            expires: expires,
            cache_control: cache_control,
            content_length: content_length,
            expiration: expiration,
            missing_meta: missing_meta,
            restore: restore,
            sse_customer_algorithm: sse_customer_algorithm,
            content_type: content_type,
            content_language: content_language,
            version_id: version_id,
            e_tag: e_tag,
            sse_customer_key_md5: sse_customer_key_md5,
        };
        Ok(s3_object)
    }

    /// Creates a copy of an object that is already stored in Amazon S3.
    pub fn copy_object(&self, input: &CopyObjectRequest) -> Result<CopyObjectOutput, S3Error> {
        let mut request = SignedRequest::new("PUT",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             &format!("/{}", input.key),
                                             &self.endpoint);
        // let mut params = Params::new();
        // params.put("Action", "CopyObject");
        // CopyObjectRequestWriter::write_params(&mut params, "", input);
        // request.set_params(params);

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();

        match status {
            200 => {
                Ok(try!(CopyObjectOutputParser::parse_xml("CopyObjectOutput", &mut stack)))
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error copying object", aws))
            },
        }
    }

    /// This operation enables you to delete multiple objects from a bucket using a
    /// single HTTP request. You may specify up to 1000 keys.
    pub fn delete_objects(&self, input: &DeleteObjectsRequest) -> Result<DeleteObjectsOutput, S3Error> {
        // let mut uri = String::from("/");
        // uri = uri +  &input.key.to_string();
        // let mut request = SignedRequest::new("DELETE", "s3", self.region, &uri);
        // let mut params = Params::new();
        //
        // let hostname = self.hostname(Some(&input.bucket));
        // request.set_hostname(Some(hostname));
        //
        // params.put("Action", "DeleteObjects");
        // DeleteObjectsRequestWriter::write_params(&mut params, "", input);
        // request.set_params(params);
        // let result = sign_and_execute(&self.dispatcher, &mut request, try!(self.credentials_provider.credentials()));
        // let status = result.status;
        // match status {
        //  200 => {
        //      Ok(try!(DeleteObjectsOutputParser::parse_xml("DeleteObjectsOutput", &mut stack)))
        //  }
        //  _ => { Err(S3Error::new("error")) }
        // }
        Err(S3Error::new("not implemented"))
    }

    /// Deletes a given object from the bucket.
    pub fn delete_object(&self,
                         input: &DeleteObjectRequest,
                         operation: Option<&mut Operation>)
                          -> Result<DeleteObjectOutput, S3Error> {
        let path: String;
        if let Some(ref version_id) = input.version_id {
            if self.endpoint.signature == Signature::V2 {
              path = format!("/{}?versionId={}", input.key, version_id);
            } else {
              path = format!("/{}", input.key);
            }
        } else {
            path = format!("/{}", input.key);
        }

        let mut request = SignedRequest::new("DELETE",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             &path,
                                             &self.endpoint);

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        // Params & Writers create x-amz headers and resources that are extracted and formatted
        // correctly during the signing phase.
        if self.endpoint.signature != Signature::V2 {
          let mut params = Params::new();
          DeleteObjectRequestWriter::write_params(&mut params, "", input);
          request.set_params(params);
        }

        let result = new_sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      operation,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;

        match status {
            204 => {
                // NOTE: No payload with 204 but there can be two headers
                // x-amz-delete-marker: true
                Ok(DeleteObjectOutput::default())
            },
            _ => {
                let mut reader = EventReader::from_str(&result.body);
                let mut stack = XmlResponse::new(reader.events().peekable());
                stack.next(); // xml start tag

                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error deleting object", aws))
            },
        }
    }

    /// Initiates a multipart upload and returns an upload ID.
    /// **Note:** After you initiate multipart upload and upload one or more parts, you must
    /// either complete or abort multipart upload in order to stop getting charged for storage of
    /// the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3
    /// frees up the parts storage and stops charging you for the parts storage.
    ///
    /// Keep in mind ways to help performance:
    /// http://docs.aws.amazon.com/AmazonS3/latest/dev/request-rate-perf-considerations.html
    ///
    pub fn multipart_upload_create(&self,
                                   input: &MultipartUploadCreateRequest)
                                   -> Result<MultipartUploadCreateOutput, S3Error> {
        let object_name = &input.key;
        let path: String;

        // NOTE: Need to change these signature items in the signature code instead of in this code set

        if self.endpoint.signature == Signature::V2 {
            path = format!("/{}?uploads", object_name);
        } else {
            path = format!("/{}", object_name);
        }

        let mut request = SignedRequest::new("POST",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             &path,
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("uploads", "");
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;

        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag

        match status {
            200 => {
                Ok(try!(MultipartUploadCreateOutputParser::parse_xml("InitiateMultipartUploadResult", &mut stack)))
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error creating multipart object upload", aws))
            },
        }
    }

    /// Uploads a part in a multipart upload.
    /// **Note:** After you initiate multipart upload and upload one or more parts, you must
    /// either complete or abort multipart upload in order to stop getting charged for storage of
    /// the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3
    /// frees up the parts storage and stops charging you for the parts storage.
    pub fn multipart_upload_part(&self, input: &MultipartUploadPartRequest) -> Result<String, S3Error> {
        let object_name = &input.key;
        let upload_id = &input.upload_id;
        let part_number = &input.part_number;
        let path: String;

        if self.endpoint.signature == Signature::V2 {
            path = format!("/{}?partNumber={}&uploadId={}", object_name, part_number, upload_id);
        } else {
            path = format!("/{}", object_name);
        }

        let mut request = SignedRequest::new("PUT",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             &path,
                                             &self.endpoint);

        request.set_payload(input.body);

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        if let Some(ref md5) = input.content_md5 {
            request.add_header("Content-MD5", md5);
        }

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("partNumber", &format!("{}", part_number));
            params.put("uploadId", upload_id);
            request.set_params(params);
        }

        let mut result = sign_and_execute(&self.dispatcher,
                                          &mut request,
                                          try!(self.credentials_provider.credentials()));
        let status = result.status;

        match status {
            200 => {
                match result.headers.get("ETag") {
                    Some(ref value) => Ok(value.to_string()),
                    None => Err(S3Error::new("Couldn't find etag in response headers.")),
                }
            },
            _ => {
                let mut reader = EventReader::from_str(&result.body);
                let mut stack = XmlResponse::new(reader.events().peekable());
                stack.next(); // xml start tag

                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error uploading a part", aws))
            },
        }
    }

    /// Completes a multipart upload by assembling previously uploaded parts.
    pub fn multipart_upload_complete(&self,
                                     input: &MultipartUploadCompleteRequest)
                                     -> Result<MultipartUploadCompleteOutput, S3Error> {
        let object_name = &input.key;
        let upload_id = &input.upload_id;
        let path: String;

        if self.endpoint.signature == Signature::V2 {
            path = format!("/{}?uploadId={}", object_name, upload_id);
        } else {
            path = format!("/{}", object_name);
        }

        let mut request = SignedRequest::new("POST",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             &path,
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("uploadId", &input.upload_id.to_string());
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        request.set_payload(input.multipart_upload);

        let mut result = sign_and_execute(&self.dispatcher,
                                          &mut request,
                                          try!(self.credentials_provider.credentials()));
        let status = result.status;

        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag

        match status {
            200 => {
                Ok(try!(MultipartUploadCompleteOutputParser::parse_xml("CompleteMultipartUploadResult", &mut stack)))
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error completing multipart upload", aws))
            },
        }
    }

    /// This operation lists in-progress multipart uploads.
    pub fn multipart_upload_list(&self, input: &MultipartUploadListRequest)
                    -> Result<MultipartUploadListOutput, S3Error> {
        let mut request = SignedRequest::new(
                                        "GET",
                                        "s3",
                                        self.region,
                                        &input.bucket,
                                        if self.endpoint.signature == Signature::V4 {"/"} else {"/?uploads"},
                                        &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("uploads", "");
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher, &mut request, try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag

        match status {
            200 => {
                Ok(try!(MultipartUploadListOutputParser::parse_xml("ListMultipartUploadsResult", &mut stack)))
            }
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error completing list_multipart_uploads", aws))
            }
        }
    }

    /// Lists the parts that have been uploaded for a specific multipart upload.
    pub fn multipart_upload_list_parts(&self, input: &MultipartUploadListPartsRequest) -> Result<MultipartUploadListPartsOutput, S3Error> {
        let object_name = &input.key;
        let upload_id = &input.upload_id;
        let path: String;

        if self.endpoint.signature == Signature::V2 {
            path = format!("/{}?uploadId={}", object_name, upload_id);
        } else {
            path = format!("/{}", object_name);
        }

        let mut request = SignedRequest::new(
                                        "GET",
                                        "s3",
                                        self.region,
                                        &input.bucket,
                                        &path,
                                        &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("uploadId", &input.upload_id.to_string());
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let mut result = sign_and_execute(&self.dispatcher, &mut request, try!(self.credentials_provider.credentials()));
        let status = result.status;

        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag

        match status {
            200 => {
                Ok(try!(MultipartUploadListPartsOutputParser::parse_xml("ListPartsResult", &mut stack)))
            }
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error completing list_parts", aws))
            }
        }
    }

    /// Aborts a multipart upload.
    /// To verify that all parts have been removed, so you don't get charged for the
    /// part storage, you should call the List Parts operation and ensure the parts
    /// list is empty.
    pub fn multipart_upload_abort(&self, input: &MultipartUploadAbortRequest)
                            -> Result<MultipartUploadAbortOutput, S3Error> {
        let object_name = &input.key;
        let upload_id = &input.upload_id;
        let path: String;

        if self.endpoint.signature == Signature::V2 {
            path = format!("/{}?uploadId={}", object_name, upload_id);
        } else {
            path = format!("/{}", object_name);
        }

        let mut request = SignedRequest::new(
                                        "DELETE",
                                        "s3",
                                        self.region,
                                        &input.bucket,
                                        &path,
                                        &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("uploadId", &input.upload_id.to_string());
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher, &mut request, try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag

        match status {
            204 => {
                Ok(MultipartUploadAbortOutput::default())
            }
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error completing list_parts", aws))
            }
        }
    }

    /// Restores an archived copy of an object back into Amazon S3
    pub fn restore_object(&self, input: &RestoreObjectRequest) -> Result<RestoreObjectOutput, S3Error> {
        let mut path: String;
        if self.endpoint.signature == Signature::V2 {
            path = format!("/{}?restore", input.key);
        } else {
            path = format!("/{}", input.key);
        }

        let mut request = SignedRequest::new("POST",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             &path,
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("restore", "");
            // params.put("Action", "RestoreObject");
            // RestoreObjectRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());
        stack.next(); // xml start tag
        stack.next();

        match status {
            200 => {
                Ok(try!(RestoreObjectOutputParser::parse_xml("RestoreObjectOutput", &mut stack)))
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error restoring object", aws))
            },
        }
    }

    /// Adds an object to a bucket.
    ///
    /// Keep in mind ways to increase performance:
    /// http://docs.aws.amazon.com/AmazonS3/latest/dev/request-rate-perf-considerations.html
    ///
    /// AWS S3 recommends any PUT/LIST/DELETE operations that exceed 100 per second should open
    /// a support ticket to increase the rate. See the link above more details.
    pub fn put_object(&self,
                      input: &PutObjectRequest,
                      operation: Option<&mut Operation>)
                      -> Result<PutObjectOutput, S3Error> {
        let path: String;
        if input.key.starts_with("/") {
          path = input.key.clone();
        } else {
          path = format!("/{}", input.key);
        }

        let mut request = SignedRequest::new("PUT",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             &path,
                                             &self.endpoint);

        if let Some(ref class) = input.storage_class {
            request.add_header("x-amz-storage-class", class);
        }

        if let Some(ref sse) = input.server_side_encryption {
            if sse.to_string().to_ascii_lowercase() == "aes256" {
                request.add_header("x-amz-server-side-encryption", sse);
            } else {
                match input.ssekms_key_id {
                    Some(ref key_id) => request.add_header("x-amz-server-side-encryption-aws-kms-key-id", key_id),
                    None => return Err(S3Error::new("KMS key specified but no key id provided.")),
                }
                request.add_header("x-amz-server-side-encryption", "aws:kms");
            }
        }

        if let Some(ref cache_control) = input.cache_control {
            request.add_header("Cache-Control", cache_control);
        }

        if let Some(ref md5) = input.content_md5 {
            request.add_header("Content-MD5", md5);
        }

        if let Some(ref metadata) = input.metadata {
            for (key, value) in metadata {
                request.add_header(&format!("x-amz-meta-{}", key), value);
            }
        }

        if let Some(ref acl) = input.acl {
            request.add_header("x-amz-acl", &canned_acl_in_aws_format(acl));
        }

        match input.content_type {
            Some(ref content_type) => request.set_content_type(content_type.to_string()),

            // binary/octet-stream is default per the S3 API docs
            None => request.set_content_type("binary/octet-stream".to_string()),
        };

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));
        request.set_payload(input.body);

        let mut result = new_sign_and_execute(&self.dispatcher,
                                          &mut request,
                                          operation,
                                          try!(self.credentials_provider.credentials()));
        let status = result.status;

        match status {
            200 => {
                let mut put_result = PutObjectOutput::default();

                Ok(put_result)
            },
            _ => {
                let mut reader = EventReader::from_str(&result.body);
                let mut stack = XmlResponse::new(reader.events().peekable());
                stack.next(); // xml start tag

                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error putting object", aws))
            },
        }
    }

    /// uses the acl subresource to set the access control list (ACL) permissions for
    /// an object that already exists in a bucket
    pub fn put_object_acl(&self, input: &PutObjectAclRequest) -> Result<(), S3Error> {
        let mut path: String;
        if self.endpoint.signature == Signature::V2 {
            path = format!("/{}?acl", input.key);
        } else {
            path = format!("/{}", input.key);
        }

        let mut request = SignedRequest::new("PUT",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             &path,
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("acl", "");
            //params.put("Action", "PutObjectAcl");
            //PutObjectAclRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        // Not doing anything but allow unused_variables is set above to kill warning.
        let acls = build_object_acls(&mut request, &input);

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        match status {
            200 => {
                // No response on acl puts
                Ok(())
            }
            _ => {
                let mut reader = EventReader::from_str(&result.body);
                let mut stack = XmlResponse::new(reader.events().peekable());
                stack.next(); // xml start tag

                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error putting object acl", aws))
            }
        }
    }

    /// Returns metadata about all of the versions of objects in a bucket.
    pub fn list_object_versions(&self, input: &ListObjectVersionsRequest) -> Result<ListVersionsResult, S3Error> {
        let mut request = SignedRequest::new("GET",
                                             "s3",
                                             self.region,
                                             &input.bucket,
                                             if self.endpoint.signature == Signature::V2 {"/?versions"} else {"/"},
                                             &self.endpoint);

        if self.endpoint.signature == Signature::V4 {
            let mut params = Params::new();
            params.put("versions", "");
            // params.put("Action", "ListObjectVersions");
            // ListObjectVersionsRequestWriter::write_params(&mut params, "", input);
            request.set_params(params);
        }

        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        let result = sign_and_execute(&self.dispatcher,
                                      &mut request,
                                      try!(self.credentials_provider.credentials()));
        let status = result.status;
        let mut reader = EventReader::from_str(&result.body);
        let mut stack = XmlResponse::new(reader.events().peekable());

        stack.next(); // xml start tag

        match status {
            200 => {
                Ok(try!(ListVersionsResultParser::parse_xml("ListVersionsResult", &mut stack)))
            },
            _ => {
                let aws = try!(AWSError::parse_xml("Error", &mut stack));
                Err(S3Error::with_aws("Error listing object versions", aws))
            },
        }
    }

    // Internal hostname method - Checks for buckets names with '.' in it.
    fn hostname(&self, bucket: Option<&BucketName>) -> String {
        match bucket {
            Some(b) => {
                // NOTE: If the bucket name contains '.' then it must follow path vs virtual bucket
                if b.contains(".") {
                    format!("{}", self.endpoint.hostname().unwrap())
                } else {
                    format!("{}.{}", b, self.endpoint.hostname().unwrap())
                }
            },
            None => format!("{}", self.endpoint.hostname().unwrap()),
        }
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
        if field_in_s3_redirect(&current_name) {
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

fn sign_and_execute<D>(dispatcher: &D,
                       signed_request: &mut SignedRequest,
                       creds: AwsCredentials)
    -> HttpResponse
    where D: DispatchSignedRequest,
{
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

// Internal method that calls the hyper dispatcher to send the URL request.
fn new_sign_and_execute<D>(dispatcher: &D,
                       signed_request: &mut SignedRequest,
                       operation: Option<&mut Operation>,
                       creds: AwsCredentials)
    -> HttpResponse
    where D: DispatchSignedRequest,
{
    signed_request.sign(&creds);
    let response: HttpResponse;

    // NOTE: May want to move to request.rs in dispatcher method instead of here. That would be
    // the lowest level before hyper library. We can actually pull in our own version of hyper
    // since this is a binary and then add time options at the tcp level to measure first byte
    // for latency but we're only interested in throughput and not latency.
    // Latency - Duration from start to first byte.
    // Throughput - Total number of full requests / total time.
    //
    if let Some(op) = operation {
        op.object = format!("{}", signed_request.path);
        op.method = signed_request.method.clone();
        op.request = format!("{}{}{}",
                     signed_request.endpoint.clone().endpoint.unwrap().into_string(),
                     signed_request.bucket,
                     signed_request.path);
        op.endpoint = signed_request.endpoint.clone().endpoint.unwrap().into_string();
        if op.method.to_lowercase() == "put" {
            op.payload_size = signed_request.payload.unwrap().len() as u64;
        }

        let start_time = UTC::now();
        let now = Instant::now();

        response = dispatcher.dispatch(signed_request).expect("Error dispatching timed request");

        op.duration = Some(now.elapsed());
        op.end_time = Some(start_time + chrono::Duration::from_std(op.duration.unwrap()).unwrap());
        op.start_time = Some(start_time);

        if op.method.to_lowercase() != "put" {
            op.payload_size = response.body.len() as u64;
        }
        op.success = if response.status < 400 {true} else {false}; //Do more here later...
        op.code = response.status;
    } else {
        response = dispatcher.dispatch(signed_request).expect("Error dispatching request");
    }

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

// Builds the bucket acl header
fn build_bucket_acls(request: &mut SignedRequest, input: &PutBucketAclRequest) -> Result<(), S3Error> {
    match input.acl {
        Some(ref canned_acl) => request.add_header("x-amz-acl", &canned_acl_in_aws_format(canned_acl)),
        None => {},
    }
    // match input.grant_write {
    // Some(ref grants) => {
    // let grant_str: String = String::new();
    // Cycle through
    // for grant in grants {
    //
    // }
    //
    // request.add_header("x-amz-grant-write", ""),
    // }
    // None => {},
    // }
    //
    Ok(())
}

// Builds the object acl header
fn build_object_acls(request: &mut SignedRequest, input: &PutObjectAclRequest) -> Result<(), S3Error> {
    match input.acl {
        Some(ref canned_acl) => request.add_header("x-amz-acl", &canned_acl_in_aws_format(canned_acl)),
        None => {},
    }
    // match input.grant_write {
    // Some(ref grants) => {
    // let grant_str: String = String::new();
    // Cycle through
    // for grant in grants {
    //
    // }
    //
    // request.add_header("x-amz-grant-write", ""),
    // }
    // None => {},
    // }
    //
    Ok(())
}
