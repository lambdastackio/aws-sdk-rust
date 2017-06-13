// Copyright 2017 LambdaStack All rights reserved.
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
//! AWS API requests.
//!
//! Wraps the Hyper library to send PUT, POST, DELETE and GET requests.

use std::io::Read;
use std::io::Error as IoError;
use std::error::Error;
use std::fmt;
use std::collections::HashMap;

use hyper::Client;
use hyper::Error as HyperError;
use hyper::header::Headers;
use hyper::method::Method;

use aws::common::signature::SignedRequest;

/// Wraps the Hyper Response that comes back from AWS S3.
///
/// All HTTP calls are sent from here.
///
#[derive(Debug, Default, RustcDecodable, RustcEncodable)]
pub struct HttpResponse {
    /// HTTP status code
    pub status: u16,
    /// XML payload
    pub body: String,
    pub body_buffer: Vec<u8>, //&'a[u8],
    // used by the get_body function to return the correct slice...
    pub is_body: bool,
    /// Unsorted list of header attributes
    pub headers: HashMap<String, String>,
}

/// HTTP Error returned from the DispatchSignedRequest Trait. It also implements the Error Trait.
#[derive(Debug, Default, PartialEq, RustcDecodable, RustcEncodable)]
pub struct HttpDispatchError {
    message: String,
}

impl Error for HttpDispatchError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for HttpDispatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<HyperError> for HttpDispatchError {
    fn from(err: HyperError) -> HttpDispatchError {
        HttpDispatchError { message: err.description().to_string() }
    }
}

impl From<IoError> for HttpDispatchError {
    fn from(err: IoError) -> HttpDispatchError {
        HttpDispatchError { message: err.description().to_string() }
    }
}

/// Trait that is added to the Hyper Client type. This is where ALL HTTP calls get sent from.
pub trait DispatchSignedRequest {
    fn dispatch(&self, request: &SignedRequest) -> Result<HttpResponse, HttpDispatchError>;
}

impl DispatchSignedRequest for Client {
    fn dispatch(&self,
                request: &SignedRequest
            ) -> Result<HttpResponse, HttpDispatchError> {
        let hyper_method = match request.method().as_ref() {
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "GET" => Method::Get,
            "HEAD" => Method::Head,
            v @ _ => return Err(HttpDispatchError { message: format!("Unsupported HTTP verb {}", v) }),
        };

        // translate the headers map to a format Hyper likes
        let mut hyper_headers = Headers::new();
        for h in request.headers().iter() {
            hyper_headers.set_raw(h.0.to_owned(), h.1.to_owned());
        }

        let epp = request.endpoint().clone().endpoint.unwrap().port();
        let port_str = match epp {
            Some(port) => format!(":{}", port),
            _ => "".to_string(),
        };

        let mut final_uri = format!("{}://{}{}{}",
                                    request.endpoint_scheme(),
                                    request.hostname(),
                                    port_str,
                                    request.path());
        if !request.canonical_query_string().is_empty() {
            let uri = final_uri.clone();
            final_uri = final_uri + &format!("{}{}", if uri.contains("?") {""} else {"?"}, request.canonical_query_string());
            final_uri = final_uri.replace("?", &request.path_options().unwrap_or("?".to_string()));
        } else {
            final_uri = final_uri + &format!("{}", request.path_options().unwrap_or("".to_string()));
        }

        // SENDS
        let mut hyper_response = match request.payload() {
            None => try!(self.request(hyper_method, &final_uri).headers(hyper_headers).body("").send()),
            Some(payload_contents) => try!(self.request(hyper_method, &final_uri)
                                                            .headers(hyper_headers)
                                                            .body(payload_contents)
                                                            .send()),
        };

        let mut headers: HashMap<String, String> = HashMap::new();
        for header in hyper_response.headers.iter() {
            headers.insert(header.name().to_string(), header.value_string());
        }

        // The initial way was to string but UTF8 errors occured on certain object types so switched
        let mut is_body = true;

        // NB: Should be a better way to do this for compressed or binary files but works.
        let mut buffer: Vec<u8> = Vec::new();
        let body:String;
        let size: usize;

        match hyper_response.read_to_end(&mut buffer) {
            Ok(len) => {size = len},
            _ => {size = 0 as usize},
        }

        if size > 0 {
            match String::from_utf8(buffer.clone()) {
                Ok(buf) => {
                    body = buf;
                    buffer = Vec::new();
                },
                _ => {
                    body = String::new();
                    is_body = false;
                },
            }
        } else {
            body = String::new();
        }

        // The HttpResponse also contains a body_buffer for large binaries mainly. Body is used
        // most often. Typically, on get_object cares.
        // Some bodies are XML and some are binary. Body is for String and body_buffer is for binary.
        Ok(HttpResponse {
            status: hyper_response.status.to_u16(),
            body: body,
            body_buffer: buffer,
            is_body: is_body,
            headers: headers,
        })
    }
}
