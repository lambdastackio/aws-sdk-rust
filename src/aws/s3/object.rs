//
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

// Portions borrowed from the rusoto project. See README.md

//! Library Documentation
//!
//! This file contains everything related to `Objects`.

#![allow(unused_variables)]
use std::str::FromStr;
use std::str;

use aws::common::params::{Params, ServiceParams};
use aws::common::xmlutil::*;
use aws::common::common::*;
use aws::errors::http::*;
use aws::s3::header::*;
use aws::s3::bucket::*;
use aws::s3::acl::*;
use aws::s3::grant::*;
use aws::s3::writeparse::*;

pub type TagSet = Vec<Tag>;

pub type PartNumber = i32;

pub type UploadIdMarker = String;

pub type NextUploadIdMarker = String;

pub type MultipartUploadList = Vec<MultipartUpload>;

pub type MaxUploads = i32;

pub type Expires = String;

pub type ObjectMetadataList = Vec<ObjectMetadata>;

/// Parse `Tag` from XML
pub struct TagParser;

/// Write `Tag` contents to a `SignedRequest`
pub struct TagWriter;

/// Parse `TagSet` from XML
pub struct TagSetParser;

/// Write `TagSet` contents to a `SignedRequest`
pub struct TagSetWriter;

/// Parse `PartNumber` from XML
pub struct PartNumberParser;

/// Write `PartNumber` contents to a `SignedRequest`
pub struct PartNumberWriter;

/// Parse `Part` from XML
pub struct PartParser;

/// Write `Part` contents to a `SignedRequest`
pub struct PartWriter;

/// Parse `MultipartUpload` from XML
pub struct MultipartUploadParser;

/// Write `MultipartUpload` contents to a `SignedRequest`
pub struct MultipartUploadWriter;

/// Parse `UploadIdMarker` from XML
pub struct UploadIdMarkerParser;

/// Write `UploadIdMarker` contents to a `SignedRequest`
pub struct UploadIdMarkerWriter;

/// Parse `NextUploadIdMarker` from XML
pub struct NextUploadIdMarkerParser;

/// Write `NextUploadIdMarker` contents to a `SignedRequest`
pub struct NextUploadIdMarkerWriter;

/// Parse `MultipartUploadList` from XML
pub struct MultipartUploadListParser;

/// Write `MultipartUploadList` contents to a `SignedRequest`
pub struct MultipartUploadListWriter;

/// Parse `ListMultipartUploadsOutput` from XML
pub struct ListMultipartUploadsOutputParser;

/// Write `ListMultipartUploadsOutput` contents to a `SignedRequest`
pub struct ListMultipartUploadsOutputWriter;

/// Parse `CompleteMultipartUploadOutput` from XML
pub struct CompleteMultipartUploadOutputParser;

/// Write `CompleteMultipartUploadOutput` contents to a `SignedRequest`
pub struct CompleteMultipartUploadOutputWriter;

/// Parse `GetObjectRequest` from XML
pub struct GetObjectRequestParser;

/// Write `GetObjectRequest` contents to a `SignedRequest`
pub struct GetObjectRequestWriter;

/// Parse `PutObjectOutput` from XML
pub struct PutObjectOutputParser;

/// Write `PutObjectOutput` contents to a `SignedRequest`
pub struct PutObjectOutputWriter;

/// Parse `MaxUploads` from XML
pub struct MaxUploadsParser;

/// Write `MaxUploads` contents to a `SignedRequest`
pub struct MaxUploadsWriter;

/// Parse `Expires` from XML
pub struct ExpiresParser;

/// Write `Expires` contents to a `SignedRequest`
pub struct ExpiresWriter;

/// Parse `ListObjectsOutput` from XML
pub struct ListObjectsOutputParser;

/// Write `ListObjectsOutput` contents to a `SignedRequest`
pub struct ListObjectsOutputWriter;

/// Parse `ObjectMetadataList` from XML
pub struct ObjectMetadataListParser;

/// Write `ObjectMetadataList` contents to a `SignedRequest`
pub struct ObjectMetadataListWriter;

/// Parse `ObjectMetadata` from XML
pub struct ObjectMetadataParser;

/// Write `ObjectMetadata` contents to a `SignedRequest`
pub struct ObjectMetadataWriter;

/// `ObjectMetadata` used for `Contents` for ListObjectsOutput
#[derive(Debug, Default)]
pub struct ObjectMetadata {
    pub last_modified: LastModified,
    pub e_tag: ETag,
    /// The class of storage used to store the object.
    pub storage_class: ObjectStorageClass,
    pub key: ObjectKey,
    pub owner: Owner,
    pub size: Size,
}

#[derive(Debug, Default)]
pub struct Delete {
    pub objects: ObjectIdentifierList,
    /// Element to enable quiet mode for the request. When you add this element, you
    /// must set its value to true.
    pub quiet: Option<Quiet>,
}

#[derive(Debug, Default)]
pub struct GetObjectOutput {
    /// Last modified date of the object
    pub last_modified: LastModified,
    /// The portion of the object returned in the response.
    pub content_range: ContentRange,
    pub request_charged: RequestCharged,
    /// Specifies what content encodings have been applied to the object and thus what
    /// decoding mechanisms must be applied to obtain the media-type referenced by the
    /// Content-Type header field.
    pub content_encoding: ContentEncoding,
    pub replication_status: ReplicationStatus,
    pub storage_class: StorageClass,
    /// The Server-side encryption algorithm used when storing this object in S3
    /// (e.g., AES256, aws:kms).
    pub server_side_encryption: ServerSideEncryption,
    /// If present, specifies the ID of the AWS Key Management Service (KMS) master
    /// encryption key that was used for the object.
    pub ssekms_key_id: SSEKMSKeyId,
    /// Specifies presentational information for the object.
    pub content_disposition: ContentDisposition,
    /// A map of metadata to store with the object in S3.
    pub metadata: Metadata,
    /// Object data.
    pub body: Body,
    pub accept_ranges: AcceptRanges,
    /// If the bucket is configured as a website, redirects requests for this object
    /// to another object in the same bucket or to an external URL. Amazon S3 stores
    /// the value of this header in the object metadata.
    pub website_redirect_location: WebsiteRedirectLocation,
    /// The date and time at which the object is no longer cacheable.
    pub expires: Expires,
    /// Specifies whether the object retrieved was (true) or was not (false) a Delete
    /// Marker. If false, this response header does not appear in the response.
    pub delete_marker: DeleteMarker,
    /// Specifies caching behavior along the request/reply chain.
    pub cache_control: CacheControl,
    /// Size of the body in bytes.
    pub content_length: ContentLength,
    /// If the object expiration is configured (see PUT Bucket lifecycle), the
    /// response includes this header. It includes the expiry-date and rule-id key
    /// value pairs providing object expiration information. The value of the rule-id
    /// is URL encoded.
    pub expiration: Expiration,
    /// This is set to the number of metadata entries not returned in x-amz-meta
    /// headers. This can happen if you create metadata using an API like SOAP that
    /// supports more flexible metadata than the REST API. For example, using SOAP,
    /// you can create metadata whose values are not legal HTTP headers.
    pub missing_meta: MissingMeta,
    /// Provides information about object restoration operation and expiration time of
    /// the restored object copy.
    pub restore: Restore,
    /// If server-side encryption with a customer-provided encryption key was
    /// requested, the response will include this header confirming the encryption
    /// algorithm used.
    pub sse_customer_algorithm: SSECustomerAlgorithm,
    /// A standard MIME type describing the format of the object data.
    pub content_type: ContentType,
    /// The language the content is in.
    pub content_language: ContentLanguage,
    /// Version of the object.
    pub version_id: ObjectVersionId,
    /// An ETag is an opaque identifier assigned by a web server to a specific version
    /// of a resource found at a URL
    pub e_tag: ETag,
    /// If server-side encryption with a customer-provided encryption key was
    /// requested, the response will include this header to provide round trip message
    /// integrity verification of the customer-provided encryption key.
    pub sse_customer_key_md5: SSECustomerKeyMD5,
}

