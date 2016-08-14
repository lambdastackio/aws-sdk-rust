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

pub type Location = String;
/// Parse `Location` from XML
struct LocationParser;
impl LocationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Location, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Location` contents to a `SignedRequest`
struct LocationWriter;
impl LocationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Location) {
        params.put(name, obj);
    }
}

#[derive(Debug, Default)]
pub struct CreateBucketOutput {
    pub location: Location,
}

/// Parse `CreateBucketOutput` from XML
struct CreateBucketOutputParser;
impl CreateBucketOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CreateBucketOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CreateBucketOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Location" {
                obj.location = try!(LocationParser::parse_xml("Location", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CreateBucketOutput` contents to a `SignedRequest`
struct CreateBucketOutputWriter;
impl CreateBucketOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CreateBucketOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LocationWriter::write_params(params, &(prefix.to_string() + "Location"), &obj.location);
    }
}

pub type S3ClientMessage = String;
/// Parse `S`3ClientMessage from XML
pub struct S3ClientMessageParser;

impl S3ClientMessageParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<S3ClientMessage, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `S3ClientMessage` contents to a `SignedRequest`
pub struct S3ClientMessageWriter;

impl S3ClientMessageWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &S3ClientMessage) {
        params.put(name, obj);
    }
}

#[derive(Debug, Default)]
pub struct ListBucketsOutput {
    pub owner: Owner,
    pub buckets: Buckets,
}

/// Parse `ListBucketsOutput` from XML
struct ListBucketsOutputParser;
impl ListBucketsOutputParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListBucketsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListBucketsOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            match current_name.as_ref() {
                "Owner" => {
                    obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                    continue;
                },
                "Buckets" => {
                    stack.next(); // skip Buckets start and go to contents
                    // this will parse all buckets:
                    obj.buckets = try!(BucketsParser::parse_xml("Bucket", stack));
                },
                _ => break,
            }
        }
        stack.next(); // skip Buckets end
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ListBucketsOutput` contents to a `SignedRequest`
struct ListBucketsOutputWriter;
impl ListBucketsOutputWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ListBucketsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        BucketsWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.buckets);
    }
}

#[derive(Debug)]
pub struct S3Client<P, D> where P: AwsCredentialsProvider, D: DispatchSignedRequest {
            credentials_provider: P,
            region: Region,
            dispatcher: D,
        }

impl<P> S3Client<P, Client> where P: AwsCredentialsProvider {
    pub fn new(credentials_provider: P, region: Region) -> Self {
        let mut client = Client::new();
        client.set_redirect_policy(RedirectPolicy::FollowNone);
        S3Client::with_request_dispatcher(client, credentials_provider, region)
    }
}

impl<P, D> S3Client<P, D> where P: AwsCredentialsProvider, D: DispatchSignedRequest {
    pub fn with_request_dispatcher(request_dispatcher: D, credentials_provider: P, region: Region) -> Self {
        S3Client {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher
        }
    }

    /// Creates a new bucket.
    /// All requests go to the us-east-1/us-standard endpoint, but can create buckets anywhere.
    pub fn create_bucket(&self, input: &CreateBucketRequest) -> Result<CreateBucketOutput, S3Error> {
        let region = Region::UsEast1;
        let mut create_config : Vec<u8>;
        let mut request = SignedRequest::new("PUT", "s3", region, "");
        let hostname = self.hostname(Some(&input.bucket));
        request.set_hostname(Some(hostname));

        if needs_create_bucket_config(self.region) {
            create_config = create_bucket_config_xml(self.region);
            request.set_payload(Some(&create_config));
        }

        match input.acl {
            None => (),
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
        let mut request = SignedRequest::new("GET", "s3", self.region, "/");
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
        let host = match self.region {
                    Region::UsEast1 => "s3.amazonaws.com".to_string(),
                    Region::CnNorth1 => format!("s3.{}.amazonaws.com.cn", self.region),
                    _ => format!("s3-{}.amazonaws.com", self.region),
                };

        match bucket {
            Some(b) => format!("{}.s3.amazonaws.com", b),
            None => host,
        }
    }
}

/// Helper function to determine if a create config is needed.
pub fn needs_create_bucket_config(region: Region) -> bool {
    match region {
        Region::UsEast1 => false,
        _ => true,
    }
}

// This is a bit hacky to get functionality until we figure out an XML writing util.
/// Manually writes out bucket configuration (location constraint) in XML.
pub fn create_bucket_config_xml(region: Region) -> Vec<u8> {
    match region {
        Region::UsEast1 => {
            Vec::new() // shouldn't actually execute this: panic! or unreachable! this?
        }
        _ => {
            let xml = format!("<CreateBucketConfiguration xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">
        <LocationConstraint>{}</LocationConstraint>
        </CreateBucketConfiguration >", region);
            xml.into_bytes()
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

fn sign_and_execute<D>(dispatcher: &D, request: &mut SignedRequest, creds: AwsCredentials) -> HttpResponse where D: DispatchSignedRequest{
    request.sign(&creds);
    let response = dispatcher.dispatch(request).expect("Error dispatching request");
    debug!("Sent request to AWS");

    if response.status == 307 {
        debug!("Got a redirect response, resending request.");
        // extract location from response, modify request and re-sign and resend.
        let new_hostname = extract_s3_redirect_location(response).unwrap();
        request.set_hostname(Some(new_hostname.to_string()));

        // This does a lot of appending and not clearing/creation, so we'll have to do that ourselves:
        request.sign(&creds);
        return dispatcher.dispatch(request).unwrap();
    }

    response
}

pub type ObjectCannedACL = String;
/// Parse `ObjectCannedACL` from XML
struct ObjectCannedACLParser;
impl ObjectCannedACLParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectCannedACL, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectCannedACL` contents to a `SignedRequest`
struct ObjectCannedACLWriter;
impl ObjectCannedACLWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ObjectCannedACL) {
        params.put(name, obj);
    }
}

