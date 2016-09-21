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
#![allow(dead_code)]
use std::str::FromStr;
use std::str;

use aws::common::params::{Params, ServiceParams};
use aws::common::xmlutil::*;
use aws::common::common::*;
use aws::errors::http::*;
use aws::errors::s3::*;
use aws::s3::header::*;
use aws::s3::bucket::*;
use aws::s3::acl::*;
use aws::s3::grant::*;
use aws::s3::writeparse::*;

pub type TagSet = Vec<Tag>;

pub type Parts = Vec<Part>;

pub type PartNumber = i32;

pub type UploadIdMarker = String;

pub type NextUploadIdMarker = String;

pub type MultipartUploadList = Vec<MultipartUpload>;

pub type PartNumberMarker = i32;

pub type NextPartNumberMarker = i32;

pub type MaxParts = i32;

pub type MaxUploads = i32;

pub type Expires = String;

pub type ObjectMetadataList = Vec<ObjectMetadata>;

pub type ObjectIdentifierList = Vec<ObjectIdentifier>;

pub type ObjectKey = String;

pub type ObjectVersionStorageClass = String;

pub type ObjectVersionId = String;

pub type DeleteMarkerVersionId = String;

pub type DeletedObjects = Vec<DeletedObject>;

pub type ObjectStorageClass = String;

pub type DeleteMarkers = Vec<DeleteMarkerEntry>;

pub type NextVersionIdMarker = String;

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

/// Parse `MultipartUploadListOutput` from XML
pub struct MultipartUploadListOutputParser;

/// Write `MultipartUploadListOutput` contents to a `SignedRequest`
pub struct MultipartUploadListOutputWriter;

/// Parse `MultipartUploadListPartsOutput` from XML
pub struct MultipartUploadListPartsOutputParser;

/// Write `MultipartUploadListPartsOutput` contents to a `SignedRequest`
pub struct MultipartUploadListPartsOutputWriter;

/// Parse `MultipartUploadListPartsRequest` from XML
pub struct MultipartUploadListPartsRequestParser;

/// Write `MultipartUploadListPartsRequest` contents to a `SignedRequest`
pub struct MultipartUploadListPartsRequestWriter;

/// Parse `MultipartUploadCompleteOutput` from XML
pub struct MultipartUploadCompleteOutputParser;

/// Write `MultipartUploadCompleteOutput` contents to a `SignedRequest`
pub struct MultipartUploadCompleteOutputWriter;

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

/// Parse `NextPartNumberMarker` from XML
pub struct NextPartNumberMarkerParser;

/// Write `NextPartNumberMarker` contents to a `SignedRequest`
pub struct NextPartNumberMarkerWriter;

/// Parse `MaxParts` from XML
pub struct MaxPartsParser;

/// Write `MaxParts` contents to a `SignedRequest`
pub struct MaxPartsWriter;

/// Parse `PartNumberMarker` from XML
pub struct PartNumberMarkerParser;

/// Write `PartNumberMarker` contents to a `SignedRequest`
pub struct PartNumberMarkerWriter;

/// Parse `Parts` from XML
pub struct PartsParser;

/// Write `Parts` contents to a `SignedRequest`
pub struct PartsWriter;

/// Write `MultipartUploadAbortOutput` contents to a `SignedRequest`
pub struct MultipartUploadAbortOutputWriter;

/// Parse `MultipartUploadAbortRequest` from XML
pub struct MultipartUploadAbortRequestParser;

/// Write `MultipartUploadAbortRequest` contents to a `SignedRequest`
pub struct MultipartUploadAbortRequestWriter;

/// Parse `MultipartUploadListRequest` from XML
pub struct MultipartUploadListRequestParser;

/// Write `MultipartUploadListRequest` contents to a `SignedRequest`
pub struct MultipartUploadListRequestWriter;

/// Parse `ObjectIdentifierList` from XML
pub struct ObjectIdentifierListParser;

/// Write `ObjectIdentifierList` contents to a `SignedRequest`
pub struct ObjectIdentifierListWriter;

/// Parse `Delete` from XML
pub struct DeleteParser;

/// Write `Delete` contents to a `SignedRequest`
pub struct DeleteWriter;

/// Parse `ObjectIdentifier` from XML
pub struct ObjectIdentifierParser;

/// Write `ObjectIdentifier` contents to a `SignedRequest`
pub struct ObjectIdentifierWriter;

/// Parse `DeleteObjectsRequest` from XML
pub struct DeleteObjectsRequestParser;

/// Write `DeleteObjectsRequest` contents to a `SignedRequest`
pub struct DeleteObjectsRequestWriter;

/// Parse `ObjectVersion` from XML
pub struct ObjectVersionParser;

/// Write `ObjectVersion` contents to a `SignedRequest`
pub struct ObjectVersionWriter;

/// Parse `ObjectKey` from XML
pub struct ObjectKeyParser;

/// Write `ObjectKey` contents to a `SignedRequest`
pub struct ObjectKeyWriter;

/// Parse `ObjectVersionStorageClass` from XML
pub struct ObjectVersionStorageClassParser;

/// Write `ObjectVersionStorageClass` contents to a `SignedRequest`
pub struct ObjectVersionStorageClassWriter;

/// Parse `CopyObjectResult` from XML
pub struct CopyObjectResultParser;

/// Write `CopyObjectResult` contents to a `SignedRequest`
pub struct CopyObjectResultWriter;

/// Parse `ObjectVersionId` from XML
pub struct ObjectVersionIdParser;

/// Write `ObjectVersionId` contents to a `SignedRequest`
pub struct ObjectVersionIdWriter;

/// Parse `DeletedObject` from XML
pub struct DeletedObjectParser;

/// Write `DeletedObject` contents to a `SignedRequest`
pub struct DeletedObjectWriter;

/// Parse `DeleteMarkerVersionId` from XML
pub struct DeleteMarkerVersionIdParser;

/// Write `DeleteMarkerVersionId` contents to a `SignedRequest`
pub struct DeleteMarkerVersionIdWriter;

/// Parse `DeletedObjects` from XML
pub struct DeletedObjectsParser;

/// Write `DeletedObjects` contents to a `SignedRequest`
pub struct DeletedObjectsWriter;

/// Parse `DeleteObjectOutput` from XML
pub struct DeleteObjectOutputParser;

/// Write `DeleteObjectOutput` contents to a `SignedRequest`
pub struct DeleteObjectOutputWriter;

/// Parse `DeleteObjectRequest` from XML
pub struct DeleteObjectRequestParser;

/// Write `DeleteObjectRequest` contents to a `SignedRequest`
pub struct DeleteObjectRequestWriter;

/// Parse `RestoreRequest` from XML
pub struct RestoreRequestParser;

/// Write `RestoreRequest` contents to a `SignedRequest`
pub struct RestoreRequestWriter;

/// Parse `RestoreObjectRequest` from XML
pub struct RestoreObjectRequestParser;

/// Write `RestoreObjectRequest` contents to a `SignedRequest`
pub struct RestoreObjectRequestWriter;

/// Parse `RestoreObjectOutput` from XML
pub struct RestoreObjectOutputParser;

/// Write `RestoreObjectOutput` contents to a `SignedRequest`
pub struct RestoreObjectOutputWriter;

/// Parse `CopyObjectOutput` from XML
pub struct CopyObjectOutputParser;

/// Write `CopyObjectOutput` contents to a `SignedRequest`
pub struct CopyObjectOutputWriter;

/// Parse `HeadObjectOutput` from XML
pub struct HeadObjectOutputParser;

/// Write `HeadObjectOutput` contents to a `SignedRequest`
pub struct HeadObjectOutputWriter;

/// Parse `ObjectVersionList` from XML
pub struct ObjectVersionListParser;

/// Write `ObjectVersionList` contents to a `SignedRequest`
pub struct ObjectVersionListWriter;

/// Parse `ObjectStorageClass` from XML
pub struct ObjectStorageClassParser;

/// Write `ObjectStorageClass` contents to a `SignedRequest`
pub struct ObjectStorageClassWriter;

/// Parse `CopyObjectRequest` from XML
pub struct CopyObjectRequestParser;

/// Write `CopyObjectRequest` contents to a `SignedRequest`
pub struct CopyObjectRequestWriter;

/// Parse `DeleteObjectsOutput` from XML
pub struct DeleteObjectsOutputParser;

/// Write `DeleteObjectsOutput` contents to a `SignedRequest`
pub struct DeleteObjectsOutputWriter;

/// Parse `GetObjectOutput` from XML
pub struct GetObjectOutputParser;

/// Write `GetObjectOutput` contents to a `SignedRequest`
pub struct GetObjectOutputWriter;

/// Parse `MultipartUploadCreateRequest` from XML
pub struct MultipartUploadCreateRequestParser;