#[derive(Debug, Default)]
pub struct RestoreObjectOutput {
    pub request_charged: RequestCharged,
}

#[derive(Debug, Default)]
pub struct RestoreObjectRequest {
    pub version_id: Option<ObjectVersionId>,
    pub restore_request: Option<RestoreRequest>,
    pub bucket: BucketName,
    pub request_payer: Option<RequestPayer>,
    pub key: ObjectKey,
}

#[derive(Debug, Default)]
pub struct RestoreRequest {
    /// Lifetime of the active copy in days
    pub days: Days,
}

#[derive(Debug, Default)]
pub struct DeleteObjectRequest {
    /// The concatenation of the authentication device's serial number, a space, and
    /// the value that is displayed on your authentication device.
    pub mfa: Option<MFA>,
    /// VersionId used to reference a specific version of the object.
    pub version_id: Option<ObjectVersionId>,
    pub bucket: BucketName,
    pub request_payer: Option<RequestPayer>,
    pub key: ObjectKey,
}

#[derive(Debug, Default)]
pub struct DeleteObjectOutput {
    /// Returns the version ID of the delete marker created as a result of the DELETE
    /// operation.
    pub version_id: ObjectVersionId,
    pub request_charged: RequestCharged,
    /// Specifies whether the versioned object that was permanently deleted was (true)
    /// or was not (false) a delete marker.
    pub delete_marker: DeleteMarker,
}

#[derive(Debug, Default)]
pub struct DeleteObjectsOutput {
    pub deleted: DeletedObjects,
    pub errors: Errors,
    pub request_charged: RequestCharged,
}

#[derive(Debug, Default)]
pub struct DeletedObject {
    pub version_id: ObjectVersionId,
    pub delete_marker_version_id: DeleteMarkerVersionId,
    pub key: ObjectKey,
    pub delete_marker: DeleteMarker,
}

#[derive(Debug, Default)]
pub struct PutObjectOutput {
    /// If server-side encryption with a customer-provided encryption key was
    /// requested, the response will include this header confirming the encryption
    /// algorithm used.
    pub sse_customer_algorithm: SSECustomerAlgorithm,
    pub request_charged: RequestCharged,
    /// Version of the object.
    pub version_id: ObjectVersionId,
    /// Entity tag for the uploaded object.
    pub e_tag: ETag,
    /// If the object expiration is configured, this will contain the expiration date
    /// (expiry-date) and rule ID (rule-id). The value of rule-id is URL encoded.
    pub expiration: Expiration,
    /// The Server-side encryption algorithm used when storing this object in S3
    /// (e.g., AES256, aws:kms).
    pub server_side_encryption: ServerSideEncryption,
    /// If server-side encryption with a customer-provided encryption key was
    /// requested, the response will include this header to provide round trip message
    /// integrity verification of the customer-provided encryption key.
    pub sse_customer_key_md5: SSECustomerKeyMD5,
    /// If present, specifies the ID of the AWS Key Management Service (KMS) master
    /// encryption key that was used for the object.
    pub ssekms_key_id: SSEKMSKeyId,
}

/// Container for specifying the configuration when you want Amazon S3 to publish
/// events to an Amazon Simple Notification Service (Amazon SNS) topic.
#[derive(Debug, Default)]
pub struct TopicConfiguration {
    pub id: Option<NotificationId>,
    /// Amazon SNS topic ARN to which Amazon S3 will publish a message when it detects
    /// events of specified type.
    pub topic_arn: TopicArn,
    pub events: EventList,
}

/// Container for specifying an configuration when you want Amazon S3 to publish
/// events to an Amazon Simple Queue Service (Amazon SQS) queue.
#[derive(Debug, Default)]
pub struct QueueConfiguration {
    pub id: Option<NotificationId>,
    /// Amazon SQS queue ARN to which Amazon S3 will publish a message when it detects
    /// events of specified type.
    pub queue_arn: QueueArn,
    pub events: EventList,
}

#[derive(Debug, Default)]
pub struct ObjectIdentifier {
    /// VersionId for the specific version of the object to delete.
    pub version_id: Option<ObjectVersionId>,
    /// Key name of the object to delete.
    pub key: ObjectKey,
}

#[derive(Debug, Default)]
pub struct CreateMultipartUploadRequest {
    pub request_payer: Option<RequestPayer>,
    /// Specifies what content encodings have been applied to the object and thus what
    /// decoding mechanisms must be applied to obtain the media-type referenced by the
    /// Content-Type header field.
    pub content_encoding: Option<ContentEncoding>,
    /// The type of storage to use for the object. Defaults to 'STANDARD'.
    pub storage_class: Option<StorageClass>,
    /// Allows grantee to read the object ACL.
    pub grant_read_acp: Option<GrantReadACP>,
    /// The Server-side encryption algorithm used when storing this object in S3
    /// (e.g., AES256, aws:kms).
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// Specifies the AWS KMS key ID to use for object encryption. All GET and PUT
    /// requests for an object protected by AWS KMS will fail if not made via SSL or
    /// using SigV4. Documentation on configuring any of the officially supported AWS
    /// SDKs and CLI can be found at
    /// http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-
    /// signature-version
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    /// Specifies presentational information for the object.
    pub content_disposition: Option<ContentDisposition>,
    /// A map of metadata to store with the object in S3.
    pub metadata: Option<Metadata>,
    /// Specifies the customer-provided encryption key for Amazon S3 to use in
    /// encrypting data. This value is used to store the object and then it is
    /// discarded; Amazon does not store the encryption key. The key must be
    /// appropriate for use with the algorithm specified in the x-amz-server-side-
    /// encryption-customer-algorithm header.
    pub sse_customer_key: Option<SSECustomerKey>,
    /// If the bucket is configured as a website, redirects requests for this object
    /// to another object in the same bucket or to an external URL. Amazon S3 stores
    /// the value of this header in the object metadata.
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    /// The date and time at which the object is no longer cacheable.
    pub expires: Option<Expires>,
    pub key: ObjectKey,
    /// Specifies caching behavior along the request/reply chain.
    pub cache_control: Option<CacheControl>,
    pub bucket: BucketName,
    /// Allows grantee to read the object data and its metadata.
    pub grant_read: Option<GrantRead>,
    /// Allows grantee to write the ACL for the applicable object.
    pub grant_write_acp: Option<GrantWriteACP>,
    /// The canned ACL to apply to the object.
    pub acl: Option<ObjectCannedACL>,
    /// Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object.
    pub grant_full_control: Option<GrantFullControl>,
    /// Specifies the algorithm to use to when encrypting the object (e.g., AES256).
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// A standard MIME type describing the format of the object data.
    pub content_type: Option<ContentType>,
    /// The language the content is in.
    pub content_language: Option<ContentLanguage>,
    /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
    /// Amazon S3 uses this header for a message integrity check to ensure the
    /// encryption key was transmitted without error.
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
}

#[derive(Debug, Default)]
pub struct CompleteMultipartUploadRequest <'a> {
    pub multipart_upload: Option<&'a [u8]>,
    pub upload_id: MultipartUploadId,
    pub bucket: BucketName,
    pub request_payer: Option<RequestPayer>,
    pub key: ObjectKey,
}

#[derive(Debug, Default)]
pub struct UploadPartRequest <'a> {
    pub body: Option<&'a [u8]>,
    /// Specifies the algorithm to use to when encrypting the object (e.g., AES256).
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    pub request_payer: Option<RequestPayer>,
    /// Size of the body in bytes. This parameter is useful when the size of the body
    /// cannot be determined automatically.
    pub content_length: Option<ContentLength>,
    pub content_md5: Option<ContentMD5>,
    pub bucket: BucketName,
    /// Specifies the customer-provided encryption key for Amazon S3 to use in
    /// encrypting data. This value is used to store the object and then it is
    /// discarded; Amazon does not store the encryption key. The key must be
    /// appropriate for use with the algorithm specified in the x-amz-server-side-
    /// encryption-customer-algorithm header. This must be the same encryption key
    /// specified in the initiate multipart upload request.
    pub sse_customer_key: Option<SSECustomerKey>,
    /// Upload ID identifying the multipart upload whose part is being uploaded.
    pub upload_id: MultipartUploadId,
    pub key: ObjectKey,
    /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
    /// Amazon S3 uses this header for a message integrity check to ensure the
    /// encryption key was transmitted without error.
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// Part number of part being uploaded. This is a positive integer between 1 and
    /// 10,000.
    pub part_number: PartNumber,
}

