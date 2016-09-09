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

//! Library Documentation
//!
//! Used for documenting the library for internal use. Not intended for client documentation.
//! `common` contains the type, struct, enum and impls that are common accross most requests
//! such as buckets, objects etc.

use std::str::FromStr;
use std::str;

use aws::common::xmlutil::*;
use aws::common::params::*;

pub type Body = Vec<u8>;

pub type MaxKeys = i32;

pub type KeyCount = i32;

pub type URI = String;

pub type ID = String;

pub type Type = String;

pub type EmailAddress = String;

pub type DisplayName = String;

pub type Code = String;

pub type HostId = String;

pub type RequestId = String;

pub type Resource = String;

pub type Value = String;

pub type SkipElement = String;

pub type IsTruncated = bool;

pub type StartAfter = String;

pub type ContinuationToken = String;

/// Parse `Body` from XML
pub struct BodyParser;

/// Write `Body` contents to a `SignedRequest`
pub struct BodyWriter;

/// Parse `MaxKeys` from XML
pub struct MaxKeysParser;

/// Write `MaxKeys` contents to a `SignedRequest`
pub struct MaxKeysWriter;

/// Parse `KeyCount` from XML
pub struct KeyCountParser;

/// Write `KeyCount` contents to a `SignedRequest`
pub struct KeyCountWriter;

/// Parse `URI` from XML
pub struct URIParser;

/// Write `URI` contents to a `SignedRequest`
pub struct URIWriter;

/// Parse `ID` from XML
pub struct IDParser;

/// Write `ID` contents to a `SignedRequest`
pub struct IDWriter;

/// Parse `Type` from XML
pub struct TypeParser;

/// Write `Type` contents to a `SignedRequest`
pub struct TypeWriter;

/// Parse `EmailAddress` from XML
pub struct EmailAddressParser;

/// Write `EmailAddress` contents to a `SignedRequest`
pub struct EmailAddressWriter;

/// Parse `DisplayName` from XML
pub struct DisplayNameParser;

/// Write `DisplayName` contents to a `SignedRequest`
pub struct DisplayNameWriter;

/// Parse `Owner` from XML
pub struct OwnerParser;

/// Write `Owner` contents to a `SignedRequest`
pub struct OwnerWriter;

/// Parse `Code` from XML
pub struct CodeParser;

/// Write `Code` contents to a `SignedRequest`
pub struct CodeWriter;

/// Parse `HostId` from XML
pub struct HostIdParser;

/// Write `HostId` contents to a `SignedRequest`
pub struct HostIdWriter;

/// Parse `RequestId` from XML
pub struct RequestIdParser;

/// Write `RequestId` contents to a `SignedRequest`
pub struct RequestIdWriter;

/// Parse `Resource` from XML
pub struct ResourceParser;

/// Write `Resource` contents to a `SignedRequest`
pub struct ResourceWriter;

/// Parse `Value` from XML
pub struct ValueParser;

/// Write `Value` contents to a `SignedRequest`
pub struct ValueWriter;

/// Parse `SkipElement` from XML
pub struct SkipElementParser;

/// Parse `IsTruncated` from XML
pub struct IsTruncatedParser;

/// Write `IsTruncated` contents to a `SignedRequest`
pub struct IsTruncatedWriter;

/// Parse `ContinuationToken` from XML
pub struct ContinuationTokenParser;

/// Write `ContinuationToken` contents to a `SignedRequest`
pub struct ContinuationTokenWriter;

/// Parse `StartAfter` from XML
pub struct StartAfterParser;

/// Write `StartAfter` contents to a `SignedRequest`
pub struct StartAfterWriter;

/// Owner
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct Owner {
    pub display_name: DisplayName,
    pub id: ID,
}

// Impls below...

impl BodyParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Body, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack)).into_bytes();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl BodyWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Body) {
        params.put(name, str::from_utf8(obj).unwrap());
    }
}

impl MaxKeysParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MaxKeys, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl MaxKeysWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MaxKeys) {
        params.put(name, &obj.to_string());
    }
}

impl KeyCountParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<KeyCount, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl KeyCountWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &KeyCount) {
        params.put(name, &obj.to_string());
    }
}

impl URIParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<URI, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl URIWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &URI) {
        params.put(name, obj);
    }
}

impl IDParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ID, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl IDWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ID) {
        params.put(name, obj);
    }
}

impl TypeParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Type, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl TypeWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Type) {
        params.put(name, obj);
    }
}

impl EmailAddressParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<EmailAddress, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl EmailAddressWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &EmailAddress) {
        params.put(name, obj);
    }
}

impl DisplayNameParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DisplayName, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl DisplayNameWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DisplayName) {
        params.put(name, obj);
    }
}

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

impl OwnerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Owner) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DisplayNameWriter::write_params(params, &(prefix.to_string() + "DisplayName"), &obj.display_name);
        IDWriter::write_params(params, &(prefix.to_string() + "ID"), &obj.id);
    }
}

impl CodeParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Code, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl CodeWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Code) {
        params.put(name, obj);
    }
}

impl HostIdParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<HostId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl HostIdWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Code) {
        params.put(name, obj);
    }
}

impl RequestIdParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RequestId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl RequestIdWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Code) {
        params.put(name, obj);
    }
}

impl ResourceParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Resource, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ResourceWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Code) {
        params.put(name, obj);
    }
}

impl ValueParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Value, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ValueWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Value) {
        params.put(name, obj);
    }
}

// NOTE: SkipElement does not walk a tree, it only pulls the value.
impl SkipElementParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SkipElement, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(format!("<{}>{}</{}>", tag_name, obj, tag_name))
    }
}

impl IsTruncatedParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<IsTruncated, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = IsTruncated::default();

        match characters(stack) {
            Err(_) => return Ok(obj),
            Ok(ref chars) => obj = bool::from_str(chars).unwrap(),
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl IsTruncatedWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &IsTruncated) {
        params.put(name, &obj.to_string());
    }
}

impl ContinuationTokenParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContinuationToken, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ContinuationTokenWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ContinuationToken) {
        params.put(name, obj);
    }
}

impl StartAfterParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<StartAfter, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl StartAfterWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &StartAfter) {
        params.put(name, obj);
    }
}
