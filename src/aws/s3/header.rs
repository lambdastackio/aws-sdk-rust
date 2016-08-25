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
use aws::s3::writeparse::*;

pub type ContentRange = String;

pub type ContentType = String;

pub type ContentMD5 = String;

pub type HostName = String;

pub type Protocol = String;

pub type ETag = String;

pub type AllowedHeader = String;

pub type AllowedHeaders = Vec<AllowedHeader>;

pub type Range = String;

pub type AcceptRanges = String;

/// Parse `ContentRange` from XML
pub struct ContentRangeParser;

/// Write `ContentRange` contents to a `SignedRequest`
pub struct ContentRangeWriter;

/// Parse `ContentType` from XML
pub struct ContentTypeParser;

/// Write `ContentType` contents to a `SignedRequest`
pub struct ContentTypeWriter;

/// Parse `ContentMD`5 from XML
pub struct ContentMD5Parser;

/// Write `ContentMD5` contents to a `SignedRequest`
pub struct ContentMD5Writer;

/// Parse `HostName` from XML
pub struct HostNameParser;

/// Write `HostName` contents to a `SignedRequest`
pub struct HostNameWriter;

/// Parse `Protocol` from XML
pub struct ProtocolParser;

/// Write `Protocol` contents to a `SignedRequest`
pub struct ProtocolWriter;

/// Parse `ETag` from XML
pub struct ETagParser;

/// Write `ETag` contents to a `SignedRequest`
pub struct ETagWriter;

/// Parse `AllowedHeader` from XML
pub struct AllowedHeaderParser;

/// Write `AllowedHeader` contents to a `SignedRequest`
pub struct AllowedHeaderWriter;

/// Parse `AllowedHeaders` from XML
pub struct AllowedHeadersParser;

/// Write `AllowedHeaders` contents to a `SignedRequest`
pub struct AllowedHeadersWriter;

/// Parse `Range` from XML
pub struct RangeParser;

/// Write `Range` contents to a `SignedRequest`
pub struct RangeWriter;

/// Parse `AcceptRanges` from XML
pub struct AcceptRangesParser;

/// Write `AcceptRanges` contents to a `SignedRequest`
pub struct AcceptRangesWriter;

// Impls below...

impl ContentRangeParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentRange, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ContentRangeWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ContentRange) {
        params.put(name, obj);
    }
}

impl ContentTypeParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentType, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ContentTypeWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ContentType) {
        params.put(name, obj);
    }
}

impl ContentMD5Parser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentMD5, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ContentMD5Writer {
    pub fn write_params(params: &mut Params, name: &str, obj: &ContentMD5) {
        params.put(name, obj);
    }
}

impl HostNameParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<HostName, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl HostNameWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &HostName) {
        params.put(name, obj);
    }
}

impl ProtocolParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Protocol, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ProtocolWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Protocol) {
        params.put(name, obj);
    }
}

impl ETagParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ETag, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ETagWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ETag) {
        params.put(name, obj);
    }
}

impl AllowedHeaderParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AllowedHeader, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl AllowedHeaderWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &AllowedHeader) {
        params.put(name, obj);
    }
}

impl AllowedHeadersParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AllowedHeaders, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "AllowedHeader" {
            obj.push(try!(AllowedHeaderParser::parse_xml("AllowedHeader", stack)));
        }
        Ok(obj)
    }
}

impl AllowedHeadersWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &AllowedHeaders) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            AllowedHeaderWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

impl RangeParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Range, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl RangeWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Range) {
        params.put(name, obj);
    }
}

impl AcceptRangesParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AcceptRanges, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl AcceptRangesWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &AcceptRanges) {
        params.put(name, obj);
    }
}
