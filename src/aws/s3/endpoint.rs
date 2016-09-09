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

//! Client Documentation
//!
//! The crate Url is required and used to build and extract portions of the Url as needed by
//! the library. See the README.md and/or src/main.rs for an example of how to use the library.

use url::Url;
use aws::common::region::Region;

/// Endpoint allows you to set a custom endpoint and/or a proxy for a given region and associate this
/// as an endpoint of where S3Client will look for the data.
///
/// NB: Endpoint is *not* JSON encodable/decodable without implementing a custom to_json trait
/// because of third party Url struct.
///
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Endpoint {
    /// Specify the specific Region you're targeting the request towards. The default is:
    /// Region::UsEast1. This value should be Region::UsEast1 for third party services since some
    /// do not use Region information for their Endpoint.
    pub region: Region,
    /// Signature is an enum of V2 or V4. Specify Signature::V2 or Signature::V4. See notes below.
    pub signature: Signature,
    /// Uses Url crate. AWS has a fixed set of Endpoints. However, third party services also
    /// need to be covered so adding an option for the Endpoint solves that requirement.
    /// If using AWS then it will default to the given Endpoint for the specified Region.
    pub endpoint: Option<Url>,
    /// Important: Proxies are used by most Enterprises. You can specify your proxy with the
    /// port in the following format https://<whatever url>:<whatever port>. Also, it honors
    /// the http_proxy, https_proxy and no_proxy environment variables if present. However,
    /// manually setting the value takes precedent.
    pub proxy: Option<Url>,
    /// `User-Agent`. It lives in `Endpoint` since you may want a different `User-Agent` for
    /// `Endpoint`. This value is an Option<String> which can be None.
    pub user_agent: Option<String>,
}

/// Required to specify which type of API Signature to use. AWS defaults to using V4 by default.
/// However, third party applications often use V2 (AWS will still honor V2).
//#[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub enum Signature {
    V2,
    V4,
}

impl Endpoint {
    /// Endpoint::new accepts Region, Signature, an optional Url and an optional proxy Url:port.
    pub fn new(region: Region,
               signature: Signature,
               endpoint: Option<Url>,
               proxy: Option<Url>,
               user_agent: Option<String>) -> Self {
        Endpoint {
            region: region,
            signature: signature,
            endpoint: default_endpoint(region, endpoint),
            proxy: proxy,
            user_agent: user_agent,
        }
    }

    /// Extracts out the host portion of the URL as defined by the crate Url.
    pub fn hostname(&self) -> Option<String> {
        match self.endpoint {
            None => None,
            Some(ref url) => Some(url.host_str().unwrap().to_string()),
        }
    }
}

// This creates the default endpoint to be used on initial create if endpoint is None
fn default_endpoint(region: Region, endpoint: Option<Url>) -> Option<Url> {
    let final_endpoint: Url;
    match endpoint {
        Some(url) => final_endpoint = url,
        None => {
            // NOTE: Must include the correct scheme (http or https)
            let endpoint: String = match region {
                Region::UsEast1 => "https://s3.amazonaws.com".to_string(),
                Region::CnNorth1 => format!("https://s3.{}.amazonaws.com.cn", region),
                _ => format!("https://s3.amazonaws.com"),
            };
            final_endpoint = Url::parse(&endpoint).unwrap();
        },
    };

    Some(final_endpoint)
}
