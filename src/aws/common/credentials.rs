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

//! Client Documentation
//!

use std::env::*;
use std::env;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::io::BufReader;
use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::sync::Mutex;
use std::cell::RefCell;
use std::time::Duration as StdDuration;

use hyper::Client;
use hyper::header::Connection;
use regex::Regex;
use chrono::{Duration, UTC, DateTime};
use serde_json::{Value, from_str};

use aws::errors::creds::CredentialsError;

/// Primarily intended for client applications but also used for internal library documentation.
///
/// AwsCredentials - Base struct for AWS
///
/// This struct is very important! Without valid credentials then access to AWS S3 will be
/// allowed.
#[derive(Clone, Debug)]
pub struct AwsCredentials {
    /// access_key_id - Can be &str or String. Represents AWS Access Key.
    access_key_id: String,
    /// secret_key - Can be &str or String. Represents AWS Secret Key.
    secret_access_key: String,
    /// token - None or String. Represents AWS Token for IAM credentials.
    token: Option<String>,
    /// expires_at - Default to 10 minutes.
    expires_at: DateTime<UTC>
}

impl AwsCredentials {
    /// First method to be called. Creates the AWS credentials.
    pub fn new<K, S>(access_key_id:K,
        secret_access_key:S,
        token:Option<String>,
        expires_at:DateTime<UTC>)
        -> AwsCredentials where K:Into<String>, S:Into<String> {
        AwsCredentials {
            access_key_id: access_key_id.into(),
            secret_access_key: secret_access_key.into(),
            token: token,
            expires_at: expires_at,
        }
    }

    /// Get a reference to the access key ID.
    pub fn aws_access_key_id(&self) -> &str {
        &self.access_key_id
    }

    /// Get a reference to the secret key.
    pub fn aws_secret_access_key(&self) -> &str {
        &self.secret_access_key
    }

    /// Get a reference to the expiration time.
    pub fn expires_at(&self) -> &DateTime<UTC> {
        &self.expires_at
    }

    /// Get a reference to the access token.
    pub fn token(&self) -> &Option<String> {
        &self.token
    }

    /// Determine whether or not the credentials have expired.
    fn credentials_are_expired(&self) -> bool {
        self.expires_at < UTC::now() + Duration::seconds(20)
    }
}

/// A trait for types that produce `AwsCredentials` This trait is implemented on most S3 calls.
pub trait AwsCredentialsProvider {
    /// Produce a new `AwsCredentials`.
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError>;
}

/// Provides AWS credentials from environment variables.
pub struct EnvironmentProvider;

impl AwsCredentialsProvider for EnvironmentProvider {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        let env_key = match var("AWS_ACCESS_KEY_ID") {
            Ok(val) => val,
            Err(_) => return Err(CredentialsError::new("No AWS_ACCESS_KEY_ID in environment"))
        };
        let env_secret = match var("AWS_SECRET_ACCESS_KEY") {
            Ok(val) => val,
            Err(_) => return Err(CredentialsError::new("No AWS_SECRET_ACCESS_KEY in environment"))
        };

        if env_key.is_empty() || env_secret.is_empty() {
            return Err(CredentialsError::new(
                            "Couldn't find either AWS_ACCESS_KEY_ID, \
                            AWS_SECRET_ACCESS_KEY or both in environment."));
        }

        // Present when using temporary credentials, e.g. on Lambda with IAM roles
        let token = match var("AWS_SESSION_TOKEN") {
            Ok(val) => {
                if val.is_empty() {
                    None
                } else {
                    Some(val)
                }
            }
            Err(_) => None,
        };

        Ok(AwsCredentials::new(env_key, env_secret, token, in_ten_minutes()))
    }
}

/// Provides AWS credentials via Parameters. This allows you to use your own config settings
/// and pull the credentials from there and set them here. This is also part of the chained
/// provider where all of the credential providers can be tried in a given order of priority.
#[derive(Clone, Debug)]
pub struct ParametersProvider {
    credentials: Option<AwsCredentials>
}

