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

use std::num::ParseIntError;
use std::str::ParseBoolError;
use std::error::Error;
use std::fmt;

use aws::errors::credentials_error::CredentialsError;
use aws::common::xmlutil::XmlParseError;

// S3 Specific Errors
#[derive(Debug)]
pub struct S3Error {
    pub message: String
}

impl S3Error {
    pub fn new<S>(message: S) -> S3Error where S: Into<String> {
        S3Error { message: message.into() }
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
        S3Error { message: err.description().to_owned() }
    }
}

impl From<ParseIntError> for S3Error {
    fn from(err: ParseIntError) -> S3Error {
        S3Error { message: err.description().to_owned() }
    }
}

impl From<ParseBoolError> for S3Error {
    fn from(err: ParseBoolError) -> S3Error {
        S3Error { message: err.description().to_owned() }
    }
}

impl From<XmlParseError> for S3Error {
    fn from(err: XmlParseError) -> S3Error {
        let XmlParseError(message) = err;
        S3Error { message: message.to_owned() }
    }
}