#[derive(Debug, Default)]
pub struct Part {
    /// Date and time at which the part was uploaded.
    pub last_modified: LastModified,
    /// Part number identifying the part. This is a positive integer between 1 and
    /// 10,000.
    pub part_number: PartNumber,
    /// Entity tag returned when the part was uploaded.
    pub e_tag: ETag,
    /// Size of the uploaded part data.
    pub size: Size,
}

#[derive(Debug, Default)]
pub struct PutObjectRequest<'a> {
    pub request_payer: Option<RequestPayer>,
    /// Specifies what content encodings have been applied to the object and thus what
    /// decoding mechanisms must be applied to obtain the media-type referenced by the
    /// Content-Type header field.
    pub content_encoding: Option<ContentEncoding>,
    /// The type of storage to use for the object. Defaults to 'STANDARD'.
    pub storage_class: Option<StorageClass>,
    /// Allows grantee to read the object ACL.
    pub grant_read_acp: Option<GrantReadACP>,
    /// The Server-side encryption algorithm used when storing this object in S3
    /// (e.g., AES256, aws:kms).
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// Specifies the AWS KMS key ID to use for object encryption. All GET and PUT
    /// requests for an object protected by AWS KMS will fail if not made via SSL or
    /// using SigV4. Documentation on configuring any of the officially supported AWS
    /// SDKs and CLI can be found at
    /// http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-
    /// signature-version
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    /// Specifies presentational information for the object.
    pub content_disposition: Option<ContentDisposition>,
    /// A map of metadata to store with the object in S3.
    pub metadata: Option<Metadata>,
    /// Object data.
    pub body: Option<&'a [u8]>,
    /// Specifies the customer-provided encryption key for Amazon S3 to use in
    /// encrypting data. This value is used to store the object and then it is
    /// discarded; Amazon does not store the encryption key. The key must be
    /// appropriate for use with the algorithm specified in the x-amz-server-side-
    /// encryption-customer-algorithm header.
    pub sse_customer_key: Option<SSECustomerKey>,
    /// If the bucket is configured as a website, redirects requests for this object
    /// to another object in the same bucket or to an external URL. Amazon S3 stores
    /// the value of this header in the object metadata.
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    /// The date and time at which the object is no longer cacheable.
    pub expires: Option<Expires>,
    pub key: ObjectKey,
    /// Specifies caching behavior along the request/reply chain.
    pub cache_control: Option<CacheControl>,
    /// Size of the body in bytes. This parameter is useful when the size of the body
    /// cannot be determined automatically.
    pub content_length: Option<ContentLength>,
    pub bucket: BucketName,
    /// Allows grantee to read the object data and its metadata.
    pub grant_read: Option<GrantRead>,
    /// Allows grantee to write the ACL for the applicable object.
    pub grant_write_acp: Option<GrantWriteACP>,
    /// The canned ACL to apply to the object.
    pub acl: Option<CannedAcl>,
    /// Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object.
    pub grant_full_control: Option<GrantFullControl>,
    /// Specifies the algorithm to use to when encrypting the object (e.g., AES256).
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// A standard MIME type describing the format of the object data.
    pub content_type: Option<ContentType>,
    /// The language the content is in.
    pub content_language: Option<ContentLanguage>,
    pub content_md5: Option<ContentMD5>,
    /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
    /// Amazon S3 uses this header for a message integrity check to ensure the
    /// encryption key was transmitted without error.
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
}

#[derive(Debug, Default)]
pub struct GetObjectRequest {
    /// Sets the Content-Encoding header of the response.
    pub response_content_encoding: Option<ResponseContentEncoding>,
    /// Sets the Content-Language header of the response.
    pub response_content_language: Option<ResponseContentLanguage>,
    /// Specifies the algorithm to use to when encrypting the object (e.g., AES256).
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// Sets the Content-Type header of the response.
    pub response_content_type: Option<ResponseContentType>,
    /// Return the object only if it has not been modified since the specified time,
    /// otherwise return a 412 (precondition failed).
    pub if_unmodified_since: Option<IfUnmodifiedSince>,
    /// VersionId used to reference a specific version of the object.
    pub version_id: Option<ObjectVersionId>,
    pub request_payer: Option<RequestPayer>,
    /// Sets the Cache-Control header of the response.
    pub response_cache_control: Option<ResponseCacheControl>,
    /// Specifies the customer-provided encryption key for Amazon S3 to use in
    /// encrypting data. This value is used to store the object and then it is
    /// discarded; Amazon does not store the encryption key. The key must be
    /// appropriate for use with the algorithm specified in the x-amz-server-side-
    /// encryption-customer-algorithm header.
    pub sse_customer_key: Option<SSECustomerKey>,
    pub bucket: BucketName,
    /// Return the object only if its entity tag (ETag) is different from the one
    /// specified, otherwise return a 304 (not modified).
    pub if_none_match: Option<IfNoneMatch>,
    /// Sets the Content-Disposition header of the response
    pub response_content_disposition: Option<ResponseContentDisposition>,
    /// Downloads the specified range bytes of an object. For more information about
    /// the HTTP Range header, go to
    /// http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35.
    pub range: Option<Range>,
    pub key: ObjectKey,
    /// Return the object only if its entity tag (ETag) is the same as the one
    /// specified, otherwise return a 412 (precondition failed).
    pub if_match: Option<IfMatch>,
    /// Sets the Expires header of the response.
    pub response_expires: Option<ResponseExpires>,
    /// Return the object only if it has been modified since the specified time,
    /// otherwise return a 304 (not modified).
    pub if_modified_since: Option<IfModifiedSince>,
    /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
    /// Amazon S3 uses this header for a message integrity check to ensure the
    /// encryption key was transmitted without error.
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
}

#[derive(Debug, Default)]
pub struct Initiator {
    /// Name of the Principal.
    pub display_name: DisplayName,
    /// If the principal is an AWS account, it provides the Canonical User ID. If the
    /// principal is an IAM User, it provides a user ARN value.
    pub id: ID,
}

#[derive(Debug, Default)]
pub struct GetObjectTorrentRequest {
    pub bucket: BucketName,
    pub request_payer: Option<RequestPayer>,
    pub key: ObjectKey,
}

#[derive(Debug, Default)]
pub struct Redirect {
    /// The specific object key to use in the redirect request. For example, redirect
    /// request to error.html. Not required if one of the sibling is present. Can be
    /// present only if ReplaceKeyPrefixWith is not provided.
    pub replace_key_with: ReplaceKeyWith,
    /// The host name to use in the redirect request.
    pub host_name: HostName,
    /// Protocol to use (http, https) when redirecting requests. The default is the
    /// protocol that is used in the original request.
    pub protocol: Protocol,
    /// The object key prefix to use in the redirect request. For example, to redirect
    /// requests for all pages with prefix docs/ (objects in the docs/ folder) to
    /// documents/, you can set a condition block with KeyPrefixEquals set to docs/
    /// and in the Redirect set ReplaceKeyPrefixWith to /documents. Not required if
    /// one of the siblings is present. Can be present only if ReplaceKeyWith is not
    /// provided.
    pub replace_key_prefix_with: ReplaceKeyPrefixWith,
    /// The HTTP redirect code to use on the response. Not required if one of the
    /// siblings is present.
    pub http_redirect_code: HttpRedirectCode,
}

#[derive(Debug, Default)]
pub struct ErrorDocument {
    /// The object key name to use when a 4XX class error occurs.
    pub key: ObjectKey,
}

#[derive(Debug, Default)]
pub struct IndexDocument {
    /// A suffix that is appended to a request that is for a directory on the website
    /// endpoint (e.g. if the suffix is index.html and you make a request to
    /// samplebucket/images/ the data that is returned will be for the object with the
    /// key name images/index.html) The suffix must not be empty and must not include
    /// a slash character.
    pub suffix: Suffix,
}