impl ParametersProvider {
    pub fn new() -> Result<ParametersProvider, CredentialsError> {
        Ok(ParametersProvider{
            credentials: None
        })
    }

    pub fn with_params<K, S>(
        access_key_id:K,
        secret_access_key:S,
        token:Option<String>)
        -> Result<ParametersProvider, CredentialsError> where K:Into<String>, S:Into<String> {

        let key = access_key_id.into();
        let secret = secret_access_key.into();

        if key.is_empty() || secret.is_empty() {
            return Err(CredentialsError::new("Keys are invalid."));
        }

        Ok(ParametersProvider {
            credentials: Some(AwsCredentials::new(key, secret, token, in_ten_minutes()))
        })
    }
}

impl AwsCredentialsProvider for ParametersProvider {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        let creds = match self.credentials {
            None => return Err(CredentialsError::new("No credentials.")),
            Some(_) => self.credentials.to_owned().unwrap(),
        };

        Ok(creds)
    }
}

/// Provides AWS credentials from a profile in a credentials file.
///
/// The credentials file is located in the home directory of the given user.
#[derive(Clone, Debug)]
pub struct ProfileProvider {
    credentials: Option<AwsCredentials>,
    location: PathBuf,
    profile: String,
}

impl ProfileProvider {
    /// Create a new `ProfileProvider` for the default credentials file path and profile name.
    ///
    /// More details on the AWS credentials file can be found at AWS.
    /// Linux or Mac OS - ~/.aws/credentials
    /// Windows - %USERPROFILE%\.aws\credentials
    ///
    /// Sets the "default" credentials but can be overridden with set_profile.

    pub fn new() -> Result<ProfileProvider, CredentialsError> {
        let location = match env::home_dir() {
            Some(home) => {
                let mut credentials_path = PathBuf::from(".aws");
                credentials_path.push("credentials");
                home.join(credentials_path)
            }
            None => return Err(CredentialsError::new(
                                    "The environment variable HOME must be set.")),
        };

        Ok(ProfileProvider {
            credentials: None,
            location: location,
            profile: "default".to_owned(),
        })
    }

    /// Create a new `ProfileProvider` for the credentials file at the given path, using
    /// the given profile.
    pub fn with_configuration<F, P>(location: F, profile: P) -> ProfileProvider
    where F: Into<PathBuf>, P: Into<String> {
        ProfileProvider {
            credentials: None,
            location: location.into(),
            profile: profile.into(),
        }
    }

    /// Get a reference to the credentials location.
    pub fn location(&self) -> &Path {
        self.location.as_ref()
    }

    /// Get a reference to the profile name. Profile name is the subsection in the credentials
    /// file. See AWS for details.
    pub fn profile(&self) -> &str {
        &self.profile
    }

    /// Set the credentials location.
    pub fn set_location<F>(&mut self, location: F) where F: Into<PathBuf> {
        self.location = location.into();
    }

    /// Set the profile name.
    pub fn set_profile<P>(&mut self, profile: P) where P: Into<String> {
        self.profile = profile.into();
    }
}

impl AwsCredentialsProvider for ProfileProvider {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
    	parse_credentials_file(self.location()).and_then(|mut profiles| {
            profiles.remove(self.profile()).ok_or(CredentialsError::new("Profile not found."))
    	})
   }
}