/// Canned ACL for S3
#[derive(Debug)]
pub enum CannedAcl {
    Private,
    PublicRead,
    PublicReadWrite,
    AuthenticatedRead,
    BucketOwnerRead,
    BucketOwnerFullControl,
}

#[derive(Debug, Default)]
pub struct CreateBucketRequest {
    /// Allows grantee the read, write, read ACP, and write ACP permissions on the
    /// bucket.
    pub grant_full_control: Option<GrantFullControl>,
    pub create_bucket_configuration: Option<CreateBucketConfiguration>,
    /// Allows grantee to write the ACL for the applicable bucket.
    pub grant_write_acp: Option<GrantWriteACP>,
    pub bucket: BucketName,
    /// The canned ACL to apply to the bucket.
    pub acl: Option<CannedAcl>,
    /// Allows grantee to create, overwrite, and delete any object in the bucket.
    pub grant_write: Option<GrantWrite>,
    /// Allows grantee to list the objects in the bucket.
    pub grant_read: Option<GrantRead>,
    /// Allows grantee to read the bucket ACL.
    pub grant_read_acp: Option<GrantReadACP>,
}

pub type BucketName = String;
/// Parse `BucketName` from XML
struct BucketNameParser;
impl BucketNameParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketName, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

#[derive(Debug, Default)]
pub struct Bucket {
    /// Date the bucket was created.
    pub creation_date: CreationDate,
    /// The name of the bucket.
    pub name: BucketName,
}

/// Parse `Bucket` from XML
struct BucketParser;
impl BucketParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Bucket, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Bucket::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "CreationDate" {
                obj.creation_date = try!(CreationDateParser::parse_xml("CreationDate", stack));
                continue;
            }
            if current_name == "Name" {
                obj.name = try!(BucketNameParser::parse_xml("Name", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `BucketName` contents to a `SignedRequest`
struct BucketNameWriter;
impl BucketNameWriter {
    fn write_params(params: &mut Params, name: &str, obj: &BucketName) {
        params.put(name, obj);
    }
}

/// Write `Bucket` contents to a `SignedRequest`
struct BucketWriter;
impl BucketWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Bucket) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        CreationDateWriter::write_params(params, &(prefix.to_string() + "CreationDate"), &obj.creation_date);
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
    }
}

pub type GrantReadACP = String;
/// Parse `GrantReadACP` from XML
struct GrantReadACPParser;
impl GrantReadACPParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantReadACP, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GrantReadACP` contents to a `SignedRequest`
struct GrantReadACPWriter;
impl GrantReadACPWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GrantReadACP) {
        params.put(name, obj);
    }
}
#[derive(Debug, Default)]
pub struct Grant {
    pub grantee: Grantee,
    /// Specifies the permission given to the grantee.
    pub permission: Permission,
}

/// Parse `Grant` from XML
struct GrantParser;
impl GrantParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Grant, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Grant::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Grantee" {
                obj.grantee = try!(GranteeParser::parse_xml("Grantee", stack));
                continue;
            }
            if current_name == "Permission" {
                obj.permission = try!(PermissionParser::parse_xml("Permission", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Grant` contents to a `SignedRequest`
