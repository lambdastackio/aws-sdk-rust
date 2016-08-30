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

//use aws::errors::creds::CredentialsError;
use aws::common::xmlutil::*;
use aws::common::common::*;
use aws::s3::header::*;
use aws::s3::writeparse::*;

/// AWSError - Default XML error returned from AWS S3.
///
/// AWS returns a core error XML structure plus additional elements based on the type of request.
/// The ```expanded_message``` captures those additional elements if you need them.
///
#[derive(Debug, Default, Clone)]
pub struct AWSError {
    /// code is an alphanumeric value that in some cases can be a number or a short
    /// description of the issue with no spaces.
    pub code: String,
    /// host_id is the base64 encoded AWS HostId of the node that received the request.
    /// This can help AWS track down issues for a given host.
    pub host_id: String,
    /// message is a full description of the error for humans.
    pub message: String,
    /// request_id is the unique ID generated for the given request coming into AWS.
    /// This value is generated on every request coming into AWS to aid in tracking
    /// down issues.
    pub request_id: String,
    /// resource identifies the type of AWS resource.
    pub resource: String,
    /// missing_header_name is added to the default AWSError because it's important for
    /// trapping errors where header items have not been added correctly such as acl etc.
    pub missing_header_name: String,
    /// expanded_message contains the details of a given error message from AWS.
    pub expanded_message: String,
}

impl fmt::Display for AWSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error code: {} - {}", self.code, self.description())
    }
}

/// Error is implemented for AWSError for basic Error features.
impl Error for AWSError {
    fn description(&self) -> &str {
        &self.message
    }
}

/// AWSError is the generic error XML block that AWS returns on all requests that result in some
/// sort of error.
///
/// There are basic core items that return on all AWS errors plus elements that are unique to the
/// given request. For example, if the signature is incorrect (V2 or V4) for some reason then
/// the core fields will be filled in and then the ```expanded_message``` field will be updated
/// with the remaining elements that can help you track down the real issue.
///
/// NB: AWSError is usually embedded as part of S3Error. S3Error.message will contain the
/// general error message for the given request while AWSError shows the exact AWS error.
///
impl AWSError {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AWSError, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = AWSError::default();
        let mut exp_msg: String = String::new();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "HostId" {
                obj.host_id = try!(HostIdParser::parse_xml("HostId", stack));
                continue;
            }
            if current_name == "RequestId" {
                obj.request_id = try!(RequestIdParser::parse_xml("RequestId", stack));
                continue;
            }
            if current_name == "Code" {
                obj.code =  try!(CodeParser::parse_xml("Code", stack));
                continue;
            }
            if current_name == "Message" {
                obj.message = try!(S3ClientMessageParser::parse_xml("Message", stack));
                continue;
            }
            if current_name == "Resource" {
                obj.resource = try!(ResourceParser::parse_xml("Resource", stack));
                continue;
            }
            if current_name == "MissingHeaderName" {
                obj.missing_header_name = try!(MissingHeaderNameParser::parse_xml("MissingHeaderName", stack));
                continue;
            }
            if current_name == tag_name || current_name.len() == 0 {
                break;
            }
            // No reason for listing parse error...
            exp_msg += &SkipElementParser::parse_xml(&current_name, stack).unwrap_or("".to_string());
        }
        // end_element_skip instead of end_element is used because AWSError changes based on the
        // type of error. However, certain portions are always there and those are the ones we're
        // interested so capture those and skip to the end.
        if exp_msg.len() > 0 {
            obj.expanded_message = exp_msg;
        }
        try!(end_element_skip(tag_name, stack));
        Ok(obj)
    }
}
