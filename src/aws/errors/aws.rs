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

use std::error::Error;
use std::fmt;

use aws::errors::creds::CredentialsError;
use aws::common::xmlutil::*;

/// AWSError - Default XML error returned from AWS S3.
///
#[derive(Debug, Default, Clone)]
pub struct AWSError {
    pub code: String,
    pub message: String,
    pub request_id: String,
    pub resource: String,
}

impl fmt::Display for AWSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error code: {} - {}", self.code, self.description())
    }
}

impl Error for AWSError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl AWSError {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AWSError, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = AWSError::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "RequestId" {
                obj.request_id = try!(characters(stack));
                continue;
            }
            if current_name == "Code" {
                obj.code = try!(characters(stack));
                continue;
            }
            if current_name == "Message" {
                obj.message = try!(characters(stack));
                continue;
            }
            if current_name == "Resource" {
                obj.resource = try!(characters(stack));
                continue;
            }
            // Cycle through until end tag - Ignore other dynamically added messages.
            if current_name == tag_name {
                break;
            }
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }

}