struct GrantWriter;
impl GrantWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Grant) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        GranteeWriter::write_params(params, &(prefix.to_string() + "Grantee"), &obj.grantee);
        PermissionWriter::write_params(params, &(prefix.to_string() + "Permission"), &obj.permission);
    }
}

/// Write `Grantee` contents to a `SignedRequest`
struct GranteeWriter;
impl GranteeWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Grantee) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.email_address {
            EmailAddressWriter::write_params(params, &(prefix.to_string() + "EmailAddress"), obj);
        }
        TypeWriter::write_params(params, &(prefix.to_string() + "xsi:type"), &obj.foo_type);
        if let Some(ref obj) = obj.display_name {
            DisplayNameWriter::write_params(params, &(prefix.to_string() + "DisplayName"), obj);
        }
        if let Some(ref obj) = obj.id {
            IDWriter::write_params(params, &(prefix.to_string() + "ID"), obj);
        }
        if let Some(ref obj) = obj.uri {
            URIWriter::write_params(params, &(prefix.to_string() + "URI"), obj);
        }
    }
}

pub type Permission = String;
/// Parse `Permission` from XML
struct PermissionParser;
impl PermissionParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Permission, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Permission` contents to a `SignedRequest`
struct PermissionWriter;
impl PermissionWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Permission) {
        params.put(name, obj);
    }
}

pub type URI = String;
/// Parse `URI` from XML
struct URIParser;
impl URIParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<URI, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `URI` contents to a `SignedRequest`
struct URIWriter;
impl URIWriter {
    fn write_params(params: &mut Params, name: &str, obj: &URI) {
        params.put(name, obj);
    }
}

pub type ID = String;
/// Parse `ID` from XML
struct IDParser;
impl IDParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ID, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ID` contents to a `SignedRequest`
struct IDWriter;
impl IDWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ID) {
        params.put(name, obj);
    }
}

pub type ContentType = String;
/// Parse `ContentType` from XML
struct ContentTypeParser;
impl ContentTypeParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentType, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ContentType` contents to a `SignedRequest`
struct ContentTypeWriter;
impl ContentTypeWriter {
    fn write_params(params: &mut Params, name: &str, obj: &ContentType) {
        params.put(name, obj);
    }
}

pub type Type = String;
/// Parse `Type` from XML
struct TypeParser;
impl TypeParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Type, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Type` contents to a `SignedRequest`
struct TypeWriter;
impl TypeWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Type) {
        params.put(name, obj);
    }
}
pub type Buckets = Vec<Bucket>;
/// Parse `Buckets` from XML
struct BucketsParser;
impl BucketsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Buckets, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Bucket" {
            obj.push(try!(BucketParser::parse_xml("Bucket", stack)));
        }
        Ok(obj)
    }
}
/// Write `Buckets` contents to a `SignedRequest`
struct BucketsWriter;
impl BucketsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Buckets) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            BucketWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

pub type EmailAddress = String;
/// Parse `EmailAddress` from XML
struct EmailAddressParser;
impl EmailAddressParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<EmailAddress, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `EmailAddress` contents to a `SignedRequest`
struct EmailAddressWriter;
impl EmailAddressWriter {
    fn write_params(params: &mut Params, name: &str, obj: &EmailAddress) {
        params.put(name, obj);
    }
}

#[derive(Debug, Default)]
pub struct Grantee {
    /// Email address of the grantee.
    pub email_address: Option<EmailAddress>,
    /// Type of grantee
    pub foo_type: Type,
    /// Screen name of the grantee.
    pub display_name: Option<DisplayName>,
    /// The canonical user ID of the grantee.
    pub id: Option<ID>,
    /// URI of the grantee group.
    pub uri: Option<URI>,
}

