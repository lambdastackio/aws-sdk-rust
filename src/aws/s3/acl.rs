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

use aws::common::params::{Params, ServiceParams};
use aws::common::xmlutil::*;
use aws::common::common::*;
use aws::s3::writeparse::*;
use aws::s3::grant::*;
use aws::s3::header::*;
//use aws::s3::bucket::*;
use aws::s3::object::*;

pub type ObjectCannedACL = String;

pub type Permission = String;

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

/// Parse `GetBucketAclRequest` from XML
pub struct GetBucketAclRequestParser;

/// Write `GetBucketAclRequest` contents to a `SignedRequest`
pub struct GetBucketAclRequestWriter;

/// Parse `PutObjectAclRequest` from XML
pub struct PutObjectAclRequestParser;

/*
/// Parse `GetBucketAclOutput` from XML
pub struct GetBucketAclOutputParser;

/// Write `GetBucketAclOutput` contents to a `SignedRequest`
pub struct GetBucketAclOutputWriter;
*/

/// Write `PutObjectAclRequest` contents to a `SignedRequest`
pub struct PutObjectAclRequestWriter;

/// Parse `AccessControlPolicy` from XML
pub struct AccessControlPolicyParser;

/// Write `AccessControlPolicy` contents to a `SignedRequest`
pub struct AccessControlPolicyWriter;

/// Parse `AccessControlList` from XML
pub struct AccessControlListParser;

/// Write `AccessControlList` contents to a `SignedRequest`
pub struct AccessControlListWriter;

/// Parse `ObjectCannedACL` from XML
pub struct ObjectCannedACLParser;

/// Write `ObjectCannedACL` contents to a `SignedRequest`
pub struct ObjectCannedACLWriter;

/// Parse `Permission` from XML
pub struct PermissionParser;

/// Write `Permission` contents to a `SignedRequest`
pub struct PermissionWriter;

#[derive(Debug, Default)]
pub struct AccessControlPolicy {
    pub owner: Owner,
    pub acl: AccessControlList,
}

#[derive(Debug, Default)]
pub struct AccessControlList {
    pub grants: Grants,
}

// NOTE: May remove this one...
/*
#[derive(Debug, Default)]
pub struct GetBucketAclOutput {
    pub owner: Owner,
    //pub acl: AccessControlList,
    pub grants: Grants,
}
*/

#[derive(Debug, Default)]
pub struct GetBucketAclRequest {
    pub bucket: BucketName,
}

#[derive(Debug, Default)]
pub struct PutBucketAclRequest {
    /// Allows grantee the read, write, read ACP, and write ACP permissions on the
    /// bucket.
    pub grant_full_control: Option<GrantFullControl>,
    /// Allows grantee to write the ACL for the applicable bucket.
    pub grant_write_acp: Option<GrantWriteACP>,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
    /// The canned ACL to apply to the bucket.
    pub acl: Option<CannedAcl>,
    pub access_control_policy: Option<AccessControlPolicy>,
    /// Allows grantee to create, overwrite, and delete any object in the bucket.
    pub grant_write: Option<GrantWrite>,
    /// Allows grantee to list the objects in the bucket.
    pub grant_read: Option<GrantRead>,
    /// Allows grantee to read the bucket ACL.
    pub grant_read_acp: Option<GrantReadACP>,
}

// Impls below...

impl PutObjectAclRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutObjectAclRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutObjectAclRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-grant-full-control" {
                obj.grant_full_control = Some(try!(GrantFullControlParser::parse_xml("x-amz-grant-full-control", stack)));
                continue;
            }
            if current_name == "x-amz-grant-write-acp" {
                obj.grant_write_acp = Some(try!(GrantWriteACPParser::parse_xml("x-amz-grant-write-acp", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "AccessControlPolicy" {
                obj.access_control_policy = Some(try!(AccessControlPolicyParser::parse_xml("AccessControlPolicy", stack)));
                continue;
            }
            if current_name == "x-amz-grant-write" {
                obj.grant_write = Some(try!(GrantWriteParser::parse_xml("x-amz-grant-write", stack)));
                continue;
            }
            if current_name == "x-amz-grant-read" {
                obj.grant_read = Some(try!(GrantReadParser::parse_xml("x-amz-grant-read", stack)));
                continue;
            }
            if current_name == "x-amz-grant-read-acp" {
                obj.grant_read_acp = Some(try!(GrantReadACPParser::parse_xml("x-amz-grant-read-acp", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl PutObjectAclRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &PutObjectAclRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.grant_full_control {
            GrantFullControlWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-full-control"), obj);
        }
        if let Some(ref obj) = obj.grant_write_acp {
            GrantWriteACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-write-acp"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.access_control_policy {
            AccessControlPolicyWriter::write_params(params, &(prefix.to_string() + "AccessControlPolicy"), obj);
        }
        if let Some(ref obj) = obj.grant_write {
            GrantWriteWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-write"), obj);
        }
        if let Some(ref obj) = obj.grant_read {
            GrantReadWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read"), obj);
        }
        if let Some(ref obj) = obj.grant_read_acp {
            GrantReadACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read-acp"), obj);
        }
    }
}

impl GetBucketAclRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketAclRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketAclRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl GetBucketAclRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetBucketAclRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
/*
impl GetBucketAclOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketAclOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketAclOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            if current_name == "Grant" {
                obj.grants = try!(GrantsParser::parse_xml("Grant", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl GetBucketAclOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetBucketAclOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        GrantsWriter::write_params(params, &(prefix.to_string() + "Grant"), &obj.grants);
    }
}
*/

impl AccessControlPolicyParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AccessControlPolicy, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = AccessControlPolicy::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            if current_name == "AccessControlList" {
                obj.acl = try!(AccessControlListParser::parse_xml("AccessControlList", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl AccessControlPolicyWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &AccessControlPolicy) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        AccessControlListWriter::write_params(params, &(prefix.to_string() + "AccessControlList"), &obj.acl);
    }
}

impl AccessControlListParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AccessControlList, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = AccessControlList::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Grant" {
                obj.grants = try!(GrantsParser::parse_xml("Grant", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl AccessControlListWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &AccessControlList) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        GrantsWriter::write_params(params, &(prefix.to_string() + "Grant"), &obj.grants);
    }
}

impl ObjectCannedACLParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectCannedACL, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ObjectCannedACLWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectCannedACL) {
        params.put(name, obj);
    }
}

impl PermissionParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Permission, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl PermissionWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Permission) {
        params.put(name, obj);
    }
}

/// Maps canned acl to AWS format.
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
