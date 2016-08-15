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

use xml::*;

use aws::common::params::{Params, ServiceParams};
use aws::common::region::Region;
use aws::common::xmlutil::*;
use aws::s3::writeparse::*;

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
        }
        _ => {
            let xml = format!("<CreateBucketConfiguration xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">
        <LocationConstraint>{}</LocationConstraint>
        </CreateBucketConfiguration >", region);
            xml.into_bytes()
        }
    }
}

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
pub struct Bucket {
    /// Date the bucket was created.
    pub creation_date: CreationDate,
    /// The name of the bucket.
    pub name: BucketName,
}