/// Parse `Grantee` from XML
struct GranteeParser;
impl GranteeParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Grantee, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Grantee::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "EmailAddress" {
                obj.email_address = Some(try!(EmailAddressParser::parse_xml("EmailAddress", stack)));
                continue;
            }
            if current_name == "xsi:type" {
                obj.foo_type = try!(TypeParser::parse_xml("xsi:type", stack));
                continue;
            }
            if current_name == "DisplayName" {
                obj.display_name = Some(try!(DisplayNameParser::parse_xml("DisplayName", stack)));
                continue;
            }
            if current_name == "ID" {
                obj.id = Some(try!(IDParser::parse_xml("ID", stack)));
                continue;
            }
            if current_name == "URI" {
                obj.uri = Some(try!(URIParser::parse_xml("URI", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

pub type DisplayName = String;
/// Parse `DisplayName` from XML
struct DisplayNameParser;
impl DisplayNameParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DisplayName, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `DisplayName` contents to a `SignedRequest`
struct DisplayNameWriter;
impl DisplayNameWriter {
    fn write_params(params: &mut Params, name: &str, obj: &DisplayName) {
        params.put(name, obj);
    }
}

pub type GrantRead = String;
/// Parse `GrantRead` from XML
struct GrantReadParser;
impl GrantReadParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantRead, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GrantRead` contents to a `SignedRequest`
struct GrantReadWriter;
impl GrantReadWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GrantRead) {
        params.put(name, obj);
    }
}

pub type CreationDate = String;
/// Parse `CreationDate` from XML
struct CreationDateParser;
impl CreationDateParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CreationDate, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `CreationDate` contents to a `SignedRequest`
struct CreationDateWriter;
impl CreationDateWriter {
    fn write_params(params: &mut Params, name: &str, obj: &CreationDate) {
        params.put(name, obj);
    }
}

pub type GrantWriteACP = String;
/// Parse `GrantWriteACP` from XML
struct GrantWriteACPParser;
impl GrantWriteACPParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantWriteACP, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GrantWriteACP` contents to a `SignedRequest`
struct GrantWriteACPWriter;
impl GrantWriteACPWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GrantWriteACP) {
        params.put(name, obj);
    }
}

pub type GrantWrite = String;
/// Parse `GrantWrite` from XML
struct GrantWriteParser;
impl GrantWriteParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantWrite, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GrantWrite` contents to a `SignedRequest`
struct GrantWriteWriter;
impl GrantWriteWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GrantWrite) {
        params.put(name, obj);
    }
}

#[derive(Debug, Default)]
pub struct CreateBucketConfiguration {
    /// Specifies the region where the bucket will be created. If you don't specify a
    /// region, the bucket will be created in US Standard.
    pub location_constraint: BucketLocationConstraint,
}

pub type GrantFullControl = String;
/// Parse `GrantFullControl` from XML
struct GrantFullControlParser;
impl GrantFullControlParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantFullControl, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GrantFullControl` contents to a `SignedRequest`
struct GrantFullControlWriter;
impl GrantFullControlWriter {
    fn write_params(params: &mut Params, name: &str, obj: &GrantFullControl) {
        params.put(name, obj);
    }
}

/// The requested bucket name is not available. The bucket namespace is shared by
/// all users of the system. Please select a different name and try again.
#[derive(Debug, Default)]
pub struct BucketAlreadyExists;

/// Parse `BucketAlreadyExists` from XML
struct BucketAlreadyExistsParser;
impl BucketAlreadyExistsParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketAlreadyExists, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = BucketAlreadyExists::default();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `BucketAlreadyExists` contents to a `SignedRequest`
struct BucketAlreadyExistsWriter;
impl BucketAlreadyExistsWriter {
    fn write_params(params: &mut Params, name: &str, obj: &BucketAlreadyExists) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
    }
}
pub type BucketLocationConstraint = String;
/// Parse `BucketLocationConstraint` from XML
struct BucketLocationConstraintParser;
impl BucketLocationConstraintParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketLocationConstraint, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `BucketLocationConstraint` contents to a `SignedRequest`
struct BucketLocationConstraintWriter;
impl BucketLocationConstraintWriter {
    fn write_params(params: &mut Params, name: &str, obj: &BucketLocationConstraint) {
        params.put(name, obj);
    }
}

#[derive(Debug, Default)]
pub struct Owner {
    pub display_name: DisplayName,
    pub id: ID,
}

/// Parse `Owner` from XML
struct OwnerParser;
impl OwnerParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Owner, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Owner::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            match current_name.as_ref() {
                "DisplayName" => {
                    obj.display_name = try!(DisplayNameParser::parse_xml("DisplayName", stack));
                    continue;
                },
                "ID" => {
                    obj.id = try!(IDParser::parse_xml("ID", stack));
                    continue;
                },
                _ => break,
            }
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Owner` contents to a `SignedRequest`
struct OwnerWriter;
impl OwnerWriter {
    fn write_params(params: &mut Params, name: &str, obj: &Owner) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DisplayNameWriter::write_params(params, &(prefix.to_string() + "DisplayName"), &obj.display_name);
        IDWriter::write_params(params, &(prefix.to_string() + "ID"), &obj.id);
    }
}

pub type Code = String;
/// Parse `Code` from XML
pub struct CodeParser;

impl CodeParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Code, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Code` contents to a `SignedRequest`
pub struct CodeWriter;

impl CodeWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Code) {
        params.put(name, obj);
    }
}

