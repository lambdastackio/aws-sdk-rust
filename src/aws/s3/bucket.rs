// Copyright 2016 LambdaStack All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// Portions borrowed from the rusoto project. See README.md
//

#![allow(unused_variables)]
#![allow(unused_mut)]

use aws::common::region::Region;
use aws::common::params::{Params, ServiceParams};
use aws::common::xmlutil::*;
// use aws::common::common::*;
use aws::errors::http::*;
use aws::s3::writeparse::*;
use aws::s3::object::*;
use aws::s3::policy::*;
use aws::s3::acl::*;
use aws::s3::header::*;
use aws::s3::grant::*;

pub type BucketName = String;

pub type Buckets = Vec<Bucket>;

pub type CloudFunctionConfigurationList = Vec<CloudFunctionConfiguration>;

pub type TopicConfigurationList = Vec<TopicConfiguration>;

pub type BucketLocationConstraint = String;

pub type TargetBucket = String;

pub type TargetPrefix = String;

/// Parse `BucketName` from XML
pub struct BucketNameParser;

/// Parse `Bucket` from XML
pub struct BucketParser;

/// Write `BucketName` contents to a `SignedRequest`
pub struct BucketNameWriter;

/// Write `Bucket` contents to a `SignedRequest`
pub struct BucketWriter;

/// Parse `Buckets` from XML
pub struct BucketsParser;

/// Write `Buckets` contents to a `SignedRequest`
pub struct BucketsWriter;

/// Parse `CloudFunctionConfigurationList` from XML
pub struct CloudFunctionConfigurationListParser;

/// Write `CloudFunctionConfigurationList` contents to a `SignedRequest`
pub struct CloudFunctionConfigurationListWriter;

/// Parse `TopicConfigurationList` from XML
pub struct TopicConfigurationListParser;

/// Write `TopicConfigurationList` contents to a `SignedRequest`
pub struct TopicConfigurationListWriter;

/// Parse `TopicConfiguration` from XML
pub struct TopicConfigurationParser;

/// Write `TopicConfiguration` contents to a `SignedRequest`
pub struct TopicConfigurationWriter;

/// Parse `NotificationConfiguration` from XML
pub struct NotificationConfigurationParser;

/// Write `NotificationConfiguration` contents to a `SignedRequest`
pub struct NotificationConfigurationWriter;

/// The requested bucket name is not available. The bucket namespace is shared by
/// all users of the system. Please select a different name and try again.
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct BucketAlreadyExists;

/// Parse `BucketAlreadyExists` from XML
pub struct BucketAlreadyExistsParser;

/// Write `BucketAlreadyExists` contents to a `SignedRequest`
pub struct BucketAlreadyExistsWriter;

/// Parse `BucketLocationConstraint` from XML
pub struct BucketLocationConstraintParser;

/// Write `BucketLocationConstraint` contents to a `SignedRequest`
pub struct BucketLocationConstraintWriter;

/// Parse `HeadBucketRequest` from XML
pub struct HeadBucketRequestParser;

/// Write `HeadBucketRequest` contents to a `SignedRequest`
pub struct HeadBucketRequestWriter;

/// Parse `TargetBucket` from XML
pub struct TargetBucketParser;

/// Write `TargetBucket` contents to a `SignedRequest`
pub struct TargetBucketWriter;

/// Parse `TargetPrefix` from XML
pub struct TargetPrefixParser;

/// Write `TargetPrefix` contents to a `SignedRequest`
pub struct TargetPrefixWriter;

/// Parse `TargetGrants` from XML
pub struct TargetGrantsParser;

/// Write `TargetGrants` contents to a `SignedRequest`
pub struct TargetGrantsWriter;

/// Parse `LoggingEnabled` from XML
pub struct LoggingEnabledParser;

/// Write `LoggingEnabled` contents to a `SignedRequest`
pub struct LoggingEnabledWriter;