#[derive(Debug, Default)]
pub struct RedirectAllRequestsTo {
    /// Name of the host where requests will be redirected.
    pub host_name: HostName,
    /// Protocol to use (http, https) when redirecting requests. The default is the
    /// protocol that is used in the original request.
    pub protocol: Option<Protocol>,
}

/// Container for specifying the AWS Lambda notification configuration.
#[derive(Debug, Default)]
pub struct LambdaFunctionConfiguration {
    /// Lambda cloud function ARN that Amazon S3 can invoke when it detects events of
    /// the specified type.
    pub lambda_function_arn: LambdaFunctionArn,
    pub id: Option<NotificationId>,
    pub events: EventList,
}

#[derive(Debug, Default)]
pub struct Tag {
    /// Value of the tag.
    pub value: Value,
    /// Name of the tag.
    pub key: ObjectKey,
}

#[derive(Debug, Default)]
pub struct CopyObjectResult {
    pub last_modified: LastModified,
    pub e_tag: ETag,
}

#[derive(Debug, Default)]
pub struct ListMultipartUploadsOutput {
    /// Upload ID after which listing began.
    pub upload_id_marker: UploadIdMarker,
    pub common_prefixes: CommonPrefixList,
    /// When a list is truncated, this element specifies the value that should be used
    /// for the key-marker request parameter in a subsequent request.
    pub next_key_marker: NextKeyMarker,
    /// Name of the bucket to which the multipart upload was initiated.
    pub bucket: BucketName,
    pub delimiter: Delimiter,
    /// When a list is truncated, this element specifies the value that should be used
    /// for the upload-id-marker request parameter in a subsequent request.
    pub next_upload_id_marker: NextUploadIdMarker,
    /// When a prefix is provided in the request, this field contains the specified
    /// prefix. The result contains only keys starting with the specified prefix.
    pub prefix: Prefix,
    pub uploads: MultipartUploadList,
    /// The key at or after which the listing began.
    pub key_marker: KeyMarker,
    /// Maximum number of multipart uploads that could have been included in the
    /// response.
    pub max_uploads: MaxUploads,
    /// Encoding type used by Amazon S3 to encode object keys in the response.
    pub encoding_type: EncodingType,
    /// Indicates whether the returned list of multipart uploads is truncated. A value
    /// of true indicates that the list was truncated. The list can be truncated if
    /// the number of multipart uploads exceeds the limit allowed or specified by max
    /// uploads.
    pub is_truncated: IsTruncated,
}

#[derive(Debug, Default)]
pub struct CreateMultipartUploadOutput {
    /// If server-side encryption with a customer-provided encryption key was
    /// requested, the response will include this header confirming the encryption
    /// algorithm used.
    pub sse_customer_algorithm: SSECustomerAlgorithm,
    pub request_charged: RequestCharged,
    /// Name of the bucket to which the multipart upload was initiated.
    pub bucket: BucketName,
    /// ID for the initiated multipart upload.
    pub upload_id: MultipartUploadId,
    /// Object key for which the multipart upload was initiated.
    pub key: ObjectKey,
    /// The Server-side encryption algorithm used when storing this object in S3
    /// (e.g., AES256, aws:kms).
    pub server_side_encryption: ServerSideEncryption,
    /// If server-side encryption with a customer-provided encryption key was
    /// requested, the response will include this header to provide round trip message
    /// integrity verification of the customer-provided encryption key.
    pub sse_customer_key_md5: SSECustomerKeyMD5,
    /// If present, specifies the ID of the AWS Key Management Service (KMS) master
    /// encryption key that was used for the object.
    pub ssekms_key_id: SSEKMSKeyId,
}

#[derive(Debug, Default)]
pub struct CompleteMultipartUploadOutput {
    pub request_charged: RequestCharged,
    pub bucket: BucketName,
    /// Version of the object.
    pub version_id: ObjectVersionId,
    /// Entity tag of the object.
    pub e_tag: ETag,
    pub location: Location,
    pub key: ObjectKey,
    /// The Server-side encryption algorithm used when storing this object in S3
    /// (e.g., AES256, aws:kms).
    pub server_side_encryption: ServerSideEncryption,
    /// If present, specifies the ID of the AWS Key Management Service (KMS) master
    /// encryption key that was used for the object.
    pub ssekms_key_id: SSEKMSKeyId,
    /// If the object expiration is configured, this will contain the expiration date
    /// (expiry-date) and rule ID (rule-id). The value of rule-id is URL encoded.
    pub expiration: Expiration,
}

/// Specifies when noncurrent object versions expire. Upon expiration, Amazon S3
/// permanently deletes the noncurrent object versions. You set this lifecycle
/// configuration action on a bucket that has versioning enabled (or suspended) to
/// request that Amazon S3 delete noncurrent object versions at a specific period
/// in the object's lifetime.
#[derive(Debug, Default)]
pub struct NoncurrentVersionExpiration {
    /// Specifies the number of days an object is noncurrent before Amazon S3 can
    /// perform the associated action. For information about the noncurrent days
    /// calculations, see [How Amazon S3 Calculates When an Object Became
    /// Noncurrent](/AmazonS3/latest/dev/s3-access-control.html) in the Amazon Simple
    /// Storage Service Developer Guide.
    pub noncurrent_days: Days,
}

#[derive(Debug, Default)]
pub struct ReplicationRule {
    /// The rule is ignored if status is not Enabled.
    pub status: ReplicationRuleStatus,
    /// Object keyname prefix identifying one or more objects to which the rule
    /// applies. Maximum prefix length can be up to 1,024 characters. Overlapping
    /// prefixes are not supported.
    pub prefix: Prefix,
    pub destination: Destination,
    /// Unique identifier for the rule. The value cannot be longer than 255
    /// characters.
    pub id: Option<ID>,
}

#[derive(Debug, Default)]
pub struct ListObjectsRequest {
    /// Required. Name of bucket.
    pub bucket: BucketName,
    /// Two versions: the older version 1 and the newer version 2. Defaults to version 1. You
    /// must specify version 2 with a value of `Some(2)` since it's an Option<i32> field type.
    pub version: Option<i32>,
    /// Limits the response to keys that begin with the specified prefix.
    pub prefix: Option<Prefix>,
    /// Sets the maximum number of keys returned in the response. The response might
    /// contain fewer keys but will never contain more.
    pub max_keys: Option<MaxKeys>,
    /// A delimiter is a character you use to group keys.
    pub delimiter: Option<Delimiter>,
    /// Specifies the key to start with when listing objects in a bucket.
    pub marker: Option<Marker>,
    pub encoding_type: Option<EncodingType>,
}

/// ListObjectsOutput contains the list of objects and their associated metadata for a given
/// bucket name. There are two versions, version 1 and version 2. AWS S3 supports both.
/// The struct field names that are unique for Version 1 are marked and those that are unique
/// for Version 2 are marked. Those not marked are common between versions. The default for
/// AWS is version 2 but you have to specify version 2 in ListObjectsRequest or it will default
/// to version 1.
#[derive(Debug, Default)]
pub struct ListObjectsOutput {
    pub name: BucketName,
    /// Version 1. When response is truncated (the IsTruncated element value in the response is
    /// true), you can use the key name in this field as marker in the subsequent
    /// request to get next set of objects. Amazon S3 lists objects in alphabetical
    /// order Note: This element is returned only if you have delimiter request
    /// parameter specified. If response does not include the NextMaker and it is
    /// truncated, you can use the value of the last Key in the response as the marker
    /// in the subsequent request to get the next set of object keys.
    pub next_marker: NextMarker,
    pub delimiter: Delimiter,
    pub max_keys: MaxKeys,
    pub prefix: Prefix,
    /// Version 1.
    pub marker: Marker,
    /// Encoding type used by Amazon S3 to encode object keys in the response.
    pub encoding_type: EncodingType,
    /// A flag that indicates whether or not Amazon S3 returned all of the results
    /// that satisfied the search criteria.
    pub is_truncated: IsTruncated,
    /// List of ObjectMetadata for each object in the given bucket.
    pub contents: ObjectMetadataList,
    pub common_prefixes: CommonPrefixList,
    /// Version 2. Returned if included in the request
    pub continuation_token: ContinuationToken,
    /// Version 2. If the response is truncated, Amazon S3 returns this parameter with a continuation token
    /// that you can specify as the continuation-token in your next request to retrieve the next
    /// set of keys.
    pub next_continuation_token: ContinuationToken,
    /// Version 2. Returned number of keys in response. Always <= MaxKeys.
    pub key_count: KeyCount,
    /// Version 2. Is included with the response if sent with the request.
    pub start_after: StartAfter,
}