fn parse_credentials_file(location: &Path)
    -> Result<HashMap<String, AwsCredentials>, CredentialsError> {
    match fs::metadata(location) {
        Err(_) => return Err(CredentialsError::new("Could not stat credentials file.")),
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(CredentialsError::new("Could not open file."));
            }
        }
    };

    let file = try!(File::open(location));

    let profile_regex = Regex::new(r"^\[([^\]]+)\]$").unwrap();
    let mut profiles: HashMap<String, AwsCredentials> = HashMap::new();
    let mut access_key_id: Option<String> = None;
    let mut secret_access_key: Option<String> = None;
    let mut profile_name: Option<String> = None;

    let file_lines = BufReader::new(&file);
    for line in file_lines.lines() {
        let unwrapped_line : String = line.unwrap();
        if unwrapped_line.starts_with('#') {
            // Ignore comments
            continue;
        }

        // handle the opening of named profile blocks
        if profile_regex.is_match(&unwrapped_line) {
            if profile_name.is_some() && access_key_id.is_some() && secret_access_key.is_some() {
                let creds = AwsCredentials::new(
                                access_key_id.unwrap(),
                                secret_access_key.unwrap(),
                                None,
                                in_ten_minutes());
                profiles.insert(profile_name.unwrap(), creds);
            }

            access_key_id = None;
            secret_access_key = None;

            let caps = profile_regex.captures(&unwrapped_line).unwrap();
            profile_name = Some(caps.at(1).unwrap().to_string());
            continue;
        }

        // otherwise look for key=value pairs we care about
        let lower_case_line = unwrapped_line.to_ascii_lowercase().to_string();

        if lower_case_line.contains("aws_access_key_id") &&
            access_key_id.is_none()
        {
            let v: Vec<&str> = unwrapped_line.split('=').collect();
            if !v.is_empty() {
                access_key_id = Some(v[1].trim_matches(' ').to_string());
            }
        } else if lower_case_line.contains("aws_secret_access_key") &&
            secret_access_key.is_none()
        {
            let v: Vec<&str> = unwrapped_line.split('=').collect();
            if !v.is_empty() {
                secret_access_key = Some(v[1].trim_matches(' ').to_string());
            }
        }
    }

    if profile_name.is_some() && access_key_id.is_some() && secret_access_key.is_some() {
        let creds = AwsCredentials::new(
                        access_key_id.unwrap(),
                        secret_access_key.unwrap(),
                        None,
                        in_ten_minutes());
        profiles.insert(profile_name.unwrap(), creds);
    }

    if profiles.is_empty() {
        return Err(CredentialsError::new("No credentials found."));
    }

    Ok(profiles)
}

/// Provides AWS credentials from a resource's IAM role. Note: This is not fully tested.
#[derive(Clone, Debug)]
pub struct IamProvider;

impl AwsCredentialsProvider for IamProvider {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        let mut address : String =
                "http://169.254.169.254/latest/meta-data/iam/security-credentials".to_string();
        let mut client = Client::new();
        client.set_read_timeout(Some(StdDuration::from_secs(15)));
        let mut response;
        match client.get(&address)
            .header(Connection::close()).send() {
                Err(_) => return Err(CredentialsError::new(
                                        "Couldn't connect to metadata service")),
                Ok(received_response) => response = received_response
            };

        let mut body = String::new();
        if let Err(_) = response.read_to_string(&mut body) {
			return Err(CredentialsError::new(
                            "Didn't get a parsable response body from metadata service"));
        }

        address.push_str("/");
        address.push_str(&body);
        body = String::new();
        match client.get(&address)
            .header(Connection::close()).send() {
                Err(_) => return Err(CredentialsError::new(
                            "Didn't get a parseable response body from instance role details")),
                Ok(received_response) => response = received_response
            };

        if let Err(_) = response.read_to_string(&mut body) {
            return Err(CredentialsError::new("Had issues with reading iam role response: {}"));
        }

        let json_object: Value;
        match from_str(&body) {
            Err(_) => return Err(CredentialsError::new("Couldn't parse metadata response body.")),
            Ok(val) => json_object = val
        };

        let access_key_id;
        match json_object.find("AccessKeyId") {
            None => return Err(CredentialsError::new("Couldn't find AccessKeyId in response.")),
            Some(val) => access_key_id =
                            val.as_str()
                            .expect("AccessKeyId value was not a string")
                            .to_owned()
                            .replace("\"", "")
        };