//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct TopicConfigurationDeprecated {
    /// Amazon SNS topic to which Amazon S3 will publish a message to report the
    /// specified events for the bucket.
    pub topic: TopicArn,
    pub id: NotificationId,
    /// Bucket event for which to send notifications.
    pub event: Event,
    pub events: EventList,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct QueueConfigurationDeprecated {
    pub queue: QueueArn,
    pub events: EventList,
    pub id: NotificationId,
    pub event: Event,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct CloudFunctionConfiguration {
    pub invocation_role: CloudFunctionInvocationRole,
    pub cloud_function: CloudFunction,
    pub events: EventList,
    pub id: NotificationId,
    pub event: Event,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct PutBucketReplicationRequest {
    pub replication_configuration: ReplicationConfiguration,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct PutBucketNotificationRequest {
    pub notification_configuration: NotificationConfigurationDeprecated,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GetBucketLoggingRequest {
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct PutBucketLifecycleRequest {
    pub lifecycle_configuration: Option<LifecycleConfiguration>,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GetBucketCorsRequest {
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GetBucketCorsOutput {
    pub cors_rules: CORSRules,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GetBucketVersioningRequest {
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GetBucketVersioningOutput {
    /// The versioning state of the bucket.
    pub status: BucketVersioningStatus,
    /// Specifies whether MFA delete is enabled in the bucket versioning
    /// configuration. This element is only returned if the bucket has been configured
    /// with MFA delete. If the bucket has never been so configured, this element is
    /// not returned.
    pub mfa_delete: MFADeleteStatus,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct PutBucketVersioningRequest {
    /// The concatenation of the authentication device's serial number, a space, and
    /// the value that is displayed on your authentication device.
    pub mfa: Option<MFA>,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
    pub versioning_configuration: VersioningConfiguration,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct CORSRule {
    /// Specifies which headers are allowed in a pre-flight OPTIONS request.
    pub allowed_headers: AllowedHeaders,
    /// One or more headers in the response that you want customers to be able to
    /// access from their applications (for example, from a JavaScript XMLHttpRequest
    /// object).
    pub expose_headers: ExposeHeaders,
    /// Identifies HTTP methods that the domain/origin specified in the rule is
    /// allowed to execute.
    pub allowed_methods: AllowedMethods,
    /// The time in seconds that your browser is to cache the preflight response for
    /// the specified resource.
    pub max_age_seconds: MaxAgeSeconds,
    /// One or more origins you want customers to be able to access the bucket from.
    pub allowed_origins: AllowedOrigins,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct VersioningConfiguration {
    /// The versioning state of the bucket.
    pub status: BucketVersioningStatus,
    /// Specifies whether MFA delete is enabled in the bucket versioning
    /// configuration. This element is only returned if the bucket has been configured
    /// with MFA delete. If the bucket has never been so configured, this element is
    /// not returned.
    pub mfa_delete: MFADelete,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteBucketCorsRequest {
    pub bucket: BucketName,
}

/// Container for specifying the notification configuration of the bucket. If this
/// element is empty, notifications are turned off on the bucket.
//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct NotificationConfiguration {
    pub queue_configurations: QueueConfigurationList,
    pub lambda_function_configurations: LambdaFunctionConfigurationList,
    pub topic_configurations: TopicConfigurationList,
    pub cloud_function_configurations: CloudFunctionConfigurationList,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GetBucketNotificationConfigurationRequest {
    /// Name of the buket to get the notification configuration for.
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteBucketWebsiteRequest {
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteBucketTaggingRequest {
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct LifecycleConfiguration {
    pub rules: Rules,
}

/// Put bucket policy request
//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct PutBucketPolicyRequest {
    /// The bucket policy as a JSON document.
    pub policy: Policy,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct RoutingRule {
    /// Container for redirect information. You can redirect requests to another host,
    /// to another page, or with another protocol. In the event of an error, you can
    /// can specify a different error code to return.
    pub redirect: Redirect,
    /// A container for describing a condition that must be met for the specified
    /// redirect to apply. For example, 1. If request is for pages in the /docs
    /// folder, redirect to the /documents folder. 2. If request results in HTTP error
    /// 4xx, redirect request to another host where you might process the error.
    pub condition: Option<Condition>,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct CreateBucketRequest {
    /// Allows grantee the read, write, read ACP, and write ACP permissions on the
    /// bucket.
    pub grant_full_control: Option<GrantFullControl>,
    pub create_bucket_configuration: Option<CreateBucketConfiguration>,
    /// Allows grantee to write the ACL for the applicable bucket.
    pub grant_write_acp: Option<GrantWriteACP>,
    pub bucket: BucketName,
    /// The canned ACL to apply to the bucket.
    pub acl: Option<CannedAcl>,
    /// Allows grantee to create, overwrite, and delete any object in the bucket.
    pub grant_write: Option<GrantWrite>,
    /// Allows grantee to list the objects in the bucket.
    pub grant_read: Option<GrantRead>,
    /// Allows grantee to read the bucket ACL.
    pub grant_read_acp: Option<GrantReadACP>,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct PutBucketLoggingRequest {
    pub bucket_logging_status: BucketLoggingStatus,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
}

/// Container for replication rules. You can add as many as 1,000 rules. Total
/// replication configuration size can be up to 2 MB.
//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ReplicationConfiguration {
    /// Container for information about a particular replication rule. Replication
    /// configuration must have at least one rule and can contain up to 1,000 rules.
    pub rules: ReplicationRules,
    /// Amazon Resource Name (ARN) of an IAM role for Amazon S3 to assume when
    /// replicating the objects.
    pub role: Role,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct LoggingEnabled {
    /// This element lets you specify a prefix for the keys that the log files will be
    /// stored under.
    pub target_prefix: TargetPrefix,
    /// Specifies the bucket where you want Amazon S3 to store server access logs. You
    /// can have your logs delivered to any bucket that you own, including the same
    /// bucket that is being logged. You can also configure multiple buckets to
    /// deliver their logs to the same target bucket. In this case you should choose a
    /// different TargetPrefix for each source bucket so that the delivered log files
    /// can be distinguished by key.
    pub target_bucket: TargetBucket,
    pub target_grants: TargetGrants,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct WebsiteConfiguration {
    pub redirect_all_requests_to: RedirectAllRequestsTo,
    pub index_document: IndexDocument,
    pub error_document: ErrorDocument,
    pub routing_rules: RoutingRules,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct BucketLoggingStatus {
    pub logging_enabled: LoggingEnabled,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct Destination {
    /// Amazon resource name (ARN) of the bucket where you want Amazon S3 to store
    /// replicas of the object identified by the rule.
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct PutBucketRequestPaymentRequest {
    pub request_payment_configuration: RequestPaymentConfiguration,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct Bucket {
    /// Date the bucket was created.
    pub creation_date: CreationDate,
    /// The name of the bucket.
    pub name: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct HeadBucketRequest {
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteBucketRequest {
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteBucketPolicyRequest {
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteBucketReplicationRequest {
    pub bucket: BucketName,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GetBucketLoggingOutput {
    pub logging_enabled: LoggingEnabled,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GetBucketReplicationOutput {
    pub replication_configuration: ReplicationConfiguration,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct PutBucketWebsiteRequest {
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
    pub website_configuration: WebsiteConfiguration,
}

// Impls and functions below...

/// Helper function to determine if a create config is needed.
pub fn needs_create_bucket_config(region: Region) -> bool {
    match region {
        Region::UsEast1 => false,
        _ => true,
    }
}

// This is a bit hacky to get functionality until we figure out an XML writing util.
/// Manually writes out bucket configuration (location constraint) in XML.
pub fn create_bucket_config_xml(region: Region) -> Vec<u8> {
    match region {
        Region::UsEast1 => {
            Vec::new() // shouldn't actually execute this: panic! or unreachable! this?
        },
        _ => {
            let xml = format!(
                "<CreateBucketConfiguration xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">
                    <LocationConstraint>{}</LocationConstraint>
                </CreateBucketConfiguration >", region);
            xml.into_bytes()
        },
    }
}

impl BucketNameParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketName, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl BucketParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Bucket, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Bucket::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "CreationDate" {
                obj.creation_date = try!(CreationDateParser::parse_xml("CreationDate", stack));
                continue;
            }
            if current_name == "Name" {
                obj.name = try!(BucketNameParser::parse_xml("Name", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl BucketNameWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &BucketName) {
        params.put(name, obj);
    }
}

impl BucketWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Bucket) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        CreationDateWriter::write_params(params, &(prefix.to_string() + "CreationDate"), &obj.creation_date);
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
    }
}

impl BucketsParser {
    #[allow(unused_variables)]
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Buckets, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Bucket" {
            obj.push(try!(BucketParser::parse_xml("Bucket", stack)));
        }
        Ok(obj)
    }
}

impl BucketsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Buckets) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            BucketWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

impl CloudFunctionConfigurationListParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CloudFunctionConfigurationList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "CloudFunctionConfiguration" {
            obj.push(try!(CloudFunctionConfigurationParser::parse_xml("CloudFunctionConfiguration", stack)));
        }
        Ok(obj)
    }
}

impl CloudFunctionConfigurationListWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CloudFunctionConfigurationList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            CloudFunctionConfigurationWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

impl TopicConfigurationListParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TopicConfigurationList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "TopicConfiguration" {
            obj.push(try!(TopicConfigurationParser::parse_xml("TopicConfiguration", stack)));
        }
        Ok(obj)
    }
}

impl TopicConfigurationListWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &TopicConfigurationList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            TopicConfigurationWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

impl TopicConfigurationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TopicConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = TopicConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Id" {
                obj.id = Some(try!(NotificationIdParser::parse_xml("Id", stack)));
                continue;
            }
            if current_name == "Topic" {
                obj.topic_arn = try!(TopicArnParser::parse_xml("Topic", stack));
                continue;
            }
            if current_name == "Event" {
                obj.events = try!(EventListParser::parse_xml("Event", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl TopicConfigurationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &TopicConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.id {
            NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), obj);
        }
        TopicArnWriter::write_params(params, &(prefix.to_string() + "Topic"), &obj.topic_arn);
        EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
    }
}

impl NotificationConfigurationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NotificationConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = NotificationConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "QueueConfiguration" {
                obj.queue_configurations = try!(QueueConfigurationListParser::parse_xml("QueueConfiguration", stack));
                continue;
            }
            if current_name == "LambdaFunctionConfiguration" {
                obj.lambda_function_configurations = try!(LambdaFunctionConfigurationListParser::parse_xml("LambdaFunctionConfiguration", stack));
                continue;
            }
            if current_name == "TopicConfiguration" {
                obj.topic_configurations = try!(TopicConfigurationListParser::parse_xml("TopicConfiguration", stack));
                continue;
            }
            if current_name == "CloudFunctionConfiguration" {
                obj.cloud_function_configurations = try!(CloudFunctionConfigurationListParser::parse_xml("CloudFunctionConfiguration", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl NotificationConfigurationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &NotificationConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        QueueConfigurationListWriter::write_params(params, &(prefix.to_string() + "QueueConfiguration"), &obj.queue_configurations);
        LambdaFunctionConfigurationListWriter::write_params(params, &(prefix.to_string() + "LambdaFunctionConfiguration"), &obj.lambda_function_configurations);
        TopicConfigurationListWriter::write_params(params, &(prefix.to_string() + "TopicConfiguration"), &obj.topic_configurations);
        CloudFunctionConfigurationListWriter::write_params(params, &(prefix.to_string() + "CloudFunctionConfiguration"), &obj.cloud_function_configurations);
    }
}

impl BucketAlreadyExistsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketAlreadyExists, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = BucketAlreadyExists::default();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl BucketAlreadyExistsWriter {
    #[allow(unused_variables)]
    pub fn write_params(params: &mut Params, name: &str, obj: &BucketAlreadyExists) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
    }
}

impl BucketLocationConstraintParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketLocationConstraint, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl BucketLocationConstraintWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &BucketLocationConstraint) {
        params.put(name, obj);
    }
}

impl HeadBucketRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<HeadBucketRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = HeadBucketRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl HeadBucketRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &HeadBucketRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

impl TargetPrefixParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TargetPrefix, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl TargetPrefixWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &TargetPrefix) {
        params.put(name, obj);
    }
}

impl TargetBucketParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TargetBucket, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl TargetBucketWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &TargetBucket) {
        params.put(name, obj);
    }
}

impl TargetGrantsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TargetGrants, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Grant" {
            obj.push(try!(TargetGrantParser::parse_xml("Grant", stack)));
        }
        Ok(obj)
    }
}

impl TargetGrantsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &TargetGrants) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            TargetGrantWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

impl LoggingEnabledParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LoggingEnabled, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = LoggingEnabled::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "TargetPrefix" {
                obj.target_prefix = try!(TargetPrefixParser::parse_xml("TargetPrefix", stack));
                continue;
            }
            if current_name == "TargetBucket" {
                obj.target_bucket = try!(TargetBucketParser::parse_xml("TargetBucket", stack));
                continue;
            }
            if current_name == "Grant" {
                obj.target_grants = try!(TargetGrantsParser::parse_xml("Grant", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl LoggingEnabledWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &LoggingEnabled) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        TargetPrefixWriter::write_params(params, &(prefix.to_string() + "TargetPrefix"), &obj.target_prefix);
        TargetBucketWriter::write_params(params, &(prefix.to_string() + "TargetBucket"), &obj.target_bucket);
        TargetGrantsWriter::write_params(params, &(prefix.to_string() + "Grant"), &obj.target_grants);
    }
}