#[derive(Debug, Default)]
pub struct ListObjectVersionsRequest {
    pub bucket: BucketName,
    /// Limits the response to keys that begin with the specified prefix.
    pub prefix: Option<Prefix>,
    /// Sets the maximum number of keys returned in the response. The response might
    /// contain fewer keys but will never contain more.
    pub max_keys: Option<MaxKeys>,
    /// A delimiter is a character you use to group keys.
    pub delimiter: Option<Delimiter>,
    /// Specifies the key to start with when listing objects in a bucket.
    pub key_marker: Option<KeyMarker>,
    pub encoding_type: Option<EncodingType>,
    /// Specifies the object version you want to start listing from.
    pub version_id_marker: Option<VersionIdMarker>,
    /// Start the list after a specifc number.
    pub start_after: Option<StartAfter>,
}

// New way
#[derive(Debug, Default)]
pub struct ListVersionsResult {
    pub name: BucketName,
    pub prefix: Prefix,
    /// Marks the last Key returned in a truncated response.
    pub key_marker: KeyMarker,
    pub delete_markers: DeleteMarkers,
    pub max_keys: MaxKeys,
    /// A flag that indicates whether or not Amazon S3 returned all of the results
    /// that satisfied the search criteria. If your results were truncated, you can
    /// make a follow-up paginated request using the NextKeyMarker and
    /// NextVersionIdMarker response parameters as a starting place in another request
    /// to return the rest of the results.
    pub is_truncated: IsTruncated,
    pub versions: ObjectVersionList,
    /// Use this value for the key marker request parameter in a subsequent request.
    pub next_key_marker: NextKeyMarker,
    pub delimiter: Delimiter,
    /// Use this value for the next version id marker parameter in a subsequent
    /// request.
    pub next_version_id_marker: NextVersionIdMarker,
    /// Encoding type used by Amazon S3 to encode object keys in the response.
    pub encoding_type: EncodingType,
    pub version_id_marker: VersionIdMarker,
    pub common_prefixes: CommonPrefixList,
}

//OLD Way - begin
pub type ObjectVersionList = Vec<ObjectVersion>;

#[derive(Debug, Default)]
pub struct ListObjectVersionsOutput {
    pub name: BucketName,
    pub versions: ObjectVersionList,
    pub delete_markers: DeleteMarkers,
    /// Use this value for the key marker request parameter in a subsequent request.
    pub next_key_marker: NextKeyMarker,
    pub delimiter: Delimiter,
    pub max_keys: MaxKeys,
    pub prefix: Prefix,
    /// Marks the last Key returned in a truncated response.
    pub key_marker: KeyMarker,
    /// Use this value for the next version id marker parameter in a subsequent
    /// request.
    pub next_version_id_marker: NextVersionIdMarker,
    /// Encoding type used by Amazon S3 to encode object keys in the response.
    pub encoding_type: EncodingType,
    /// A flag that indicates whether or not Amazon S3 returned all of the results
    /// that satisfied the search criteria. If your results were truncated, you can
    /// make a follow-up paginated request using the NextKeyMarker and
    /// NextVersionIdMarker response parameters as a starting place in another request
    /// to return the rest of the results.
    pub is_truncated: IsTruncated,
    pub version_id_marker: VersionIdMarker,
    pub common_prefixes: CommonPrefixList,
}
//OLD Way - end

#[derive(Debug, Default)]
pub struct DeleteMarkerEntry {
    pub owner: Owner,
    /// Specifies whether the object is (true) or is not (false) the latest version of
    /// an object.
    pub is_latest: IsLatest,
    /// Version ID of an object.
    pub version_id: ObjectVersionId,
    /// The object key.
    pub key: ObjectKey,
    /// Date and time the object was last modified.
    pub last_modified: LastModified,
}

#[derive(Debug, Default)]
pub struct HeadObjectOutput {
    /// Last modified date of the object
    pub last_modified: LastModified,
    pub request_charged: RequestCharged,
    /// Specifies what content encodings have been applied to the object and thus what
    /// decoding mechanisms must be applied to obtain the media-type referenced by the
    /// Content-Type header field.
    pub content_encoding: ContentEncoding,
    pub replication_status: ReplicationStatus,
    pub storage_class: StorageClass,
    /// The Server-side encryption algorithm used when storing this object in S3
    /// (e.g., AES256, aws:kms).
    pub server_side_encryption: ServerSideEncryption,
    /// If present, specifies the ID of the AWS Key Management Service (KMS) master
    /// encryption key that was used for the object.
    pub ssekms_key_id: SSEKMSKeyId,
    /// Specifies presentational information for the object.
    pub content_disposition: ContentDisposition,
    /// A map of metadata to store with the object in S3.
    pub metadata: Metadata,
    pub accept_ranges: AcceptRanges,
    /// If the bucket is configured as a website, redirects requests for this object
    /// to another object in the same bucket or to an external URL. Amazon S3 stores
    /// the value of this header in the object metadata.
    pub website_redirect_location: WebsiteRedirectLocation,
    /// The date and time at which the object is no longer cacheable.
    pub expires: Expires,
    /// Specifies whether the object retrieved was (true) or was not (false) a Delete
    /// Marker. If false, this response header does not appear in the response.
    pub delete_marker: DeleteMarker,
    /// Specifies caching behavior along the request/reply chain.
    pub cache_control: CacheControl,
    /// Size of the body in bytes.
    pub content_length: ContentLength,
    /// If the object expiration is configured (see PUT Bucket lifecycle), the
    /// response includes this header. It includes the expiry-date and rule-id key
    /// value pairs providing object expiration information. The value of the rule-id
    /// is URL encoded.
    pub expiration: Expiration,
    /// This is set to the number of metadata entries not returned in x-amz-meta
    /// headers. This can happen if you create metadata using an API like SOAP that
    /// supports more flexible metadata than the REST API. For example, using SOAP,
    /// you can create metadata whose values are not legal HTTP headers.
    pub missing_meta: MissingMeta,
    /// Provides information about object restoration operation and expiration time of
    /// the restored object copy.
    pub restore: Restore,
    /// If server-side encryption with a customer-provided encryption key was
    /// requested, the response will include this header confirming the encryption
    /// algorithm used.
    pub sse_customer_algorithm: SSECustomerAlgorithm,
    /// A standard MIME type describing the format of the object data.
    pub content_type: ContentType,
    /// The language the content is in.
    pub content_language: ContentLanguage,
    /// Version of the object.
    pub version_id: ObjectVersionId,
    /// An ETag is an opaque identifier assigned by a web server to a specific version
    /// of a resource found at a URL
    pub e_tag: ETag,
    /// If server-side encryption with a customer-provided encryption key was
    /// requested, the response will include this header to provide round trip message
    /// integrity verification of the customer-provided encryption key.
    pub sse_customer_key_md5: SSECustomerKeyMD5,
}

#[derive(Debug, Default)]
pub struct CopyObjectOutput {
    /// If server-side encryption with a customer-provided encryption key was
    /// requested, the response will include this header confirming the encryption
    /// algorithm used.
    pub sse_customer_algorithm: SSECustomerAlgorithm,
    pub copy_source_version_id: CopySourceVersionId,
    /// The Server-side encryption algorithm used when storing this object in S3
    /// (e.g., AES256, aws:kms).
    pub server_side_encryption: ServerSideEncryption,
    pub request_charged: RequestCharged,
    /// If the object expiration is configured, the response includes this header.
    pub expiration: Expiration,
    /// If server-side encryption with a customer-provided encryption key was
    /// requested, the response will include this header to provide round trip message
    /// integrity verification of the customer-provided encryption key.
    pub sse_customer_key_md5: SSECustomerKeyMD5,
    pub copy_object_result: CopyObjectResult,
    /// If present, specifies the ID of the AWS Key Management Service (KMS) master
    /// encryption key that was used for the object.
    pub ssekms_key_id: SSEKMSKeyId,
}

