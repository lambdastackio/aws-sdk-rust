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

use std::str::FromStr;
use std::str;

use aws::common::params::{Params, ServiceParams};
use aws::common::xmlutil::*;
use aws::s3::bucket::Bucket;

pub type ObjectCannedACL = String;
/// Parse `ObjectCannedACL` from XML
pub struct ObjectCannedACLParser;

impl ObjectCannedACLParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectCannedACL, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ObjectCannedACL` contents to a `SignedRequest`
pub struct ObjectCannedACLWriter;

impl ObjectCannedACLWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectCannedACL) {
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
pub struct ListBucketsOutput {
    pub owner: Owner,
    pub buckets: Buckets,
}

/// Parse `ListBucketsOutput` from XML
pub struct ListBucketsOutputParser;

impl ListBucketsOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListBucketsOutput, XmlParseError> {
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
pub struct ListBucketsOutputWriter;
impl ListBucketsOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ListBucketsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        BucketsWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.buckets);
    }
}

#[derive(Debug, Default)]
pub struct CreateBucketOutput {
    pub location: Location,
}

/// Parse `CreateBucketOutput` from XML
pub struct CreateBucketOutputParser;

impl CreateBucketOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CreateBucketOutput, XmlParseError> {
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
pub struct CreateBucketOutputWriter;

impl CreateBucketOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CreateBucketOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LocationWriter::write_params(params, &(prefix.to_string() + "Location"), &obj.location);
    }
}

pub type BucketName = String;
/// Parse `BucketName` from XML
pub struct BucketNameParser;

impl BucketNameParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketName, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Parse `Bucket` from XML
pub struct BucketParser;

impl BucketParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Bucket, XmlParseError> {
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
pub struct BucketNameWriter;

impl BucketNameWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &BucketName) {
        params.put(name, obj);
    }
}

/// Write `Bucket` contents to a `SignedRequest`
pub struct BucketWriter;

impl BucketWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Bucket) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        CreationDateWriter::write_params(params, &(prefix.to_string() + "CreationDate"), &obj.creation_date);
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
    }
}

pub type Buckets = Vec<Bucket>;
/// Parse `Buckets` from XML
pub struct BucketsParser;

impl BucketsParser {
    #[allow(unused_variables)]
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Buckets, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Bucket" {
            obj.push(try!(BucketParser::parse_xml("Bucket", stack)));
        }
        Ok(obj)
    }
}

/// Write `Buckets` contents to a `SignedRequest`
pub struct BucketsWriter;

impl BucketsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Buckets) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            BucketWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

#[derive(Debug, Default)]
pub struct CreateBucketConfiguration {
    /// Specifies the region where the bucket will be created. If you don't specify a
    /// region, the bucket will be created in US Standard.
    pub location_constraint: BucketLocationConstraint,
}

/// The requested bucket name is not available. The bucket namespace is shared by
/// all users of the system. Please select a different name and try again.
#[derive(Debug, Default)]
pub struct BucketAlreadyExists;

/// Parse `BucketAlreadyExists` from XML
pub struct BucketAlreadyExistsParser;

impl BucketAlreadyExistsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketAlreadyExists, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = BucketAlreadyExists::default();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `BucketAlreadyExists` contents to a `SignedRequest`
pub struct BucketAlreadyExistsWriter;

impl BucketAlreadyExistsWriter {
    #[allow(unused_variables)]
    pub fn write_params(params: &mut Params, name: &str, obj: &BucketAlreadyExists) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
    }
}

pub type BucketLocationConstraint = String;
/// Parse `BucketLocationConstraint` from XML
pub struct BucketLocationConstraintParser;

impl BucketLocationConstraintParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketLocationConstraint, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `BucketLocationConstraint` contents to a `SignedRequest`
pub struct BucketLocationConstraintWriter;

impl BucketLocationConstraintWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &BucketLocationConstraint) {
        params.put(name, obj);
    }
}

pub type Location = String;
/// Parse `Location` from XML
pub struct LocationParser;

impl LocationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Location, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Location` contents to a `SignedRequest`
pub struct LocationWriter;