/// Write `MultipartUploadCreateRequest` contents to a `SignedRequest`
pub struct MultipartUploadCreateRequestWriter;

/// Parse `GetObjectTorrentRequest` from XML
pub struct GetObjectTorrentRequestParser;

/// Write `GetObjectTorrentRequest` contents to a `SignedRequest`
pub struct GetObjectTorrentRequestWriter;

/// Parse `DeleteMarkerEntry` from XML
pub struct DeleteMarkerEntryParser;

/// Write `DeleteMarkerEntry` contents to a `SignedRequest`
pub struct DeleteMarkerEntryWriter;

/// Parse `DeleteMarkers` from XML
pub struct DeleteMarkersParser;

/// Write `DeleteMarkers` contents to a `SignedRequest`
pub struct DeleteMarkersWriter;

/// Parse `ListObjectsRequest` from XML
pub struct ListObjectsRequestParser;

/// Write `ListObjectsRequest` contents to a `SignedRequest`
pub struct ListObjectsRequestWriter;

/// Parse `NextVersionIdMarker` from XML
pub struct NextVersionIdMarkerParser;

/// Write `NextVersionIdMarker` contents to a `SignedRequest`
pub struct NextVersionIdMarkerWriter;

/// Parse `ListVersionsResult` from XML
pub struct ListVersionsResultParser;

/// Write `ListVersionsResult` contents to a `SignedRequest`
pub struct ListVersionsResultWriter;

/// Parse `ListObjectVersionsRequest` from XML
pub struct ListObjectVersionsRequestParser;

/// Write `ListObjectVersionsRequest` contents to a `SignedRequest`
pub struct ListObjectVersionsRequestWriter;

/// Parse `MultipartUploadCreateOutput` from XML
pub struct MultipartUploadCreateOutputParser;

/// Write `MultipartUploadCreateOutput` contents to a `SignedRequest`
pub struct MultipartUploadCreateOutputWriter;

/// Write `HeadObjectRequest` contents to a `SignedRequest`
struct HeadObjectRequestWriter;

/// Parse `HeadObjectRequest` from XML
struct HeadObjectRequestParser;

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ObjectVersion {
    /// Date and time the object was last modified.
    pub last_modified: LastModified,
    /// Version ID of an object.
    pub version_id: ObjectVersionId,
    pub e_tag: ETag,
    /// The class of storage used to store the object.
    pub storage_class: ObjectVersionStorageClass,
    /// The object key.
    pub key: ObjectKey,
    pub owner: Owner,
    /// Specifies whether the object is (true) or is not (false) the latest version of
    /// an object.
    pub is_latest: IsLatest,
    /// Size in bytes of the object.
    pub size: Size,
}