#[derive(Debug, Default)]
pub struct CopyObjectRequest {
    pub request_payer: Option<RequestPayer>,
    /// Copies the object if it has been modified since the specified time.
    pub copy_source_if_modified_since: Option<CopySourceIfModifiedSince>,
    /// Copies the object if it hasn't been modified since the specified time.
    pub copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince>,
    /// Specifies what content encodings have been applied to the object and thus what
    /// decoding mechanisms must be applied to obtain the media-type referenced by the
    /// Content-Type header field.
    pub content_encoding: Option<ContentEncoding>,
    /// Specifies the customer-provided encryption key for Amazon S3 to use to decrypt
    /// the source object. The encryption key provided in this header must be one that
    /// was used when the source object was created.
    pub copy_source_sse_customer_key: Option<CopySourceSSECustomerKey>,
    /// The type of storage to use for the object. Defaults to 'STANDARD'.
    pub storage_class: Option<StorageClass>,
    /// Allows grantee to read the object ACL.
    pub grant_read_acp: Option<GrantReadACP>,
    /// The Server-side encryption algorithm used when storing this object in S3
    /// (e.g., AES256, aws:kms).
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// Specifies the AWS KMS key ID to use for object encryption. All GET and PUT
    /// requests for an object protected by AWS KMS will fail if not made via SSL or
    /// using SigV4. Documentation on configuring any of the officially supported AWS
    /// SDKs and CLI can be found at
    /// http://docs.aws.amazon.com/AmazonS3/latest/dev/UsingAWSSDK.html#specify-
    /// signature-version
    pub ssekms_key_id: Option<SSEKMSKeyId>,
    /// Specifies presentational information for the object.
    pub content_disposition: Option<ContentDisposition>,
    /// A map of metadata to store with the object in S3.
    pub metadata: Option<Metadata>,
    /// Specifies the customer-provided encryption key for Amazon S3 to use in
    /// encrypting data. This value is used to store the object and then it is
    /// discarded; Amazon does not store the encryption key. The key must be
    /// appropriate for use with the algorithm specified in the x-amz-server-side-
    /// encryption-customer-algorithm header.
    pub sse_customer_key: Option<SSECustomerKey>,
    /// If the bucket is configured as a website, redirects requests for this object
    /// to another object in the same bucket or to an external URL. Amazon S3 stores
    /// the value of this header in the object metadata.
    pub website_redirect_location: Option<WebsiteRedirectLocation>,
    /// The name of the source bucket and key name of the source object, separated by
    /// a slash (/). Must be URL-encoded.
    pub copy_source: CopySource,
    /// The date and time at which the object is no longer cacheable.
    pub expires: Option<Expires>,
    pub key: ObjectKey,
    /// Specifies caching behavior along the request/reply chain.
    pub cache_control: Option<CacheControl>,
    /// Specifies the algorithm to use when decrypting the source object (e.g.,
    /// AES256).
    pub copy_source_sse_customer_algorithm: Option<CopySourceSSECustomerAlgorithm>,
    pub bucket: BucketName,
    /// Allows grantee to read the object data and its metadata.
    pub grant_read: Option<GrantRead>,
    /// Allows grantee to write the ACL for the applicable object.
    pub grant_write_acp: Option<GrantWriteACP>,
    /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
    /// Amazon S3 uses this header for a message integrity check to ensure the
    /// encryption key was transmitted without error.
    pub copy_source_sse_customer_key_md5: Option<CopySourceSSECustomerKeyMD5>,
    /// The canned ACL to apply to the object.
    pub acl: Option<CannedAcl>,
    /// Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object.
    pub grant_full_control: Option<GrantFullControl>,
    /// Copies the object if its entity tag (ETag) matches the specified tag.
    pub copy_source_if_match: Option<CopySourceIfMatch>,
    /// Specifies the algorithm to use to when encrypting the object (e.g., AES256).
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// A standard MIME type describing the format of the object data.
    pub content_type: Option<ContentType>,
    /// The language the content is in.
    pub content_language: Option<ContentLanguage>,
    /// Specifies whether the metadata is copied from the source object or replaced
    /// with metadata provided in the request.
    pub metadata_directive: Option<MetadataDirective>,
    /// Copies the object if its entity tag (ETag) is different than the specified
    /// ETag.
    pub copy_source_if_none_match: Option<CopySourceIfNoneMatch>,
    /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
    /// Amazon S3 uses this header for a message integrity check to ensure the
    /// encryption key was transmitted without error.
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
}

#[derive(Debug, Default)]
pub struct DeleteObjectsRequest {
    /// The concatenation of the authentication device's serial number, a space, and
    /// the value that is displayed on your authentication device.
    pub mfa: Option<MFA>,
    pub bucket: BucketName,
    pub request_payer: Option<RequestPayer>,
    pub delete: Delete,
}

#[derive(Debug, Default)]
pub struct MultipartUpload {
    /// Identifies who initiated the multipart upload.
    pub initiator: Initiator,
    /// Date and time at which the multipart upload was initiated.
    pub initiated: Initiated,
    /// Upload ID that identifies the multipart upload.
    pub upload_id: MultipartUploadId,
    /// The class of storage used to store the object.
    pub storage_class: StorageClass,
    /// Key of the object for which the multipart upload was initiated.
    pub key: ObjectKey,
    pub owner: Owner,
}

impl TagParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<Tag, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Tag::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Value" {
                obj.value = try!(ValueParser::parse_xml("Value", stack));
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

impl TagWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Tag) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ValueWriter::write_params(params, &(prefix.to_string() + "Value"), &obj.value);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}

impl TagSetParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<TagSet, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Tag" {
            obj.push(try!(TagParser::parse_xml("Tag", stack)));
        }
        Ok(obj)
    }
}

impl TagSetWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &TagSet) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            TagWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

impl PartNumberParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<PartNumber, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl PartNumberWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &PartNumber) {
        params.put(name, &obj.to_string());
    }
}

impl PartParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<Part, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Part::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LastModified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
                continue;
            }
            if current_name == "PartNumber" {
                obj.part_number = try!(PartNumberParser::parse_xml("PartNumber", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "Size" {
                obj.size = try!(SizeParser::parse_xml("Size", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl PartWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Part) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(
                params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
        PartNumberWriter::write_params(
                params, &(prefix.to_string() + "PartNumber"), &obj.part_number);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        SizeWriter::write_params(params, &(prefix.to_string() + "Size"), &obj.size);
    }
}

impl MultipartUploadParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<MultipartUpload, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = MultipartUpload::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Initiator" {
                obj.initiator = try!(InitiatorParser::parse_xml("Initiator", stack));
                continue;
            }
            if current_name == "Initiated" {
                obj.initiated = try!(InitiatedParser::parse_xml("Initiated", stack));
                continue;
            }
            if current_name == "UploadId" {
                obj.upload_id = try!(MultipartUploadIdParser::parse_xml("UploadId", stack));
                continue;
            }
            if current_name == "StorageClass" {
                obj.storage_class = try!(StorageClassParser::parse_xml("StorageClass", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl MultipartUploadWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MultipartUpload) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        InitiatorWriter::write_params(params, &(prefix.to_string() + "Initiator"), &obj.initiator);
        InitiatedWriter::write_params(params, &(prefix.to_string() + "Initiated"), &obj.initiated);
        MultipartUploadIdWriter::write_params(
                params, &(prefix.to_string() + "UploadId"), &obj.upload_id);
        StorageClassWriter::write_params(
                params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
    }
}

impl UploadIdMarkerParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<UploadIdMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = UploadIdMarker::default();

        match characters(stack) {
            Err(why) => return Ok(obj),
            Ok(chars) => obj = chars,
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl UploadIdMarkerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &UploadIdMarker) {
        params.put(name, obj);
    }
}

impl NextUploadIdMarkerParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<NextUploadIdMarker, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NextUploadIdMarker::default();

        match characters(stack) {
            Err(why) => return Ok(obj),
            Ok(chars) => obj = chars,
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl NextUploadIdMarkerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &NextUploadIdMarker) {
        params.put(name, obj);
    }
}

impl MultipartUploadListParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<MultipartUploadList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "MultipartUpload" {
            obj.push(try!(MultipartUploadParser::parse_xml("MultipartUpload", stack)));
        }
        Ok(obj)
    }
}