impl LocationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Location) {
        params.put(name, obj);
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

pub type GrantReadACP = String;
/// Parse `GrantReadACP` from XML
pub struct GrantReadACPParser;

impl GrantReadACPParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantReadACP, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GrantReadACP` contents to a `SignedRequest`
pub struct GrantReadACPWriter;

impl GrantReadACPWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GrantReadACP) {
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
pub struct GrantParser;

impl GrantParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Grant, XmlParseError> {
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
pub struct GrantWriter;

impl GrantWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Grant) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        GranteeWriter::write_params(params, &(prefix.to_string() + "Grantee"), &obj.grantee);
        PermissionWriter::write_params(params, &(prefix.to_string() + "Permission"), &obj.permission);
    }
}

/// Write `Grantee` contents to a `SignedRequest`
pub struct GranteeWriter;

impl GranteeWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Grantee) {
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

pub type URI = String;
/// Parse `URI` from XML
pub struct URIParser;

impl URIParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<URI, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `URI` contents to a `SignedRequest`
pub struct URIWriter;

impl URIWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &URI) {
        params.put(name, obj);
    }
}

pub type ID = String;
/// Parse `ID` from XML
pub struct IDParser;

impl IDParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ID, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ID` contents to a `SignedRequest`
pub struct IDWriter;

impl IDWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ID) {
        params.put(name, obj);
    }
}

pub type ContentType = String;
/// Parse `ContentType` from XML
pub struct ContentTypeParser;

impl ContentTypeParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentType, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ContentType` contents to a `SignedRequest`
pub struct ContentTypeWriter;

impl ContentTypeWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ContentType) {
        params.put(name, obj);
    }
}

pub type Type = String;
/// Parse `Type` from XML
pub struct TypeParser;

impl TypeParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Type, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Type` contents to a `SignedRequest`
pub struct TypeWriter;

impl TypeWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Type) {
        params.put(name, obj);
    }
}

pub type EmailAddress = String;
/// Parse `EmailAddress` from XML
pub struct EmailAddressParser;

impl EmailAddressParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<EmailAddress, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `EmailAddress` contents to a `SignedRequest`
pub struct EmailAddressWriter;

impl EmailAddressWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &EmailAddress) {
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
pub struct GranteeParser;

impl GranteeParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Grantee, XmlParseError> {
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
pub struct DisplayNameParser;

impl DisplayNameParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DisplayName, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `DisplayName` contents to a `SignedRequest`
pub struct DisplayNameWriter;

impl DisplayNameWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DisplayName) {
        params.put(name, obj);
    }
}

pub type GrantRead = String;
/// Parse `GrantRead` from XML
pub struct GrantReadParser;

impl GrantReadParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantRead, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `GrantRead` contents to a `SignedRequest`
pub struct GrantReadWriter;

impl GrantReadWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GrantRead) {
        params.put(name, obj);
    }
}

pub type CreationDate = String;
/// Parse `CreationDate` from XML
pub struct CreationDateParser;

impl CreationDateParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CreationDate, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CreationDate` contents to a `SignedRequest`
pub struct CreationDateWriter;

impl CreationDateWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CreationDate) {
        params.put(name, obj);
    }
}

pub type GrantWriteACP = String;
/// Parse `GrantWriteACP` from XML
pub struct GrantWriteACPParser;

impl GrantWriteACPParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantWriteACP, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `GrantWriteACP` contents to a `SignedRequest`
pub struct GrantWriteACPWriter;

impl GrantWriteACPWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GrantWriteACP) {
        params.put(name, obj);
    }
}

pub type GrantWrite = String;
/// Parse `GrantWrite` from XML
pub struct GrantWriteParser;

impl GrantWriteParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantWrite, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `GrantWrite` contents to a `SignedRequest`
pub struct GrantWriteWriter;

impl GrantWriteWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GrantWrite) {
        params.put(name, obj);
    }
}

pub type Permission = String;
/// Parse `Permission` from XML
pub struct PermissionParser;

impl PermissionParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Permission, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Permission` contents to a `SignedRequest`
pub struct PermissionWriter;

impl PermissionWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Permission) {
        params.put(name, obj);
    }
}

pub type GrantFullControl = String;
/// Parse `GrantFullControl` from XML
pub struct GrantFullControlParser;

impl GrantFullControlParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantFullControl, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `GrantFullControl` contents to a `SignedRequest`
pub struct GrantFullControlWriter;

impl GrantFullControlWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GrantFullControl) {
        params.put(name, obj);
    }
}

#[derive(Debug, Default)]
pub struct Owner {
    pub display_name: DisplayName,
    pub id: ID,
}

/// Parse `Owner` from XML
pub struct OwnerParser;

impl OwnerParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Owner, XmlParseError> {
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
pub struct OwnerWriter;

impl OwnerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Owner) {
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
