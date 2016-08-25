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
use aws::s3::acl::*;

#[derive(Debug, Default)]
pub struct TargetGrant {
    pub grantee: Grantee,
    /// Logging permissions assigned to the Grantee for the bucket.
    pub permission: BucketLogsPermission,
}

#[derive(Debug, Default)]
pub struct Grant {
    pub grantee: Grantee,
    /// Specifies the permission given to the grantee.
    pub permission: Permission,
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

pub type GrantRead = String;

pub type GrantWrite = String;

pub type GrantWriteACP = String;

pub type GrantReadACP = String;

pub type GrantFullControl = String;

pub type Grants = Vec<Grant>;

pub type TargetGrants = Vec<TargetGrant>;

/// Parse `Grants` from XML
pub struct GrantsParser;

/// Write `Grants` contents to a `SignedRequest`
pub struct GrantsWriter;

/// Parse `GrantReadACP` from XML
pub struct GrantReadACPParser;

/// Write `GrantReadACP` contents to a `SignedRequest`
pub struct GrantReadACPWriter;

/// Parse `Grant` from XML
pub struct GrantParser;

/// Write `Grant` contents to a `SignedRequest`
pub struct GrantWriter;

/// Parse `Grantee` from XML
pub struct GranteeParser;

/// Write `Grantee` contents to a `SignedRequest`
pub struct GranteeWriter;

/// Parse `GrantRead` from XML
pub struct GrantReadParser;

/// Write `GrantRead` contents to a `SignedRequest`
pub struct GrantReadWriter;

/// Parse `GrantWriteACP` from XML
pub struct GrantWriteACPParser;

/// Write `GrantWriteACP` contents to a `SignedRequest`
pub struct GrantWriteACPWriter;

/// Parse `GrantWrite` from XML
pub struct GrantWriteParser;

/// Write `GrantWrite` contents to a `SignedRequest`
pub struct GrantWriteWriter;

/// Parse `GrantFullControl` from XML
pub struct GrantFullControlParser;

/// Write `GrantFullControl` contents to a `SignedRequest`
pub struct GrantFullControlWriter;

/// Parse `TargetGrant` from XML
pub struct TargetGrantParser;

/// Write `TargetGrant` contents to a `SignedRequest`
pub struct TargetGrantWriter;

// Impls below...


impl GrantsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Grants, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Grant" {
            obj.push(try!(GrantParser::parse_xml("Grant", stack)));
        }
        Ok(obj)
    }
}

impl GrantsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Grants) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            GrantWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

impl GrantReadACPParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantReadACP, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl GrantReadACPWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GrantReadACP) {
        params.put(name, obj);
    }
}

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

impl GrantWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Grant) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        GranteeWriter::write_params(params, &(prefix.to_string() + "Grantee"), &obj.grantee);
        PermissionWriter::write_params(params, &(prefix.to_string() + "Permission"), &obj.permission);
    }
}

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

impl GrantReadParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantRead, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl GrantReadWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GrantRead) {
        params.put(name, obj);
    }
}


impl GrantWriteACPParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantWriteACP, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl GrantWriteACPWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GrantWriteACP) {
        params.put(name, obj);
    }
}


impl GrantWriteParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantWrite, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl GrantWriteWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GrantWrite) {
        params.put(name, obj);
    }
}

impl GrantFullControlParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GrantFullControl, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl GrantFullControlWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GrantFullControl) {
        params.put(name, obj);
    }
}

impl TargetGrantParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TargetGrant, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = TargetGrant::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Grantee" {
                obj.grantee = try!(GranteeParser::parse_xml("Grantee", stack));
                continue;
            }
            if current_name == "Permission" {
                obj.permission = try!(BucketLogsPermissionParser::parse_xml("Permission", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl TargetGrantWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &TargetGrant) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        GranteeWriter::write_params(params, &(prefix.to_string() + "Grantee"), &obj.grantee);
        BucketLogsPermissionWriter::write_params(params, &(prefix.to_string() + "Permission"), &obj.permission);
    }
}