#[derive(Debug, Default)]
pub struct ObjectVersion {
    /// Date and time the object was last modified.
    pub last_modified: LastModified,
    /// Version ID of an object.
    pub version_id: ObjectVersionId,
    pub e_tag: ETag,
    /// The class of storage used to store the object.
    pub storage_class: ObjectVersionStorageClass,
    /// The object key.
    pub key: ObjectKey,
    pub owner: Owner,
    /// Specifies whether the object is (true) or is not (false) the latest version of
    /// an object.
    pub is_latest: IsLatest,
    /// Size in bytes of the object.
    pub size: Size,
}

/// Parse `ObjectVersion` from XML
pub struct ObjectVersionParser;
impl ObjectVersionParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectVersion, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ObjectVersion::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LastModified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
                continue;
            }
            if current_name == "VersionId" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("VersionId", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "StorageClass" {
                obj.storage_class = try!(ObjectVersionStorageClassParser::parse_xml("StorageClass", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            if current_name == "IsLatest" {
                obj.is_latest = try!(IsLatestParser::parse_xml("IsLatest", stack));
                continue;
            }
            if current_name == "Size" {
                obj.size = try!(SizeParser::parse_xml("Size", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectVersion` contents to a `SignedRequest`
pub struct ObjectVersionWriter;

impl ObjectVersionWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectVersion) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        ObjectVersionStorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        IsLatestWriter::write_params(params, &(prefix.to_string() + "IsLatest"), &obj.is_latest);
        SizeWriter::write_params(params, &(prefix.to_string() + "Size"), &obj.size);
    }
}

pub type ObjectKey = String;
/// Parse `ObjectKey` from XML
pub struct ObjectKeyParser;
impl ObjectKeyParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectKey, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectKey` contents to a `SignedRequest`
pub struct ObjectKeyWriter;
impl ObjectKeyWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectKey) {
        params.put(name, obj);
    }
}

pub type ObjectVersionStorageClass = String;
/// Parse `ObjectVersionStorageClass` from XML
pub struct ObjectVersionStorageClassParser;
impl ObjectVersionStorageClassParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectVersionStorageClass, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectVersionStorageClass` contents to a `SignedRequest`
pub struct ObjectVersionStorageClassWriter;
impl ObjectVersionStorageClassWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectVersionStorageClass) {
        params.put(name, obj);
    }
}

pub type Size = i32;
/// Parse `Size` from XML
pub struct SizeParser;
impl SizeParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Size, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Size` contents to a `SignedRequest`
pub struct SizeWriter;
impl SizeWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Size) {
        params.put(name, &obj.to_string());
    }
}

pub type IsLatest = bool;
/// Parse `IsLatest` from XML
pub struct IsLatestParser;
impl IsLatestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<IsLatest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `IsLatest` contents to a `SignedRequest`
pub struct IsLatestWriter;
impl IsLatestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &IsLatest) {
        params.put(name, &obj.to_string());
    }
}

pub type ETag = String;
/// Parse `ETag` from XML
pub struct ETagParser;
impl ETagParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ETag, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ETag` contents to a `SignedRequest`
pub struct ETagWriter;
impl ETagWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ETag) {
        params.put(name, obj);
    }
}

pub type ObjectVersionId = String;
/// Parse `ObjectVersionId` from XML
pub struct ObjectVersionIdParser;
impl ObjectVersionIdParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectVersionId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectVersionId` contents to a `SignedRequest`
pub struct ObjectVersionIdWriter;
impl ObjectVersionIdWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectVersionId) {
        params.put(name, obj);
    }
}

pub type LastModified = String;
/// Parse `LastModified` from XML
pub struct LastModifiedParser;
impl LastModifiedParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LastModified, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `LastModified` contents to a `SignedRequest`
pub struct LastModifiedWriter;
impl LastModifiedWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &LastModified) {
        params.put(name, obj);
    }
}
