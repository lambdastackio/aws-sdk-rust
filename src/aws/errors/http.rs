/*
 Copyright 2017 LambdaStack All rights reserved.

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

use aws::common::params::*;
use aws::common::xmlutil::*;

pub type HttpErrorCodeReturnedEquals = String;

pub type CreationDate = String;

pub type HttpRedirectCode = String;


/// Parse `HttpErrorCodeReturnedEquals` from XML
pub struct HttpErrorCodeReturnedEqualsParser;

/// Write `HttpErrorCodeReturnedEquals` contents to a `SignedRequest`
pub struct HttpErrorCodeReturnedEqualsWriter;

/// Parse `CreationDate` from XML
pub struct CreationDateParser;

/// Write `CreationDate` contents to a `SignedRequest`
pub struct CreationDateWriter;

/// Parse `HttpRedirectCode` from XML
pub struct HttpRedirectCodeParser;

/// Write `HttpRedirectCode` contents to a `SignedRequest`
pub struct HttpRedirectCodeWriter;

// Impls below...

impl HttpErrorCodeReturnedEqualsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<HttpErrorCodeReturnedEquals, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl HttpErrorCodeReturnedEqualsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &HttpErrorCodeReturnedEquals) {
        params.put(name, obj);
    }
}

/*---------------------------------*/

impl CreationDateParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CreationDate, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl CreationDateWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CreationDate) {
        params.put(name, obj);
    }
}

/*---------------------------------*/

impl HttpRedirectCodeParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<HttpRedirectCode, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl HttpRedirectCodeWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &HttpRedirectCode) {
        params.put(name, obj);
    }
}

/*---------------------------------*/