        let secret_access_key;
        match json_object.find("SecretAccessKey") {
            None => return Err(CredentialsError::new(
                                        "Couldn't find SecretAccessKey in response.")),
            Some(val) => secret_access_key =
                            val.as_str()
                            .expect("SecretAccessKey value was not a string")
                            .to_owned()
                            .replace("\"", "")
        };

        let expiration;
        match json_object.find("Expiration") {
            None => return Err(CredentialsError::new("Couldn't find Expiration in response.")),
            Some(val) => expiration =
                            val.as_str()
                            .expect("Expiration value was not a string")
                            .to_owned()
                            .replace("\"", "")
        };

        let expiration_time = try!(expiration.parse());

        let token_from_response;
        match json_object.find("Token") {
            None => return Err(CredentialsError::new("Couldn't find Token in response.")),
            Some(val) => token_from_response =
                            val.as_str()
                            .expect("Token value was not a string")
                            .to_owned()
                            .replace("\"", "")
        };

        Ok(AwsCredentials::new(
                    access_key_id,
                    secret_access_key,
                    Some(token_from_response),
                    expiration_time))
    }
}

/// Wrapper for AwsCredentialsProvider that caches the credentials returned by the
/// wrapped provider.  Each time the credentials are accessed, they are checked to see if
/// they have expired, in which case they are retrieved from the wrapped provider again.
pub struct BaseAutoRefreshingProvider<P, T> {
	pub credentials_provider: P,
	cached_credentials: T
}

/// Threadsafe AutoRefreshingProvider that locks cached credentials with a Mutex
pub type AutoRefreshingProviderSync<P> = BaseAutoRefreshingProvider<P, Mutex<AwsCredentials>>;

impl <P: AwsCredentialsProvider> AutoRefreshingProviderSync<P> {
    pub fn with_mutex(provider: P) -> Result<AutoRefreshingProviderSync<P>, CredentialsError> {
		let creds = try!(provider.credentials());
		Ok(BaseAutoRefreshingProvider {
			credentials_provider: provider,
			cached_credentials: Mutex::new(creds)
		})
	}
}

impl <P: AwsCredentialsProvider> AwsCredentialsProvider for
                                    BaseAutoRefreshingProvider<P, Mutex<AwsCredentials>> {
	fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
		let mut creds = self.cached_credentials.lock().unwrap();
		if creds.credentials_are_expired() {
			*creds = try!(self.credentials_provider.credentials());
		}
		Ok(creds.clone())
	}
}

/// !Sync AutoRefreshingProvider that caches credentials in a RefCell
pub type AutoRefreshingProvider<P> = BaseAutoRefreshingProvider<P, RefCell<AwsCredentials>>;

impl <P: AwsCredentialsProvider> AutoRefreshingProvider<P> {
	pub fn with_refcell(provider: P) -> Result<AutoRefreshingProvider<P>, CredentialsError> {
		let creds = try!(provider.credentials());
		Ok(BaseAutoRefreshingProvider {
			credentials_provider: provider,
			cached_credentials: RefCell::new(creds)
		})
	}
}

impl <P: AwsCredentialsProvider> AwsCredentialsProvider for BaseAutoRefreshingProvider<P, RefCell<AwsCredentials>> {
	fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
		let mut creds = self.cached_credentials.borrow_mut();
		if creds.credentials_are_expired() {
			*creds = try!(self.credentials_provider.credentials());
		}
		Ok(creds.clone())
	}
}

/// The credentials provider you probably want to use if you don't require Sync for your
/// AWS services. Wraps a ChainProvider in an AutoRefreshingProvider that uses a RefCell to cache
/// credentials.
///
/// The underlying ChainProvider checks multiple sources for credentials, and the
/// AutoRefreshingProvider refreshes the credentials automatically when they expire. The RefCell
/// allows this caching to happen without the overhead of a Mutex, but is !Sync.
///
/// For a Sync implementation of the same, see DefaultCredentialsProviderSync
pub type DefaultCredentialsProvider = AutoRefreshingProvider<ChainProvider>;

