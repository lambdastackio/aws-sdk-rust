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

use url::Url;
use aws::common::region::Region;

/// Endpoint allows you to set a custom endpoint and/or a proxy for a given region and associate this
/// as an endpoint of where S3Client will look for the data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Endpoint {
    pub region: Region,
    pub signature: String,
    pub endpoint: Option<Url>,
    pub proxy: Option<Url>,
}

impl Endpoint {
    /// Endpoint::new accepts Region, an optional Url and an optional proxy url:port.
    pub fn new<S>(region: Region, signature: S, endpoint: Option<Url>, proxy: Option<Url>) -> Self
        where S: Into<String>,
    {
        Endpoint {
            region: region,
            signature: signature.into(),
            endpoint: default_endpoint(region, endpoint),
            proxy: proxy,
        }
    }

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
