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

#[derive(Debug, Default)]
pub struct LifecycleExpiration {
    /// Indicates at what date the object is to be moved or deleted. Should be in GMT
    /// ISO 8601 Format.
    pub date: Date,
    /// Indicates the lifetime, in days, of the objects that are subject to the rule.
    /// The value must be a non-zero positive integer.
    pub days: Days,
}

/// Parse `LifecycleExpiration` from XML
struct LifecycleExpirationParser;

impl LifecycleExpirationParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LifecycleExpiration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = LifecycleExpiration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            match current_name.as_ref() {
                "Date" => {
                    obj.date = try!(DateParser::parse_xml("Date", stack));
                    continue;
                },
                "Days" => {
                    obj.days = try!(DaysParser::parse_xml("Days", stack));
                    continue;
                },
                _ => break,
            }
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `LifecycleExpiration` contents to a `SignedRequest`
struct LifecycleExpirationWriter;

impl LifecycleExpirationWriter {
    fn write_params(params: &mut Params, name: &str, obj: &LifecycleExpiration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DateWriter::write_params(params, &(prefix.to_string() + "Date"), &obj.date);
        DaysWriter::write_params(params, &(prefix.to_string() + "Days"), &obj.days);
    }
}
