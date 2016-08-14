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

#[derive(Debug, Default)]
pub struct PutBucketNotificationRequest {
    pub notification_configuration: NotificationConfigurationDeprecated,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
}

/// Parse `PutBucketNotificationRequest` from XML
struct PutBucketNotificationRequestParser;

impl PutBucketNotificationRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketNotificationRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketNotificationRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            match current_name.as_ref() {
                "NotificationConfiguration" => {
                    obj.notification_configuration = try!(NotificationConfigurationDeprecatedParser::parse_xml("NotificationConfiguration", stack));
                    continue;
                },
                "Content-MD5" => {
                    obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                    continue;
                },
                "Bucket" => {
                    obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                    continue;
                },
                _ => break,
            }
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `PutBucketNotificationRequest` contents to a `SignedRequest`
struct PutBucketNotificationRequestWriter;

impl PutBucketNotificationRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &PutBucketNotificationRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        NotificationConfigurationDeprecatedWriter::write_params(params, &(prefix.to_string() + "NotificationConfiguration"), &obj.notification_configuration);
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }

        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}