impl DefaultCredentialsProvider {
    pub fn new(parameters_provider: Option<ParametersProvider>)
        -> Result<DefaultCredentialsProvider, CredentialsError> {
        Ok(try!(AutoRefreshingProvider::with_refcell(ChainProvider::new(parameters_provider))))
    }
}

/// The credentials provider you probably want to use if you do require your AWS services sync.
/// Wraps a ChainProvider in an AutoRefreshingProvider that uses a Mutex to lock credentials in a
/// threadsafe manner.
///
/// The underlying ChainProvider checks multiple sources for credentials, and the
/// AutoRefreshingProvider refreshes the credentials automatically when they expire.  The Mutex
/// allows this caching to happen in a Sync manner, incurring the overhead of a Mutex when
/// credentials expire and need to be refreshed.
///
/// For a !Sync implementation of the same, see DefaultCredentialsProvider
pub type DefaultCredentialsProviderSync = AutoRefreshingProviderSync<ChainProvider>;

impl DefaultCredentialsProviderSync {
    pub fn new(parameters_provider: Option<ParametersProvider>)
        -> Result<DefaultCredentialsProviderSync, CredentialsError> {
        Ok(try!(AutoRefreshingProviderSync::with_mutex(ChainProvider::new(parameters_provider))))
    }
}

/// Provides AWS credentials from multiple possible sources using a priority order.
///
/// The following sources are checked in order for credentials when calling `credentials`:
///
/// 1. Environment variables: `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`
/// 2. Parameters option. This is set in your code however you wish to set it. For example,
///    you could read from your own config file and set them or however.
/// 3. AWS credentials file. Usually located at `~/.aws/credentials`.
/// 4. IAM instance profile. Will only work if running on an EC2 instance with an instance
///    profile/role.
///
/// If the sources are exhausted without finding credentials, an error is returned.
/// NB: If the chain makes it to the IAM provider then TCP timeout may cause a wait.
#[derive(Debug, Clone)]
pub struct ChainProvider {
    parameters_provider: Option<ParametersProvider>,
    profile_provider: Option<ProfileProvider>,
}

impl AwsCredentialsProvider for ChainProvider {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
    	EnvironmentProvider.credentials()
            .or_else(|_| {
                match self.parameters_provider {
                    Some(ref provider) => provider.credentials(),
                    None => Err(CredentialsError::new(""))
                }
            })
    		.or_else(|_| {
                match self.profile_provider {
                    Some(ref provider) => provider.credentials(),
                    None => Err(CredentialsError::new(""))
                }
            })
    		.or_else(|_| IamProvider.credentials())
    		.or_else(|_| Err(CredentialsError::new(
                "Couldn't find AWS credentials in environment, credentials file, or IAM role.")))
    }
}

impl ChainProvider {
    /// Create a new `ChainProvider` using a `ParametersProvider` with the default settings.
    pub fn new(parameters_provider: Option<ParametersProvider>) -> ChainProvider {
        ChainProvider {
            parameters_provider: parameters_provider,
            profile_provider: ProfileProvider::new().ok(),
        }
    }

    /// Create a new `ChainProvider` using the provided `ParametersProvider`.
    pub fn with_param_provider(&self,
        parameters_provider: ParametersProvider
        ) -> ChainProvider {
        ChainProvider {
            parameters_provider: Some(parameters_provider),
            profile_provider: None,
        }
    }

    /// Create a new `ChainProvider` using the provided `ProfileProvider`.
    pub fn with_profile_provider(&self,
        profile_provider: ProfileProvider
        ) -> ChainProvider {
        ChainProvider {
            parameters_provider: None,
            profile_provider: Some(profile_provider),
        }
    }
}

// Basic internal function for returning the time ten minutes from now.
fn in_ten_minutes() -> DateTime<UTC> {
    UTC::now() + Duration::seconds(600)
}
