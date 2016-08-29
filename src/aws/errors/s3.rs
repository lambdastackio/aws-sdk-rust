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

use std::num::ParseIntError;
use std::str::ParseBoolError;
use std::error::Error;
use std::fmt;

use aws::errors::aws::*;
use aws::errors::creds::CredentialsError;
use aws::common::xmlutil::*;
use aws::common::params::*;
use aws::common::common::*;
use aws::s3::writeparse::*;
//use aws::s3::object::*;

// S3 Specific Errors
#[derive(Debug)]
pub struct S3Error {
    pub message: String,
    pub aws: AWSError
}

#[derive(Debug, Default)]
pub struct S3ClientError {
    pub version_id: ObjectVersionId,
    pub code: Code,
    pub message: S3ClientMessage,
    pub key: ObjectKey,
}

/// Parse `S`3ClientError from XML
pub struct S3ClientErrorParser;

/// Write `S3ClientError` contents to a `SignedRequest`
pub struct S3ClientErrorWriter;


// Impls below...

impl S3Error {
    pub fn new<S>(message: S) -> S3Error where S: Into<String> {
        S3Error { message: message.into(), aws: AWSError::default() }
    }

    pub fn with_aws<S>(message: S, aws: AWSError) -> S3Error where S: Into<String> {
        S3Error { message: message.into(), aws: aws }
    }
}

impl fmt::Display for S3Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for S3Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<CredentialsError> for S3Error {
    fn from(err: CredentialsError) -> S3Error {
        S3Error { message: err.description().to_owned(), aws: AWSError::default() }
    }
}

impl From<ParseIntError> for S3Error {
    fn from(err: ParseIntError) -> S3Error {
        S3Error { message: err.description().to_owned(), aws: AWSError::default() }
    }
}

impl From<ParseBoolError> for S3Error {
    fn from(err: ParseBoolError) -> S3Error {
        S3Error { message: err.description().to_owned(), aws: AWSError::default() }
    }
}

impl From<XmlParseError> for S3Error {
    fn from(err: XmlParseError) -> S3Error {
        let XmlParseError(message) = err;
        S3Error { message: message.to_owned(), aws: AWSError::default() }
    }
}

impl S3ClientErrorParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<S3ClientError, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = S3ClientError::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "VersionId" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("VersionId", stack));
                continue;
            }
            if current_name == "Code" {
                obj.code = try!(CodeParser::parse_xml("Code", stack));
                continue;
            }
            if current_name == "Message" {
                obj.message = try!(S3ClientMessageParser::parse_xml("Message", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl S3ClientErrorWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &S3ClientError) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
        CodeWriter::write_params(params, &(prefix.to_string() + "Code"), &obj.code);
        S3ClientMessageWriter::write_params(params, &(prefix.to_string() + "Message"), &obj.message);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}