impl MultipartUploadListWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MultipartUploadList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            MultipartUploadWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

impl ListMultipartUploadsOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<ListMultipartUploadsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListMultipartUploadsOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "UploadIdMarker" {
                obj.upload_id_marker = try!(UploadIdMarkerParser::parse_xml(
                                            "UploadIdMarker", stack));
                continue;
            }
            if current_name == "CommonPrefix" {
                obj.common_prefixes = try!(CommonPrefixListParser::parse_xml(
                                            "CommonPrefix", stack));
                continue;
            }
            if current_name == "NextKeyMarker" {
                obj.next_key_marker = try!(NextKeyMarkerParser::parse_xml(
                                            "NextKeyMarker", stack));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "Delimiter" {
                obj.delimiter = try!(DelimiterParser::parse_xml("Delimiter", stack));
                continue;
            }
            if current_name == "NextUploadIdMarker" {
                obj.next_upload_id_marker = try!(NextUploadIdMarkerParser::parse_xml(
                                            "NextUploadIdMarker", stack));
                continue;
            }
            if current_name == "Prefix" {
                obj.prefix = try!(PrefixParser::parse_xml("Prefix", stack));
                continue;
            }
            if current_name == "MultipartUpload" {
                obj.uploads = try!(MultipartUploadListParser::parse_xml(
                                            "MultipartUpload", stack));
                continue;
            }
            if current_name == "KeyMarker" {
                obj.key_marker = try!(KeyMarkerParser::parse_xml("KeyMarker", stack));
                continue;
            }
            if current_name == "MaxUploads" {
                obj.max_uploads = try!(MaxUploadsParser::parse_xml("MaxUploads", stack));
                continue;
            }
            if current_name == "EncodingType" {
                obj.encoding_type = try!(EncodingTypeParser::parse_xml("EncodingType", stack));
                continue;
            }
            if current_name == "IsTruncated" {
                obj.is_truncated = try!(IsTruncatedParser::parse_xml("IsTruncated", stack));
                continue;
            }
            if current_name == "Upload" {
                obj.uploads.push(try!(MultipartUploadParser::parse_xml("Upload", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ListMultipartUploadsOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ListMultipartUploadsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        UploadIdMarkerWriter::write_params(
                params, &(prefix.to_string() + "UploadIdMarker"), &obj.upload_id_marker);
        CommonPrefixListWriter::write_params(
                params, &(prefix.to_string() + "CommonPrefix"), &obj.common_prefixes);
        NextKeyMarkerWriter::write_params(
                params, &(prefix.to_string() + "NextKeyMarker"), &obj.next_key_marker);
        BucketNameWriter::write_params(
                params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        DelimiterWriter::write_params(
                params, &(prefix.to_string() + "Delimiter"), &obj.delimiter);
        NextUploadIdMarkerWriter::write_params(
                params, &(prefix.to_string() + "NextUploadIdMarker"), &obj.next_upload_id_marker);
        PrefixWriter::write_params(
                params, &(prefix.to_string() + "Prefix"), &obj.prefix);
        MultipartUploadListWriter::write_params(
                params, &(prefix.to_string() + "MultipartUpload"), &obj.uploads);
        KeyMarkerWriter::write_params(
                params, &(prefix.to_string() + "KeyMarker"), &obj.key_marker);
        MaxUploadsWriter::write_params(
                params, &(prefix.to_string() + "MaxUploads"), &obj.max_uploads);
        EncodingTypeWriter::write_params(
                params, &(prefix.to_string() + "EncodingType"), &obj.encoding_type);
        IsTruncatedWriter::write_params(
                params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
    }
}

impl CompleteMultipartUploadOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<CompleteMultipartUploadOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CompleteMultipartUploadOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml(
                                    "x-amz-request-charged", stack));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-version-id" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml(
                                    "x-amz-version-id", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "Location" {
                obj.location = try!(LocationParser::parse_xml("Location", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml(
                                    "x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml(
                                    "x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            if current_name == "x-amz-expiration" {
                obj.expiration = try!(ExpirationParser::parse_xml("x-amz-expiration", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl CompleteMultipartUploadOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CompleteMultipartUploadOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RequestChargedWriter::write_params(
                params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        BucketNameWriter::write_params(
                params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        ObjectVersionIdWriter::write_params(
                params, &(prefix.to_string() + "x-amz-version-id"), &obj.version_id);
        ETagWriter::write_params(
                params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        LocationWriter::write_params(
                params, &(prefix.to_string() + "Location"), &obj.location);
        ObjectKeyWriter::write_params(
                params, &(prefix.to_string() + "Key"), &obj.key);
        ServerSideEncryptionWriter::write_params(
                params,
                &(prefix.to_string() + "x-amz-server-side-encryption"),
                &obj.server_side_encryption);
        SSEKMSKeyIdWriter::write_params(
                params,
                &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"),
                &obj.ssekms_key_id);
        ExpirationWriter::write_params(
                params, &(prefix.to_string() + "x-amz-expiration"), &obj.expiration);
    }
}

impl GetObjectRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
    -> Result<GetObjectRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetObjectRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "response-content-encoding" {
                obj.response_content_encoding = Some(
                                    try!(ResponseContentEncodingParser::parse_xml(
                                            "response-content-encoding", stack)));
                continue;
            }
            if current_name == "response-content-language" {
                obj.response_content_language = Some(
                                    try!(ResponseContentLanguageParser::parse_xml(
                                            "response-content-language", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = Some(
                                    try!(SSECustomerAlgorithmParser::parse_xml(
                                            "x-amz-server-side-encryption-customer-algorithm",
                                            stack)));
                continue;
            }
            if current_name == "response-content-type" {
                obj.response_content_type = Some(
                                    try!(ResponseContentTypeParser::parse_xml(
                                            "response-content-type", stack)));
                continue;
            }
            if current_name == "If-Unmodified-Since" {
                obj.if_unmodified_since = Some(
                                    try!(IfUnmodifiedSinceParser::parse_xml(
                                            "If-Unmodified-Since", stack)));
                continue;
            }
            if current_name == "versionId" {
                obj.version_id = Some(try!(ObjectVersionIdParser::parse_xml("versionId", stack)));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(
                                    try!(RequestPayerParser::parse_xml(
                                            "x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "response-cache-control" {
                obj.response_cache_control = Some(
                                    try!(ResponseCacheControlParser::parse_xml(
                                            "response-cache-control", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key" {
                obj.sse_customer_key = Some(
                                    try!(SSECustomerKeyParser::parse_xml(
                                            "x-amz-server-side-encryption-customer-key", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "If-None-Match" {
                obj.if_none_match = Some(try!(IfNoneMatchParser::parse_xml(
                                            "If-None-Match", stack)));
                continue;
            }
            if current_name == "response-content-disposition" {
                obj.response_content_disposition = Some(
                                    try!(ResponseContentDispositionParser::parse_xml(
                                            "response-content-disposition", stack)));
                continue;
            }
            if current_name == "Range" {
                obj.range = Some(try!(RangeParser::parse_xml("Range", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "If-Match" {
                obj.if_match = Some(try!(IfMatchParser::parse_xml("If-Match", stack)));
                continue;
            }
            if current_name == "response-expires" {
                obj.response_expires = Some(try!(ResponseExpiresParser::parse_xml(
                                            "response-expires", stack)));
                continue;
            }
            if current_name == "If-Modified-Since" {
                obj.if_modified_since = Some(try!(IfModifiedSinceParser::parse_xml(
                                            "If-Modified-Since", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = Some(try!(SSECustomerKeyMD5Parser::parse_xml(
                                            "x-amz-server-side-encryption-customer-key-MD5",
                                            stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl GetObjectRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetObjectRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.response_content_encoding {
            ResponseContentEncodingWriter::write_params(
                        params, &(prefix.to_string() + "response-content-encoding"), obj);
        }
        if let Some(ref obj) = obj.response_content_language {
            ResponseContentLanguageWriter::write_params(
                        params, &(prefix.to_string() + "response-content-language"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_algorithm {
            SSECustomerAlgorithmWriter::write_params(
                        params,
                        &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"),
                        obj);
        }
        if let Some(ref obj) = obj.response_content_type {
            ResponseContentTypeWriter::write_params(
                        params, &(prefix.to_string() + "response-content-type"), obj);
        }
        if let Some(ref obj) = obj.if_unmodified_since {
            IfUnmodifiedSinceWriter::write_params(
                        params, &(prefix.to_string() + "If-Unmodified-Since"), obj);
        }
        if let Some(ref obj) = obj.version_id {
            ObjectVersionIdWriter::write_params(
                        params, &(prefix.to_string() + "versionId"), obj);
        }
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(
                        params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        if let Some(ref obj) = obj.response_cache_control {
            ResponseCacheControlWriter::write_params(
                        params, &(prefix.to_string() + "response-cache-control"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key {
            SSECustomerKeyWriter::write_params(
                        params,
                        &(prefix.to_string() + "x-amz-server-side-encryption-customer-key"),
                        obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.if_none_match {
            IfNoneMatchWriter::write_params(params, &(prefix.to_string() + "If-None-Match"), obj);
        }
        if let Some(ref obj) = obj.response_content_disposition {
            ResponseContentDispositionWriter::write_params(
                        params, &(prefix.to_string() + "response-content-disposition"), obj);
        }
        if let Some(ref obj) = obj.range {
            RangeWriter::write_params(params, &(prefix.to_string() + "Range"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        if let Some(ref obj) = obj.if_match {
            IfMatchWriter::write_params(params, &(prefix.to_string() + "If-Match"), obj);
        }
        if let Some(ref obj) = obj.response_expires {
            ResponseExpiresWriter::write_params(
                        params, &(prefix.to_string() + "response-expires"), obj);
        }
        if let Some(ref obj) = obj.if_modified_since {
            IfModifiedSinceWriter::write_params(
                        params, &(prefix.to_string() + "If-Modified-Since"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key_md5 {
            SSECustomerKeyMD5Writer::write_params(
                        params,
                        &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"),
                        obj);
        }
    }
}

impl PutObjectOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<PutObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml(
                        "x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml(
                        "x-amz-request-charged", stack));
                continue;
            }
            if current_name == "x-amz-version-id" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml(
                        "x-amz-version-id", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "x-amz-expiration" {
                obj.expiration = try!(ExpirationParser::parse_xml("x-amz-expiration", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml(
                        "x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml(
                        "x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml(
                        "x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl PutObjectOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &PutObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        SSECustomerAlgorithmWriter::write_params(
                params,
                &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"),
                &obj.sse_customer_algorithm);
        RequestChargedWriter::write_params(
                params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        ObjectVersionIdWriter::write_params(
                params, &(prefix.to_string() + "x-amz-version-id"), &obj.version_id);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        ExpirationWriter::write_params(
                params, &(prefix.to_string() + "x-amz-expiration"), &obj.expiration);
        ServerSideEncryptionWriter::write_params(
                params,
                &(prefix.to_string() + "x-amz-server-side-encryption"),
                &obj.server_side_encryption);
        SSECustomerKeyMD5Writer::write_params(
            params,
            &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"),
            &obj.sse_customer_key_md5);
        SSEKMSKeyIdWriter::write_params(
            params,
            &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"),
            &obj.ssekms_key_id);
    }
}

impl MaxUploadsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<MaxUploads, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl MaxUploadsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MaxUploads) {
        params.put(name, &obj.to_string());
    }
}

impl ExpiresParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<Expires, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ExpiresWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Expires) {
        params.put(name, obj);
    }
}

impl ListObjectsOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListObjectsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListObjectsOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Name" {
                obj.name = try!(BucketNameParser::parse_xml("Name", stack));
                continue;
            }
            // Version 1
            if current_name == "NextMarker" {
                obj.next_marker = try!(NextMarkerParser::parse_xml("NextMarker", stack));
                continue;
            }
            if current_name == "Delimiter" {
                obj.delimiter = try!(DelimiterParser::parse_xml("Delimiter", stack));
                continue;
            }
            if current_name == "MaxKeys" {
                obj.max_keys = try!(MaxKeysParser::parse_xml("MaxKeys", stack));
                continue;
            }
            if current_name == "Prefix" {
                obj.prefix = try!(PrefixParser::parse_xml("Prefix", stack));
                continue;
            }
            // Version 1
            if current_name == "Marker" {
                obj.marker = try!(MarkerParser::parse_xml("Marker", stack));
                continue;
            }
            if current_name == "EncodingType" {
                obj.encoding_type = try!(EncodingTypeParser::parse_xml("EncodingType", stack));
                continue;
            }
            if current_name == "IsTruncated" {
                obj.is_truncated = try!(IsTruncatedParser::parse_xml("IsTruncated", stack));
                continue;
            }
            if current_name == "Contents" {
                obj.contents = try!(ObjectMetadataListParser::parse_xml("Contents", stack));
                continue;
            }
            if current_name == "CommonPrefix" {
                obj.common_prefixes = try!(CommonPrefixListParser::parse_xml("CommonPrefix", stack));
                continue;
            }
            // Version 2
            if current_name == "KeyCount" {
                obj.key_count = try!(KeyCountParser::parse_xml("KeyCount", stack));
                continue;
            }
            // Version 2
            if current_name == "ContinuationToken" {
                obj.continuation_token = try!(ContinuationTokenParser::parse_xml("ContinuationToken", stack));
                continue;
            }
            // Version 2
            if current_name == "NextContinuationToken" {
                obj.next_continuation_token = try!(ContinuationTokenParser::parse_xml("NextContinuationToken", stack));
                continue;
            }
            // Version 2
            if current_name == "StartAfter" {
                obj.start_after = try!(StartAfterParser::parse_xml("StartAfter", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ListObjectsOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ListObjectsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
        NextMarkerWriter::write_params(params, &(prefix.to_string() + "NextMarker"), &obj.next_marker);
        DelimiterWriter::write_params(params, &(prefix.to_string() + "Delimiter"), &obj.delimiter);
        MaxKeysWriter::write_params(params, &(prefix.to_string() + "MaxKeys"), &obj.max_keys);
        PrefixWriter::write_params(params, &(prefix.to_string() + "Prefix"), &obj.prefix);
        MarkerWriter::write_params(params, &(prefix.to_string() + "Marker"), &obj.marker);
        EncodingTypeWriter::write_params(params, &(prefix.to_string() + "EncodingType"), &obj.encoding_type);
        IsTruncatedWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
        ObjectMetadataListWriter::write_params(params, &(prefix.to_string() + "Contents"), &obj.contents);
        CommonPrefixListWriter::write_params(params, &(prefix.to_string() + "CommonPrefix"), &obj.common_prefixes);
    }
}

impl ObjectMetadataListParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectMetadataList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == tag_name {
            obj.push(try!(ObjectMetadataParser::parse_xml(tag_name, stack)));
        }
        Ok(obj)
    }
}

impl ObjectMetadataListWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectMetadataList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            ObjectMetadataWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

impl ObjectMetadataParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectMetadata, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ObjectMetadata::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LastModified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "StorageClass" {
                obj.storage_class = try!(ObjectStorageClassParser::parse_xml("StorageClass", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            if current_name == "Size" {
                obj.size = try!(SizeParser::parse_xml("Size", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ObjectMetadataWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectMetadata) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        ObjectStorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        SizeWriter::write_params(params, &(prefix.to_string() + "Size"), &obj.size);
    }
}