/// `ObjectMetadata` used for `Contents` for ListObjectsOutput
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ObjectMetadata {
    pub last_modified: LastModified,
    pub e_tag: ETag,
    /// The class of storage used to store the object.
    pub storage_class: ObjectStorageClass,
    pub key: ObjectKey,
    pub owner: Owner,
    pub size: Size,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct Delete {
    pub objects: ObjectIdentifierList,
    /// Element to enable quiet mode for the request. When you add this element, you
    /// must set its value to true.
    pub quiet: Option<Quiet>,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct RestoreObjectOutput {
    pub request_charged: RequestCharged,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct RestoreObjectRequest {
    pub version_id: Option<ObjectVersionId>,
    pub restore_request: Option<RestoreRequest>,
    pub bucket: BucketName,
    pub request_payer: Option<RequestPayer>,
    pub key: ObjectKey,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct RestoreRequest {
    /// Lifetime of the active copy in days
    pub days: Days,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteObjectOutput {
    /// Returns the version ID of the delete marker created as a result of the DELETE
    /// operation.
    pub version_id: ObjectVersionId,
    pub request_charged: RequestCharged,
    /// Specifies whether the versioned object that was permanently deleted was (true)
    /// or was not (false) a delete marker.
    pub delete_marker: DeleteMarker,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteObjectsOutput {
    pub deleted: DeletedObjects,
    pub errors: Errors,
    pub request_charged: RequestCharged,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeletedObject {
    pub version_id: ObjectVersionId,
    pub delete_marker_version_id: DeleteMarkerVersionId,
    pub key: ObjectKey,
    pub delete_marker: DeleteMarker,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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
//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct TopicConfiguration {
    pub id: Option<NotificationId>,
    /// Amazon SNS topic ARN to which Amazon S3 will publish a message when it detects
    /// events of specified type.
    pub topic_arn: TopicArn,
    pub events: EventList,
}

/// Container for specifying an configuration when you want Amazon S3 to publish
/// events to an Amazon Simple Queue Service (Amazon SQS) queue.
//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct QueueConfiguration {
    pub id: Option<NotificationId>,
    /// Amazon SQS queue ARN to which Amazon S3 will publish a message when it detects
    /// events of specified type.
    pub queue_arn: QueueArn,
    pub events: EventList,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ObjectIdentifier {
    /// VersionId for the specific version of the object to delete.
    pub version_id: Option<ObjectVersionId>,
    /// Key name of the object to delete.
    pub key: ObjectKey,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct MultipartUploadCreateRequest {
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

//#[derive(Debug, Default)]
// NOTE: &'a [u8] may require a custom RustcDecodable

/// NB: MultipartUploadCompleteRequest is *not* JSON decodable without implementing a custom to_json trait
/// because of Option<&'a [u8]>.
///
#[derive(Debug, Default, RustcEncodable)]
pub struct MultipartUploadCompleteRequest <'a> {
    pub multipart_upload: Option<&'a [u8]>,
    pub upload_id: MultipartUploadId,
    pub bucket: BucketName,
    pub request_payer: Option<RequestPayer>,
    pub key: ObjectKey,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct MultipartUploadAbortRequest {
    pub upload_id: MultipartUploadId,
    pub bucket: BucketName,
    pub request_payer: Option<RequestPayer>,
    pub key: ObjectKey,
}

//#[derive(Debug, Default)]
// NOTE: &'a [u8] may require a custom RustcDecodable

/// NB: MultipartUploadPartRequest is *not* JSON decodable without implementing a custom to_json trait
/// because of Option<&'a [u8]>.
///
#[derive(Debug, Default, RustcEncodable)]
pub struct MultipartUploadPartRequest <'a> {
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

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct MultipartUploadListPartsOutput {
    /// Identifies who initiated the multipart upload.
    pub initiator: Initiator,
    /// Name of the bucket to which the multipart upload was initiated.
    pub bucket: BucketName,
    /// When a list is truncated, this element specifies the last part in the list, as
    /// well as the value to use for the part-number-marker request parameter in a
    /// subsequent request.
    pub next_part_number_marker: NextPartNumberMarker,
    pub parts: Parts,
    /// Upload ID identifying the multipart upload whose parts are being listed.
    pub upload_id: MultipartUploadId,
    /// The class of storage used to store the object.
    pub storage_class: StorageClass,
    /// Object key for which the multipart upload was initiated.
    pub key: ObjectKey,
    pub request_charged: RequestCharged,
    pub owner: Owner,
    /// Maximum number of parts that were allowed in the response.
    pub max_parts: MaxParts,
    /// Indicates whether the returned list of parts is truncated.
    pub is_truncated: IsTruncated,
    /// Part number after which listing begins.
    pub part_number_marker: PartNumberMarker,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct MultipartUploadListPartsRequest {
    pub request_payer: Option<RequestPayer>,
    pub bucket: BucketName,
    /// Upload ID identifying the multipart upload whose parts are being listed.
    pub upload_id: MultipartUploadId,
    pub key: ObjectKey,
    /// Sets the maximum number of parts to return.
    pub max_parts: Option<MaxParts>,
    /// Specifies the part after which listing should begin. Only parts with higher
    /// part numbers will be listed.
    pub part_number_marker: Option<PartNumberMarker>,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct MultipartUploadListRequest {
    /// Together with key-marker, specifies the multipart upload after which listing
    /// should begin. If key-marker is not specified, the upload-id-marker parameter
    /// is ignored.
    pub upload_id_marker: Option<UploadIdMarker>,
    pub bucket: BucketName,
    /// Character you use to group keys.
    pub delimiter: Option<Delimiter>,
    /// Lists in-progress uploads only for those keys that begin with the specified
    /// prefix.
    pub prefix: Option<Prefix>,
    /// Together with upload-id-marker, this parameter specifies the multipart upload
    /// after which listing should begin.
    pub key_marker: Option<KeyMarker>,
    /// Sets the maximum number of multipart uploads, from 1 to 1,000, to return in
    /// the response body. 1,000 is the maximum number of uploads that can be returned
    /// in a response.
    pub max_uploads: Option<MaxUploads>,
    pub encoding_type: Option<EncodingType>,
}

//#[derive(Debug, Default)]
// NOTE: &'a [u8] may require a custom RustcDecodable

/// NB: PutObjectRequest is *not* JSON decodable without implementing a custom to_json trait
/// because of Option<&'a [u8]>.
///
#[derive(Debug, Default, RustcEncodable)]
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

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct Initiator {
    /// Name of the Principal.
    pub display_name: DisplayName,
    /// If the principal is an AWS account, it provides the Canonical User ID. If the
    /// principal is an IAM User, it provides a user ARN value.
    pub id: ID,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct GetObjectTorrentRequest {
    pub bucket: BucketName,
    pub request_payer: Option<RequestPayer>,
    pub key: ObjectKey,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct ErrorDocument {
    /// The object key name to use when a 4XX class error occurs.
    pub key: ObjectKey,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct IndexDocument {
    /// A suffix that is appended to a request that is for a directory on the website
    /// endpoint (e.g. if the suffix is index.html and you make a request to
    /// samplebucket/images/ the data that is returned will be for the object with the
    /// key name images/index.html) The suffix must not be empty and must not include
    /// a slash character.
    pub suffix: Suffix,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct RedirectAllRequestsTo {
    /// Name of the host where requests will be redirected.
    pub host_name: HostName,
    /// Protocol to use (http, https) when redirecting requests. The default is the
    /// protocol that is used in the original request.
    pub protocol: Option<Protocol>,
}

/// Container for specifying the AWS Lambda notification configuration.
//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct LambdaFunctionConfiguration {
    /// Lambda cloud function ARN that Amazon S3 can invoke when it detects events of
    /// the specified type.
    pub lambda_function_arn: LambdaFunctionArn,
    pub id: Option<NotificationId>,
    pub events: EventList,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct Tag {
    /// Value of the tag.
    pub value: Value,
    /// Name of the tag.
    pub key: ObjectKey,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct CopyObjectResult {
    pub last_modified: LastModified,
    pub e_tag: ETag,
}

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct MultipartUploadListOutput {
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

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct MultipartUploadCreateOutput {
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

//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct MultipartUploadCompleteOutput {
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

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct MultipartUploadAbortOutput {
    pub request_charged: RequestCharged,
}

/// Specifies when noncurrent object versions expire. Upon expiration, Amazon S3
/// permanently deletes the noncurrent object versions. You set this lifecycle
/// configuration action on a bucket that has versioning enabled (or suspended) to
/// request that Amazon S3 delete noncurrent object versions at a specific period
/// in the object's lifetime.
//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct NoncurrentVersionExpiration {
    /// Specifies the number of days an object is noncurrent before Amazon S3 can
    /// perform the associated action. For information about the noncurrent days
    /// calculations, see [How Amazon S3 Calculates When an Object Became
    /// Noncurrent](/AmazonS3/latest/dev/s3-access-control.html) in the Amazon Simple
    /// Storage Service Developer Guide.
    pub noncurrent_days: Days,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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
//#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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
//#[derive(Debug, Default)]
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct DeleteObjectsRequest {
    /// The concatenation of the authentication device's serial number, a space, and
    /// the value that is displayed on your authentication device.
    pub mfa: Option<MFA>,
    pub bucket: BucketName,
    pub request_payer: Option<RequestPayer>,
    pub delete: Delete,
}

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
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

#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct HeadObjectRequest {
    /// Specifies the algorithm to use to when encrypting the object (e.g., AES256).
    pub sse_customer_algorithm: Option<SSECustomerAlgorithm>,
    /// Specifies the customer-provided encryption key for Amazon S3 to use in
    /// encrypting data. This value is used to store the object and then it is
    /// discarded; Amazon does not store the encryption key. The key must be
    /// appropriate for use with the algorithm specified in the x-amz-server-side-
    /// encryption-customer-algorithm header.
    pub sse_customer_key: Option<SSECustomerKey>,
    /// Return the object only if it has not been modified since the specified time,
    /// otherwise return a 412 (precondition failed).
    pub if_unmodified_since: Option<IfUnmodifiedSince>,
    /// VersionId used to reference a specific version of the object.
    pub version_id: Option<ObjectVersionId>,
    pub request_payer: Option<RequestPayer>,
    pub bucket: BucketName,
    /// Return the object only if its entity tag (ETag) is different from the one
    /// specified, otherwise return a 304 (not modified).
    pub if_none_match: Option<IfNoneMatch>,
    /// Downloads the specified range bytes of an object. For more information about
    /// the HTTP Range header, go to
    /// http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35.
    pub range: Option<Range>,
    pub key: ObjectKey,
    /// Return the object only if its entity tag (ETag) is the same as the one
    /// specified, otherwise return a 412 (precondition failed).
    pub if_match: Option<IfMatch>,
    /// Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321.
    /// Amazon S3 uses this header for a message integrity check to ensure the
    /// encryption key was transmitted without error.
    pub sse_customer_key_md5: Option<SSECustomerKeyMD5>,
    /// Return the object only if it has been modified since the specified time,
    /// otherwise return a 304 (not modified).
    pub if_modified_since: Option<IfModifiedSince>,
}

// functions below...

/// Writes out XML with all the parts in it for S3 to complete.
pub fn multipart_upload_finish_xml(parts: &[String]) -> Result<Vec<u8>, S3Error> {
    if parts.len() < 1 {
        return Err(S3Error::new("Can't finish upload. NO parts!"));
    }
    let mut response = String::from("<CompleteMultipartUpload>");

    let mut part_number = 1;
    for etag in parts {
        response = response + &format!("<Part><PartNumber>{}</PartNumber><ETag>{}</ETag></Part>", part_number, etag);
        part_number += 1;
    }

    response = response + "</CompleteMultipartUpload>";

    Ok(response.into_bytes())
}

// Impls below...

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

impl MultipartUploadListOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<MultipartUploadListOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = MultipartUploadListOutput::default();
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

impl MultipartUploadListOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MultipartUploadListOutput) {
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

impl MultipartUploadCompleteOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<MultipartUploadCompleteOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = MultipartUploadCompleteOutput::default();
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

impl MultipartUploadCompleteOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MultipartUploadCompleteOutput) {
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

impl MultipartUploadListPartsOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MultipartUploadListPartsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = MultipartUploadListPartsOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Initiator" {
                obj.initiator = try!(InitiatorParser::parse_xml("Initiator", stack));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "NextPartNumberMarker" {
                obj.next_part_number_marker = try!(NextPartNumberMarkerParser::parse_xml("NextPartNumberMarker", stack));
                continue;
            }
            if current_name == "Part" {
                obj.parts = try!(PartsParser::parse_xml("Part", stack));
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
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            if current_name == "MaxParts" {
                obj.max_parts = try!(MaxPartsParser::parse_xml("MaxParts", stack));
                continue;
            }
            if current_name == "IsTruncated" {
                obj.is_truncated = try!(IsTruncatedParser::parse_xml("IsTruncated", stack));
                continue;
            }
            if current_name == "PartNumberMarker" {
                obj.part_number_marker = try!(PartNumberMarkerParser::parse_xml("PartNumberMarker", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl MultipartUploadListPartsOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MultipartUploadListPartsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        InitiatorWriter::write_params(params, &(prefix.to_string() + "Initiator"), &obj.initiator);
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        NextPartNumberMarkerWriter::write_params(params, &(prefix.to_string() + "NextPartNumberMarker"), &obj.next_part_number_marker);
        PartsWriter::write_params(params, &(prefix.to_string() + "Part"), &obj.parts);
        MultipartUploadIdWriter::write_params(params, &(prefix.to_string() + "UploadId"), &obj.upload_id);
        StorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        MaxPartsWriter::write_params(params, &(prefix.to_string() + "MaxParts"), &obj.max_parts);
        IsTruncatedWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
        PartNumberMarkerWriter::write_params(params, &(prefix.to_string() + "PartNumberMarker"), &obj.part_number_marker);
    }
}

impl MultipartUploadListPartsRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MultipartUploadListPartsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = MultipartUploadListPartsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "uploadId" {
                obj.upload_id = try!(MultipartUploadIdParser::parse_xml("uploadId", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "max-parts" {
                obj.max_parts = Some(try!(MaxPartsParser::parse_xml("max-parts", stack)));
                continue;
            }
            if current_name == "part-number-marker" {
                obj.part_number_marker = Some(try!(PartNumberMarkerParser::parse_xml("part-number-marker", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl MultipartUploadListPartsRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MultipartUploadListPartsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        MultipartUploadIdWriter::write_params(params, &(prefix.to_string() + "uploadId"), &obj.upload_id);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        if let Some(ref obj) = obj.max_parts {
            MaxPartsWriter::write_params(params, &(prefix.to_string() + "max-parts"), obj);
        }
        if let Some(ref obj) = obj.part_number_marker {
            PartNumberMarkerWriter::write_params(params, &(prefix.to_string() + "part-number-marker"), obj);
        }
    }
}

impl NextPartNumberMarkerParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NextPartNumberMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl NextPartNumberMarkerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &NextPartNumberMarker) {
        params.put(name, &obj.to_string());
    }
}

impl MaxPartsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MaxParts, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl MaxPartsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MaxParts) {
        params.put(name, &obj.to_string());
    }
}

impl PartNumberMarkerParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PartNumberMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl PartNumberMarkerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &PartNumberMarker) {
        params.put(name, &obj.to_string());
    }
}

impl PartsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Parts, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Part" {
            obj.push(try!(PartParser::parse_xml("Part", stack)));
        }
        Ok(obj)
    }
}

impl PartsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Parts) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            PartWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

impl MultipartUploadAbortOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MultipartUploadAbortOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
    }
}

impl MultipartUploadAbortRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MultipartUploadAbortRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = MultipartUploadAbortRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "uploadId" {
                obj.upload_id = try!(MultipartUploadIdParser::parse_xml("uploadId", stack));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
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

impl MultipartUploadAbortRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MultipartUploadAbortRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        MultipartUploadIdWriter::write_params(params, &(prefix.to_string() + "uploadId"), &obj.upload_id);
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}

impl MultipartUploadListRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MultipartUploadListRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = MultipartUploadListRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "upload-id-marker" {
                obj.upload_id_marker = Some(try!(UploadIdMarkerParser::parse_xml("upload-id-marker", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "delimiter" {
                obj.delimiter = Some(try!(DelimiterParser::parse_xml("delimiter", stack)));
                continue;
            }
            if current_name == "prefix" {
                obj.prefix = Some(try!(PrefixParser::parse_xml("prefix", stack)));
                continue;
            }
            if current_name == "key-marker" {
                obj.key_marker = Some(try!(KeyMarkerParser::parse_xml("key-marker", stack)));
                continue;
            }
            if current_name == "max-uploads" {
                obj.max_uploads = Some(try!(MaxUploadsParser::parse_xml("max-uploads", stack)));
                continue;
            }
            if current_name == "encoding-type" {
                obj.encoding_type = Some(try!(EncodingTypeParser::parse_xml("encoding-type", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl MultipartUploadListRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MultipartUploadListRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.upload_id_marker {
            UploadIdMarkerWriter::write_params(params, &(prefix.to_string() + "upload-id-marker"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.delimiter {
            DelimiterWriter::write_params(params, &(prefix.to_string() + "delimiter"), obj);
        }
        if let Some(ref obj) = obj.prefix {
            PrefixWriter::write_params(params, &(prefix.to_string() + "prefix"), obj);
        }
        if let Some(ref obj) = obj.key_marker {
            KeyMarkerWriter::write_params(params, &(prefix.to_string() + "key-marker"), obj);
        }
        if let Some(ref obj) = obj.max_uploads {
            MaxUploadsWriter::write_params(params, &(prefix.to_string() + "max-uploads"), obj);
        }
        if let Some(ref obj) = obj.encoding_type {
            EncodingTypeWriter::write_params(params, &(prefix.to_string() + "encoding-type"), obj);
        }
    }
}

impl ObjectIdentifierListParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectIdentifierList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "ObjectIdentifier" {
            obj.push(try!(ObjectIdentifierParser::parse_xml("ObjectIdentifier", stack)));
        }
        Ok(obj)
    }
}

impl ObjectIdentifierListWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectIdentifierList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            ObjectIdentifierWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

impl DeleteParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Delete, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Delete::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "ObjectIdentifier" {
                obj.objects = try!(ObjectIdentifierListParser::parse_xml("ObjectIdentifier", stack));
                continue;
            }
            if current_name == "Quiet" {
                obj.quiet = Some(try!(QuietParser::parse_xml("Quiet", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl DeleteWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Delete) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ObjectIdentifierListWriter::write_params(params, &(prefix.to_string() + "ObjectIdentifier"), &obj.objects);
        if let Some(ref obj) = obj.quiet {
            QuietWriter::write_params(params, &(prefix.to_string() + "Quiet"), obj);
        }
    }
}

impl ObjectIdentifierParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectIdentifier, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ObjectIdentifier::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "VersionId" {
                obj.version_id = Some(try!(ObjectVersionIdParser::parse_xml("VersionId", stack)));
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

impl ObjectIdentifierWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectIdentifier) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.version_id {
            ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}

impl DeleteObjectsRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteObjectsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteObjectsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-mfa" {
                obj.mfa = Some(try!(MFAParser::parse_xml("x-amz-mfa", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Delete" {
                obj.delete = try!(DeleteParser::parse_xml("Delete", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl DeleteObjectsRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteObjectsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.mfa {
            MFAWriter::write_params(params, &(prefix.to_string() + "x-amz-mfa"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        DeleteWriter::write_params(params, &(prefix.to_string() + "Delete"), &obj.delete);
    }
}

impl ObjectVersionParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectVersion, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ObjectVersion::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LastModified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
                continue;
            }
            if current_name == "VersionId" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("VersionId", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "StorageClass" {
                obj.storage_class = try!(ObjectVersionStorageClassParser::parse_xml("StorageClass", stack));
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
            if current_name == "IsLatest" {
                obj.is_latest = try!(IsLatestParser::parse_xml("IsLatest", stack));
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

impl ObjectVersionWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectVersion) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        ObjectVersionStorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        IsLatestWriter::write_params(params, &(prefix.to_string() + "IsLatest"), &obj.is_latest);
        SizeWriter::write_params(params, &(prefix.to_string() + "Size"), &obj.size);
    }
}

impl ObjectKeyParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectKey, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ObjectKeyWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectKey) {
        params.put(name, obj);
    }
}

impl ObjectVersionStorageClassParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectVersionStorageClass, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ObjectVersionStorageClassWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectVersionStorageClass) {
        params.put(name, obj);
    }
}

impl CopyObjectResultParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopyObjectResult, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CopyObjectResult::default();
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
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl CopyObjectResultWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CopyObjectResult) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
    }
}

impl ObjectVersionIdParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectVersionId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ObjectVersionIdWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectVersionId) {
        params.put(name, obj);
    }
}

impl DeleteMarkerVersionIdParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<DeleteMarkerVersionId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl DeleteMarkerVersionIdWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteMarkerVersionId) {
        params.put(name, obj);
    }
}

impl DeletedObjectParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeletedObject, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeletedObject::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "VersionId" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("VersionId", stack));
                continue;
            }
            if current_name == "DeleteMarkerVersionId" {
                obj.delete_marker_version_id = try!(DeleteMarkerVersionIdParser::parse_xml("DeleteMarkerVersionId", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "DeleteMarker" {
                obj.delete_marker = try!(DeleteMarkerParser::parse_xml("DeleteMarker", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl DeletedObjectWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeletedObject) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
        DeleteMarkerVersionIdWriter::write_params(params, &(prefix.to_string() + "DeleteMarkerVersionId"), &obj.delete_marker_version_id);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        DeleteMarkerWriter::write_params(params, &(prefix.to_string() + "DeleteMarker"), &obj.delete_marker);
    }
}

impl DeletedObjectsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeletedObjects, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "DeletedObject" {
            obj.push(try!(DeletedObjectParser::parse_xml("DeletedObject", stack)));
        }
        Ok(obj)
    }
}

impl DeletedObjectsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeletedObjects) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            DeletedObjectWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

impl DeleteObjectOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-version-id" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("x-amz-version-id", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "x-amz-delete-marker" {
                obj.delete_marker = try!(DeleteMarkerParser::parse_xml("x-amz-delete-marker", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl DeleteObjectOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "x-amz-version-id"), &obj.version_id);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        DeleteMarkerWriter::write_params(params, &(prefix.to_string() + "x-amz-delete-marker"), &obj.delete_marker);
    }
}

impl DeleteObjectRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteObjectRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteObjectRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-mfa" {
                obj.mfa = Some(try!(MFAParser::parse_xml("x-amz-mfa", stack)));
                continue;
            }
            if current_name == "versionId" {
                obj.version_id = Some(try!(ObjectVersionIdParser::parse_xml("versionId", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
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

impl DeleteObjectRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteObjectRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.mfa {
            MFAWriter::write_params(params, &(prefix.to_string() + "x-amz-mfa"), obj);
        }
        if let Some(ref obj) = obj.version_id {
            ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "versionId"), obj);
        }
        //BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        //ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}

impl RestoreRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RestoreRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = RestoreRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Days" {
                obj.days = try!(DaysParser::parse_xml("Days", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl RestoreRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &RestoreRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DaysWriter::write_params(params, &(prefix.to_string() + "Days"), &obj.days);
    }
}

impl RestoreObjectRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RestoreObjectRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = RestoreObjectRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "versionId" {
                obj.version_id = Some(try!(ObjectVersionIdParser::parse_xml("versionId", stack)));
                continue;
            }
            if current_name == "RestoreRequest" {
                obj.restore_request = Some(try!(RestoreRequestParser::parse_xml("RestoreRequest", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
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

impl RestoreObjectRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &RestoreObjectRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.version_id {
            ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "versionId"), obj);
        }
        if let Some(ref obj) = obj.restore_request {
            RestoreRequestWriter::write_params(params, &(prefix.to_string() + "RestoreRequest"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}

impl RestoreObjectOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RestoreObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = RestoreObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl RestoreObjectOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &RestoreObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
    }
}

impl CopyObjectOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopyObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CopyObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "x-amz-copy-source-version-id" {
                obj.copy_source_version_id = try!(CopySourceVersionIdParser::parse_xml("x-amz-copy-source-version-id", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "x-amz-expiration" {
                obj.expiration = try!(ExpirationParser::parse_xml("x-amz-expiration", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            if current_name == "CopyObjectResult" {
                obj.copy_object_result = try!(CopyObjectResultParser::parse_xml("CopyObjectResult", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl CopyObjectOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CopyObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), &obj.sse_customer_algorithm);
        CopySourceVersionIdWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-version-id"), &obj.copy_source_version_id);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        ExpirationWriter::write_params(params, &(prefix.to_string() + "x-amz-expiration"), &obj.expiration);
        SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), &obj.sse_customer_key_md5);
        CopyObjectResultWriter::write_params(params, &(prefix.to_string() + "CopyObjectResult"), &obj.copy_object_result);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
    }
}

impl HeadObjectOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<HeadObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = HeadObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Last-Modified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("Last-Modified", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "Content-Encoding" {
                obj.content_encoding = try!(ContentEncodingParser::parse_xml("Content-Encoding", stack));
                continue;
            }
            if current_name == "x-amz-replication-status" {
                obj.replication_status = try!(ReplicationStatusParser::parse_xml("x-amz-replication-status", stack));
                continue;
            }
            if current_name == "x-amz-storage-class" {
                obj.storage_class = try!(StorageClassParser::parse_xml("x-amz-storage-class", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            if current_name == "Content-Disposition" {
                obj.content_disposition = try!(ContentDispositionParser::parse_xml("Content-Disposition", stack));
                continue;
            }
            if current_name == "x-amz-meta-" {
                obj.metadata = try!(MetadataParser::parse_xml("x-amz-meta-", stack));
                continue;
            }
            if current_name == "accept-ranges" {
                obj.accept_ranges = try!(AcceptRangesParser::parse_xml("accept-ranges", stack));
                continue;
            }
            if current_name == "x-amz-website-redirect-location" {
                obj.website_redirect_location = try!(WebsiteRedirectLocationParser::parse_xml("x-amz-website-redirect-location", stack));
                continue;
            }
            if current_name == "Expires" {
                obj.expires = try!(ExpiresParser::parse_xml("Expires", stack));
                continue;
            }
            if current_name == "x-amz-delete-marker" {
                obj.delete_marker = try!(DeleteMarkerParser::parse_xml("x-amz-delete-marker", stack));
                continue;
            }
            if current_name == "Cache-Control" {
                obj.cache_control = try!(CacheControlParser::parse_xml("Cache-Control", stack));
                continue;
            }
            if current_name == "Content-Length" {
                obj.content_length = try!(ContentLengthParser::parse_xml("Content-Length", stack));
                continue;
            }
            if current_name == "x-amz-expiration" {
                obj.expiration = try!(ExpirationParser::parse_xml("x-amz-expiration", stack));
                continue;
            }
            if current_name == "x-amz-missing-meta" {
                obj.missing_meta = try!(MissingMetaParser::parse_xml("x-amz-missing-meta", stack));
                continue;
            }
            if current_name == "x-amz-restore" {
                obj.restore = try!(RestoreParser::parse_xml("x-amz-restore", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "Content-Type" {
                obj.content_type = try!(ContentTypeParser::parse_xml("Content-Type", stack));
                continue;
            }
            if current_name == "Content-Language" {
                obj.content_language = try!(ContentLanguageParser::parse_xml("Content-Language", stack));
                continue;
            }
            if current_name == "x-amz-version-id" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("x-amz-version-id", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl HeadObjectOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &HeadObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "Last-Modified"), &obj.last_modified);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        ContentEncodingWriter::write_params(params, &(prefix.to_string() + "Content-Encoding"), &obj.content_encoding);
        ReplicationStatusWriter::write_params(params, &(prefix.to_string() + "x-amz-replication-status"), &obj.replication_status);
        StorageClassWriter::write_params(params, &(prefix.to_string() + "x-amz-storage-class"), &obj.storage_class);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
        ContentDispositionWriter::write_params(params, &(prefix.to_string() + "Content-Disposition"), &obj.content_disposition);
        MetadataWriter::write_params(params, &(prefix.to_string() + "x-amz-meta-"), &obj.metadata);
        AcceptRangesWriter::write_params(params, &(prefix.to_string() + "accept-ranges"), &obj.accept_ranges);
        WebsiteRedirectLocationWriter::write_params(params, &(prefix.to_string() + "x-amz-website-redirect-location"), &obj.website_redirect_location);
        ExpiresWriter::write_params(params, &(prefix.to_string() + "Expires"), &obj.expires);
        DeleteMarkerWriter::write_params(params, &(prefix.to_string() + "x-amz-delete-marker"), &obj.delete_marker);
        CacheControlWriter::write_params(params, &(prefix.to_string() + "Cache-Control"), &obj.cache_control);
        ContentLengthWriter::write_params(params, &(prefix.to_string() + "Content-Length"), &obj.content_length);
        ExpirationWriter::write_params(params, &(prefix.to_string() + "x-amz-expiration"), &obj.expiration);
        MissingMetaWriter::write_params(params, &(prefix.to_string() + "x-amz-missing-meta"), &obj.missing_meta);
        RestoreWriter::write_params(params, &(prefix.to_string() + "x-amz-restore"), &obj.restore);
        SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), &obj.sse_customer_algorithm);
        ContentTypeWriter::write_params(params, &(prefix.to_string() + "Content-Type"), &obj.content_type);
        ContentLanguageWriter::write_params(params, &(prefix.to_string() + "Content-Language"), &obj.content_language);
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "x-amz-version-id"), &obj.version_id);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), &obj.sse_customer_key_md5);
    }
}

impl ObjectVersionListParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectVersionList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Version" {
            obj.push(try!(ObjectVersionParser::parse_xml("Version", stack)));
        }
        Ok(obj)
    }
}

impl ObjectVersionListWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectVersionList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            ObjectVersionWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

impl ObjectStorageClassParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectStorageClass, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ObjectStorageClassWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectStorageClass) {
        params.put(name, obj);
    }
}

impl CopyObjectRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopyObjectRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CopyObjectRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-if-modified-since" {
                obj.copy_source_if_modified_since = Some(try!(CopySourceIfModifiedSinceParser::parse_xml("x-amz-copy-source-if-modified-since", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-if-unmodified-since" {
                obj.copy_source_if_unmodified_since = Some(try!(CopySourceIfUnmodifiedSinceParser::parse_xml("x-amz-copy-source-if-unmodified-since", stack)));
                continue;
            }
            if current_name == "Content-Encoding" {
                obj.content_encoding = Some(try!(ContentEncodingParser::parse_xml("Content-Encoding", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-server-side-encryption-customer-key" {
                obj.copy_source_sse_customer_key = Some(try!(CopySourceSSECustomerKeyParser::parse_xml("x-amz-copy-source-server-side-encryption-customer-key", stack)));
                continue;
            }
            if current_name == "x-amz-storage-class" {
                obj.storage_class = Some(try!(StorageClassParser::parse_xml("x-amz-storage-class", stack)));
                continue;
            }
            if current_name == "x-amz-grant-read-acp" {
                obj.grant_read_acp = Some(try!(GrantReadACPParser::parse_xml("x-amz-grant-read-acp", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = Some(try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = Some(try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack)));
                continue;
            }
            if current_name == "Content-Disposition" {
                obj.content_disposition = Some(try!(ContentDispositionParser::parse_xml("Content-Disposition", stack)));
                continue;
            }
            if current_name == "x-amz-meta-" {
                obj.metadata = Some(try!(MetadataParser::parse_xml("x-amz-meta-", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key" {
                obj.sse_customer_key = Some(try!(SSECustomerKeyParser::parse_xml("x-amz-server-side-encryption-customer-key", stack)));
                continue;
            }
            if current_name == "x-amz-website-redirect-location" {
                obj.website_redirect_location = Some(try!(WebsiteRedirectLocationParser::parse_xml("x-amz-website-redirect-location", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source" {
                obj.copy_source = try!(CopySourceParser::parse_xml("x-amz-copy-source", stack));
                continue;
            }
            if current_name == "Expires" {
                obj.expires = Some(try!(ExpiresParser::parse_xml("Expires", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "Cache-Control" {
                obj.cache_control = Some(try!(CacheControlParser::parse_xml("Cache-Control", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-server-side-encryption-customer-algorithm" {
                obj.copy_source_sse_customer_algorithm = Some(try!(CopySourceSSECustomerAlgorithmParser::parse_xml("x-amz-copy-source-server-side-encryption-customer-algorithm", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-grant-read" {
                obj.grant_read = Some(try!(GrantReadParser::parse_xml("x-amz-grant-read", stack)));
                continue;
            }
            if current_name == "x-amz-grant-write-acp" {
                obj.grant_write_acp = Some(try!(GrantWriteACPParser::parse_xml("x-amz-grant-write-acp", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-server-side-encryption-customer-key-MD5" {
                obj.copy_source_sse_customer_key_md5 = Some(try!(CopySourceSSECustomerKeyMD5Parser::parse_xml("x-amz-copy-source-server-side-encryption-customer-key-MD5", stack)));
                continue;
            }
            if current_name == "x-amz-grant-full-control" {
                obj.grant_full_control = Some(try!(GrantFullControlParser::parse_xml("x-amz-grant-full-control", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-if-match" {
                obj.copy_source_if_match = Some(try!(CopySourceIfMatchParser::parse_xml("x-amz-copy-source-if-match", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = Some(try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack)));
                continue;
            }
            if current_name == "Content-Type" {
                obj.content_type = Some(try!(ContentTypeParser::parse_xml("Content-Type", stack)));
                continue;
            }
            if current_name == "Content-Language" {
                obj.content_language = Some(try!(ContentLanguageParser::parse_xml("Content-Language", stack)));
                continue;
            }
            if current_name == "x-amz-metadata-directive" {
                obj.metadata_directive = Some(try!(MetadataDirectiveParser::parse_xml("x-amz-metadata-directive", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-if-none-match" {
                obj.copy_source_if_none_match = Some(try!(CopySourceIfNoneMatchParser::parse_xml("x-amz-copy-source-if-none-match", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = Some(try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl CopyObjectRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CopyObjectRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        if let Some(ref obj) = obj.copy_source_if_modified_since {
            CopySourceIfModifiedSinceWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-modified-since"), obj);
        }
        if let Some(ref obj) = obj.copy_source_if_unmodified_since {
            CopySourceIfUnmodifiedSinceWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-unmodified-since"), obj);
        }
        if let Some(ref obj) = obj.content_encoding {
            ContentEncodingWriter::write_params(params, &(prefix.to_string() + "Content-Encoding"), obj);
        }
        if let Some(ref obj) = obj.copy_source_sse_customer_key {
            CopySourceSSECustomerKeyWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-server-side-encryption-customer-key"), obj);
        }
        if let Some(ref obj) = obj.storage_class {
            StorageClassWriter::write_params(params, &(prefix.to_string() + "x-amz-storage-class"), obj);
        }
        if let Some(ref obj) = obj.grant_read_acp {
            GrantReadACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read-acp"), obj);
        }
        if let Some(ref obj) = obj.server_side_encryption {
            ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), obj);
        }
        if let Some(ref obj) = obj.ssekms_key_id {
            SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), obj);
        }
        if let Some(ref obj) = obj.content_disposition {
            ContentDispositionWriter::write_params(params, &(prefix.to_string() + "Content-Disposition"), obj);
        }
        if let Some(ref obj) = obj.metadata {
            MetadataWriter::write_params(params, &(prefix.to_string() + "x-amz-meta-"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key {
            SSECustomerKeyWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key"), obj);
        }
        if let Some(ref obj) = obj.website_redirect_location {
            WebsiteRedirectLocationWriter::write_params(params, &(prefix.to_string() + "x-amz-website-redirect-location"), obj);
        }
        CopySourceWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source"), &obj.copy_source);
        if let Some(ref obj) = obj.expires {
            ExpiresWriter::write_params(params, &(prefix.to_string() + "Expires"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        if let Some(ref obj) = obj.cache_control {
            CacheControlWriter::write_params(params, &(prefix.to_string() + "Cache-Control"), obj);
        }
        if let Some(ref obj) = obj.copy_source_sse_customer_algorithm {
            CopySourceSSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-server-side-encryption-customer-algorithm"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.grant_read {
            GrantReadWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read"), obj);
        }
        if let Some(ref obj) = obj.grant_write_acp {
            GrantWriteACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-write-acp"), obj);
        }
        if let Some(ref obj) = obj.copy_source_sse_customer_key_md5 {
            CopySourceSSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-copy-source-server-side-encryption-customer-key-MD5"), obj);
        }
        if let Some(ref obj) = obj.grant_full_control {
            GrantFullControlWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-full-control"), obj);
        }
        if let Some(ref obj) = obj.copy_source_if_match {
            CopySourceIfMatchWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-match"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_algorithm {
            SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), obj);
        }
        if let Some(ref obj) = obj.content_type {
            ContentTypeWriter::write_params(params, &(prefix.to_string() + "Content-Type"), obj);
        }
        if let Some(ref obj) = obj.content_language {
            ContentLanguageWriter::write_params(params, &(prefix.to_string() + "Content-Language"), obj);
        }
        if let Some(ref obj) = obj.metadata_directive {
            MetadataDirectiveWriter::write_params(params, &(prefix.to_string() + "x-amz-metadata-directive"), obj);
        }
        if let Some(ref obj) = obj.copy_source_if_none_match {
            CopySourceIfNoneMatchWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-none-match"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key_md5 {
            SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), obj);
        }
    }
}

impl DeleteObjectsOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteObjectsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteObjectsOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "DeletedObject" {
                obj.deleted = try!(DeletedObjectsParser::parse_xml("DeletedObject", stack));
                continue;
            }
            if current_name == "Error" {
                obj.errors = try!(ErrorsParser::parse_xml("Error", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl DeleteObjectsOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteObjectsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DeletedObjectsWriter::write_params(params, &(prefix.to_string() + "DeletedObject"), &obj.deleted);
        ErrorsWriter::write_params(params, &(prefix.to_string() + "Error"), &obj.errors);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
    }
}

impl GetObjectOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Last-Modified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("Last-Modified", stack));
                continue;
            }
            if current_name == "Content-Range" {
                obj.content_range = try!(ContentRangeParser::parse_xml("Content-Range", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "Content-Encoding" {
                obj.content_encoding = try!(ContentEncodingParser::parse_xml("Content-Encoding", stack));
                continue;
            }
            if current_name == "x-amz-replication-status" {
                obj.replication_status = try!(ReplicationStatusParser::parse_xml("x-amz-replication-status", stack));
                continue;
            }
            if current_name == "x-amz-storage-class" {
                obj.storage_class = try!(StorageClassParser::parse_xml("x-amz-storage-class", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            if current_name == "Content-Disposition" {
                obj.content_disposition = try!(ContentDispositionParser::parse_xml("Content-Disposition", stack));
                continue;
            }
            if current_name == "x-amz-meta-" {
                obj.metadata = try!(MetadataParser::parse_xml("x-amz-meta-", stack));
                continue;
            }
            if current_name == "Body" {
                obj.body = try!(BodyParser::parse_xml("Body", stack));
                continue;
            }
            if current_name == "accept-ranges" {
                obj.accept_ranges = try!(AcceptRangesParser::parse_xml("accept-ranges", stack));
                continue;
            }
            if current_name == "x-amz-website-redirect-location" {
                obj.website_redirect_location = try!(WebsiteRedirectLocationParser::parse_xml("x-amz-website-redirect-location", stack));
                continue;
            }
            if current_name == "Expires" {
                obj.expires = try!(ExpiresParser::parse_xml("Expires", stack));
                continue;
            }
            if current_name == "x-amz-delete-marker" {
                obj.delete_marker = try!(DeleteMarkerParser::parse_xml("x-amz-delete-marker", stack));
                continue;
            }
            if current_name == "Cache-Control" {
                obj.cache_control = try!(CacheControlParser::parse_xml("Cache-Control", stack));
                continue;
            }
            if current_name == "Content-Length" {
                obj.content_length = try!(ContentLengthParser::parse_xml("Content-Length", stack));
                continue;
            }
            if current_name == "x-amz-expiration" {
                obj.expiration = try!(ExpirationParser::parse_xml("x-amz-expiration", stack));
                continue;
            }
            if current_name == "x-amz-missing-meta" {
                obj.missing_meta = try!(MissingMetaParser::parse_xml("x-amz-missing-meta", stack));
                continue;
            }
            if current_name == "x-amz-restore" {
                obj.restore = try!(RestoreParser::parse_xml("x-amz-restore", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "Content-Type" {
                obj.content_type = try!(ContentTypeParser::parse_xml("Content-Type", stack));
                continue;
            }
            if current_name == "Content-Language" {
                obj.content_language = try!(ContentLanguageParser::parse_xml("Content-Language", stack));
                continue;
            }
            if current_name == "x-amz-version-id" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("x-amz-version-id", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl GetObjectOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "Last-Modified"), &obj.last_modified);
        ContentRangeWriter::write_params(params, &(prefix.to_string() + "Content-Range"), &obj.content_range);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        ContentEncodingWriter::write_params(params, &(prefix.to_string() + "Content-Encoding"), &obj.content_encoding);
        ReplicationStatusWriter::write_params(params, &(prefix.to_string() + "x-amz-replication-status"), &obj.replication_status);
        StorageClassWriter::write_params(params, &(prefix.to_string() + "x-amz-storage-class"), &obj.storage_class);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
        ContentDispositionWriter::write_params(params, &(prefix.to_string() + "Content-Disposition"), &obj.content_disposition);
        MetadataWriter::write_params(params, &(prefix.to_string() + "x-amz-meta-"), &obj.metadata);
        BodyWriter::write_params(params, &(prefix.to_string() + "Body"), &obj.body);
        AcceptRangesWriter::write_params(params, &(prefix.to_string() + "accept-ranges"), &obj.accept_ranges);
        WebsiteRedirectLocationWriter::write_params(params, &(prefix.to_string() + "x-amz-website-redirect-location"), &obj.website_redirect_location);
        ExpiresWriter::write_params(params, &(prefix.to_string() + "Expires"), &obj.expires);
        DeleteMarkerWriter::write_params(params, &(prefix.to_string() + "x-amz-delete-marker"), &obj.delete_marker);
        CacheControlWriter::write_params(params, &(prefix.to_string() + "Cache-Control"), &obj.cache_control);
        ContentLengthWriter::write_params(params, &(prefix.to_string() + "Content-Length"), &obj.content_length);
        ExpirationWriter::write_params(params, &(prefix.to_string() + "x-amz-expiration"), &obj.expiration);
        MissingMetaWriter::write_params(params, &(prefix.to_string() + "x-amz-missing-meta"), &obj.missing_meta);
        RestoreWriter::write_params(params, &(prefix.to_string() + "x-amz-restore"), &obj.restore);
        SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), &obj.sse_customer_algorithm);
        ContentTypeWriter::write_params(params, &(prefix.to_string() + "Content-Type"), &obj.content_type);
        ContentLanguageWriter::write_params(params, &(prefix.to_string() + "Content-Language"), &obj.content_language);
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "x-amz-version-id"), &obj.version_id);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), &obj.sse_customer_key_md5);
    }
}

impl MultipartUploadCreateRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MultipartUploadCreateRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = MultipartUploadCreateRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Content-Encoding" {
                obj.content_encoding = Some(try!(ContentEncodingParser::parse_xml("Content-Encoding", stack)));
                continue;
            }
            if current_name == "x-amz-storage-class" {
                obj.storage_class = Some(try!(StorageClassParser::parse_xml("x-amz-storage-class", stack)));
                continue;
            }
            if current_name == "x-amz-grant-read-acp" {
                obj.grant_read_acp = Some(try!(GrantReadACPParser::parse_xml("x-amz-grant-read-acp", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = Some(try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = Some(try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack)));
                continue;
            }
            if current_name == "Content-Disposition" {
                obj.content_disposition = Some(try!(ContentDispositionParser::parse_xml("Content-Disposition", stack)));
                continue;
            }
            if current_name == "x-amz-meta-" {
                obj.metadata = Some(try!(MetadataParser::parse_xml("x-amz-meta-", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key" {
                obj.sse_customer_key = Some(try!(SSECustomerKeyParser::parse_xml("x-amz-server-side-encryption-customer-key", stack)));
                continue;
            }
            if current_name == "x-amz-website-redirect-location" {
                obj.website_redirect_location = Some(try!(WebsiteRedirectLocationParser::parse_xml("x-amz-website-redirect-location", stack)));
                continue;
            }
            if current_name == "Expires" {
                obj.expires = Some(try!(ExpiresParser::parse_xml("Expires", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "Cache-Control" {
                obj.cache_control = Some(try!(CacheControlParser::parse_xml("Cache-Control", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-grant-read" {
                obj.grant_read = Some(try!(GrantReadParser::parse_xml("x-amz-grant-read", stack)));
                continue;
            }
            if current_name == "x-amz-grant-write-acp" {
                obj.grant_write_acp = Some(try!(GrantWriteACPParser::parse_xml("x-amz-grant-write-acp", stack)));
                continue;
            }
            if current_name == "x-amz-acl" {
                obj.acl = Some(try!(ObjectCannedACLParser::parse_xml("x-amz-acl", stack)));
                continue;
            }
            if current_name == "x-amz-grant-full-control" {
                obj.grant_full_control = Some(try!(GrantFullControlParser::parse_xml("x-amz-grant-full-control", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = Some(try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack)));
                continue;
            }
            if current_name == "Content-Type" {
                obj.content_type = Some(try!(ContentTypeParser::parse_xml("Content-Type", stack)));
                continue;
            }
            if current_name == "Content-Language" {
                obj.content_language = Some(try!(ContentLanguageParser::parse_xml("Content-Language", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = Some(try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl MultipartUploadCreateRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MultipartUploadCreateRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        if let Some(ref obj) = obj.content_encoding {
            ContentEncodingWriter::write_params(params, &(prefix.to_string() + "Content-Encoding"), obj);
        }
        if let Some(ref obj) = obj.storage_class {
            StorageClassWriter::write_params(params, &(prefix.to_string() + "x-amz-storage-class"), obj);
        }
        if let Some(ref obj) = obj.grant_read_acp {
            GrantReadACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read-acp"), obj);
        }
        if let Some(ref obj) = obj.server_side_encryption {
            ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), obj);
        }
        if let Some(ref obj) = obj.ssekms_key_id {
            SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), obj);
        }
        if let Some(ref obj) = obj.content_disposition {
            ContentDispositionWriter::write_params(params, &(prefix.to_string() + "Content-Disposition"), obj);
        }
        if let Some(ref obj) = obj.metadata {
            MetadataWriter::write_params(params, &(prefix.to_string() + "x-amz-meta-"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key {
            SSECustomerKeyWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key"), obj);
        }
        if let Some(ref obj) = obj.website_redirect_location {
            WebsiteRedirectLocationWriter::write_params(params, &(prefix.to_string() + "x-amz-website-redirect-location"), obj);
        }
        if let Some(ref obj) = obj.expires {
            ExpiresWriter::write_params(params, &(prefix.to_string() + "Expires"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        if let Some(ref obj) = obj.cache_control {
            CacheControlWriter::write_params(params, &(prefix.to_string() + "Cache-Control"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.grant_read {
            GrantReadWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read"), obj);
        }
        if let Some(ref obj) = obj.grant_write_acp {
            GrantWriteACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-write-acp"), obj);
        }
        if let Some(ref obj) = obj.acl {
            ObjectCannedACLWriter::write_params(params, &(prefix.to_string() + "x-amz-acl"), obj);
        }
        if let Some(ref obj) = obj.grant_full_control {
            GrantFullControlWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-full-control"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_algorithm {
            SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), obj);
        }
        if let Some(ref obj) = obj.content_type {
            ContentTypeWriter::write_params(params, &(prefix.to_string() + "Content-Type"), obj);
        }
        if let Some(ref obj) = obj.content_language {
            ContentLanguageWriter::write_params(params, &(prefix.to_string() + "Content-Language"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key_md5 {
            SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), obj);
        }
    }
}

impl GetObjectTorrentRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetObjectTorrentRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetObjectTorrentRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
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

impl GetObjectTorrentRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetObjectTorrentRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}

impl DeleteMarkerEntryParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMarkerEntry, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteMarkerEntry::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            if current_name == "IsLatest" {
                obj.is_latest = try!(IsLatestParser::parse_xml("IsLatest", stack));
                continue;
            }
            if current_name == "VersionId" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("VersionId", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "LastModified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl DeleteMarkerEntryWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteMarkerEntry) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        IsLatestWriter::write_params(params, &(prefix.to_string() + "IsLatest"), &obj.is_latest);
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
    }
}

impl DeleteMarkersParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMarkers, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "DeleteMarker" {
            obj.push(try!(DeleteMarkerEntryParser::parse_xml("DeleteMarker", stack)));
        }
        Ok(obj)
    }
}

impl DeleteMarkersWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteMarkers) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            DeleteMarkerEntryWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

impl ListObjectsRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListObjectsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListObjectsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "prefix" {
                obj.prefix = Some(try!(PrefixParser::parse_xml("prefix", stack)));
                continue;
            }
            if current_name == "max-keys" {
                obj.max_keys = Some(try!(MaxKeysParser::parse_xml("max-keys", stack)));
                continue;
            }
            if current_name == "delimiter" {
                obj.delimiter = Some(try!(DelimiterParser::parse_xml("delimiter", stack)));
                continue;
            }
            if current_name == "marker" {
                obj.marker = Some(try!(MarkerParser::parse_xml("marker", stack)));
                continue;
            }
            if current_name == "encoding-type" {
                obj.encoding_type = Some(try!(EncodingTypeParser::parse_xml("encoding-type", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ListObjectsRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ListObjectsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.prefix {
            PrefixWriter::write_params(params, &(prefix.to_string() + "prefix"), obj);
        }
        if let Some(ref obj) = obj.max_keys {
            MaxKeysWriter::write_params(params, &(prefix.to_string() + "max-keys"), obj);
        }
        if let Some(ref obj) = obj.delimiter {
            DelimiterWriter::write_params(params, &(prefix.to_string() + "delimiter"), obj);
        }
        if let Some(ref obj) = obj.marker {
            MarkerWriter::write_params(params, &(prefix.to_string() + "marker"), obj);
        }
        if let Some(ref obj) = obj.encoding_type {
            EncodingTypeWriter::write_params(params, &(prefix.to_string() + "encoding-type"), obj);
        }
    }
}

impl NextVersionIdMarkerParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NextVersionIdMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl NextVersionIdMarkerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &NextVersionIdMarker) {
        params.put(name, obj);
    }
}

//ListVersionsOutput
impl ListVersionsResultParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListVersionsResult, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListVersionsResult::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Name" {
                obj.name = try!(BucketNameParser::parse_xml("Name", stack));
                continue;
            }
            if current_name == "Version" {
                obj.versions = try!(ObjectVersionListParser::parse_xml("Version", stack));
                continue;
            }
            if current_name == "DeleteMarker" {
                obj.delete_markers = try!(DeleteMarkersParser::parse_xml("DeleteMarker", stack));
                continue;
            }
            if current_name == "NextKeyMarker" {
                obj.next_key_marker = try!(NextKeyMarkerParser::parse_xml("NextKeyMarker", stack));
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
            if current_name == "KeyMarker" {
                obj.key_marker = try!(KeyMarkerParser::parse_xml("KeyMarker", stack));
                continue;
            }
            if current_name == "NextVersionIdMarker" {
                obj.next_version_id_marker = try!(NextVersionIdMarkerParser::parse_xml("NextVersionIdMarker", stack));
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
            if current_name == "VersionIdMarker" {
                obj.version_id_marker = try!(VersionIdMarkerParser::parse_xml("VersionIdMarker", stack));
                continue;
            }
            if current_name == "CommonPrefix" {
                obj.common_prefixes = try!(CommonPrefixListParser::parse_xml("CommonPrefix", stack));
                continue;
            }
            break;
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ListVersionsResultWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ListVersionsResult) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
        ObjectVersionListWriter::write_params(params, &(prefix.to_string() + "Version"), &obj.versions);
        DeleteMarkersWriter::write_params(params, &(prefix.to_string() + "DeleteMarkerEntry"), &obj.delete_markers);
        NextKeyMarkerWriter::write_params(params, &(prefix.to_string() + "NextKeyMarker"), &obj.next_key_marker);
        DelimiterWriter::write_params(params, &(prefix.to_string() + "Delimiter"), &obj.delimiter);
        MaxKeysWriter::write_params(params, &(prefix.to_string() + "MaxKeys"), &obj.max_keys);
        PrefixWriter::write_params(params, &(prefix.to_string() + "Prefix"), &obj.prefix);
        KeyMarkerWriter::write_params(params, &(prefix.to_string() + "KeyMarker"), &obj.key_marker);
        NextVersionIdMarkerWriter::write_params(params, &(prefix.to_string() + "NextVersionIdMarker"), &obj.next_version_id_marker);
        EncodingTypeWriter::write_params(params, &(prefix.to_string() + "EncodingType"), &obj.encoding_type);
        IsTruncatedWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
        VersionIdMarkerWriter::write_params(params, &(prefix.to_string() + "VersionIdMarker"), &obj.version_id_marker);
        CommonPrefixListWriter::write_params(params, &(prefix.to_string() + "CommonPrefix"), &obj.common_prefixes);
    }
}

impl ListObjectVersionsRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListObjectVersionsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListObjectVersionsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "prefix" {
                obj.prefix = Some(try!(PrefixParser::parse_xml("prefix", stack)));
                continue;
            }
            if current_name == "max-keys" {
                obj.max_keys = Some(try!(MaxKeysParser::parse_xml("max-keys", stack)));
                continue;
            }
            if current_name == "delimiter" {
                obj.delimiter = Some(try!(DelimiterParser::parse_xml("delimiter", stack)));
                continue;
            }
            if current_name == "key-marker" {
                obj.key_marker = Some(try!(KeyMarkerParser::parse_xml("key-marker", stack)));
                continue;
            }
            if current_name == "encoding-type" {
                obj.encoding_type = Some(try!(EncodingTypeParser::parse_xml("encoding-type", stack)));
                continue;
            }
            if current_name == "version-id-marker" {
                obj.version_id_marker = Some(try!(VersionIdMarkerParser::parse_xml("version-id-marker", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl ListObjectVersionsRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ListObjectVersionsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.prefix {
            PrefixWriter::write_params(params, &(prefix.to_string() + "prefix"), obj);
        }
        if let Some(ref obj) = obj.max_keys {
            MaxKeysWriter::write_params(params, &(prefix.to_string() + "max-keys"), obj);
        }
        if let Some(ref obj) = obj.delimiter {
            DelimiterWriter::write_params(params, &(prefix.to_string() + "delimiter"), obj);
        }
        if let Some(ref obj) = obj.key_marker {
            KeyMarkerWriter::write_params(params, &(prefix.to_string() + "key-marker"), obj);
        }
        if let Some(ref obj) = obj.encoding_type {
            EncodingTypeWriter::write_params(params, &(prefix.to_string() + "encoding-type"), obj);
        }
        if let Some(ref obj) = obj.version_id_marker {
            VersionIdMarkerWriter::write_params(params, &(prefix.to_string() + "version-id-marker"), obj);
        }
    }
}

impl MultipartUploadCreateOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MultipartUploadCreateOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = MultipartUploadCreateOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "UploadId" {
                obj.upload_id = try!(MultipartUploadIdParser::parse_xml("UploadId", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl MultipartUploadCreateOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MultipartUploadCreateOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), &obj.sse_customer_algorithm);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        MultipartUploadIdWriter::write_params(params, &(prefix.to_string() + "UploadId"), &obj.upload_id);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), &obj.sse_customer_key_md5);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
    }
}

impl HeadObjectRequestParser {
    fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<HeadObjectRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = HeadObjectRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = Some(try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key" {
                obj.sse_customer_key = Some(try!(SSECustomerKeyParser::parse_xml("x-amz-server-side-encryption-customer-key", stack)));
                continue;
            }
            if current_name == "If-Unmodified-Since" {
                obj.if_unmodified_since = Some(try!(IfUnmodifiedSinceParser::parse_xml("If-Unmodified-Since", stack)));
                continue;
            }
            if current_name == "versionId" {
                obj.version_id = Some(try!(ObjectVersionIdParser::parse_xml("versionId", stack)));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "If-None-Match" {
                obj.if_none_match = Some(try!(IfNoneMatchParser::parse_xml("If-None-Match", stack)));
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
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = Some(try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack)));
                continue;
            }
            if current_name == "If-Modified-Since" {
                obj.if_modified_since = Some(try!(IfModifiedSinceParser::parse_xml("If-Modified-Since", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

impl HeadObjectRequestWriter {
    fn write_params(params: &mut Params, name: &str, obj: &HeadObjectRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.sse_customer_algorithm {
            SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key {
            SSECustomerKeyWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key"), obj);
        }
        if let Some(ref obj) = obj.if_unmodified_since {
            IfUnmodifiedSinceWriter::write_params(params, &(prefix.to_string() + "If-Unmodified-Since"), obj);
        }
        if let Some(ref obj) = obj.version_id {
            ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "versionId"), obj);
        }
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.if_none_match {
            IfNoneMatchWriter::write_params(params, &(prefix.to_string() + "If-None-Match"), obj);
        }
        if let Some(ref obj) = obj.range {
            RangeWriter::write_params(params, &(prefix.to_string() + "Range"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        if let Some(ref obj) = obj.if_match {
            IfMatchWriter::write_params(params, &(prefix.to_string() + "If-Match"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key_md5 {
            SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), obj);
        }
        if let Some(ref obj) = obj.if_modified_since {
            IfModifiedSinceWriter::write_params(params, &(prefix.to_string() + "If-Modified-Since"), obj);
        }
    }
}
