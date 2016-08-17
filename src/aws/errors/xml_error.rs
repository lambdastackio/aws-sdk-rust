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

use aws::common::xmlutil::{XmlParseError, Peek, Next};
use aws::common::xmlutil::{characters, start_element, end_element, string_field, peek_at_name};
use aws::common::params::{Params, ServiceParams};

#[derive(Default, Debug)]
pub struct XmlError {
	pub error_type: String,
	pub code: String,
	pub message: String,
	pub detail: Option<String>
}

pub struct XmlErrorDeserializer;
impl XmlErrorDeserializer {
	pub fn deserialize<'a, T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<XmlError, XmlParseError> {
		try!(start_element(tag_name, stack));

		let mut obj = XmlError::default();

		loop {
			match &try!(peek_at_name(stack))[..] {
				"Type" => {
					obj.error_type = try!(string_field("Type", stack));
					continue;
				},
				"Code" => {
					obj.code = try!(string_field("Code", stack));
					continue;
				},
				"Message" => {
					obj.message = try!(string_field("Message", stack));
					continue;
				},
				"Detail" => {
					try!(start_element("Detail", stack));
					if let Ok(characters) = characters(stack){
						obj.detail = Some(characters.to_string());
						try!(end_element("Detail", stack));
					}
					continue;
				},
				_ => break
			}
		}

		try!(end_element(tag_name, stack));

		Ok(obj)
	}
}

pub type Errors = Vec<String>;

/// Parse `Errors` from XML
#[allow(dead_code)]
struct ErrorsParser;

impl ErrorsParser {
	#[allow(dead_code)]
	#[allow(unused_variables)]
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Errors, XmlParseError> {
        let mut obj : Vec<String> = Vec::new();
        while try!(peek_at_name(stack)) == "Error" {
            obj.push(try!(ErrorsParser::parse_single_error(stack)));
        }
        Ok(obj)
    }

    fn parse_single_error<T: Peek + Next>(stack: &mut T) -> Result<String, XmlParseError> {
        // TODO: go back to try!

        match characters(stack) {
            Err(why) => Err(why),
            Ok(val) => Ok(val),
        }
    }
}

/// Write `Errors` contents to a `SignedRequest`
#[allow(dead_code)]
struct ErrorsWriter;

impl ErrorsWriter {
	#[allow(dead_code)]
    fn write_params(params: &mut Params, name: &str, obj: &Errors) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            ErrorsWriter::write_param(params, key, element);
            index += 1;
        }
    }

	#[allow(dead_code)]
    fn write_param(params: &mut Params, key: &str, value: &str) {
        params.put(key, value);
    }
}
