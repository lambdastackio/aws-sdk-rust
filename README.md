# AWS SDK for Rust
## BETA!
AWS SDK with initial emphasis on S3. Supports both V2 and V4 authentication signatures. Allows for custom app config, environment variables, standard /.aws/credentials and future Iam credentials. No environmental assumptions or opinions are made on how your access to AWS should be handled.

## S3
The initial access is for solid AWS S3 support. The SDK is built to allow non-aws environments that support an S3 API interface such as Ceph and other object stores to accessible from Rust.

## Proxy
In many corporate environments the use of proxies are mandatory. Proxies are usually handled by setting the following environment variables:
```
http_proxy=<whatever your proxy url>:<whatever port if any>
https_proxy=<whatever your proxy url>:<whatever port if any>
no_proxy=<whatever IPs, domains, hosts, etc that should not go through a proxy (separate by commas)>
```

You can also pass in the proxy settings via Url to the S3Client or pass in `None`. This will allow you to use your own config file if desired so that you can read in the proxy settings and pass them S3Client.

## Configuring Credentials

Before using the SDK, ensure that you've configured credentials. The best
way to configure credentials on a development machine is to use the
`~/.aws/credentials` file, which might look like:

```
[default]
aws_access_key_id = <whatever access_key_id>
aws_secret_access_key = <whatever secret_access_key>
```

You can learn more about the credentials file from this
[blog post](http://blogs.aws.amazon.com/security/post/Tx3D6U6WSFGOK2H/A-New-and-Standardized-Way-to-Manage-Credentials-in-the-AWS-SDKs).

Alternatively, you can set the following environment variables:

```
AWS_ACCESS_KEY_ID=<whatever access_key_id>
AWS_SECRET_ACCESS_KEY=<whatever secret_access_key>
```

## Using the Rust SDK

To use a service in the SDK, create a service variable by calling the `S3Client::new(...)`
function. Once you have a service client, you can call API operations which each
return response data and a possible error.

To list buckets from S3, you could run:

```rust
// NOTE: See the src/main.rs for more examples...

extern crate aws_sdk_rust;
extern crate url;
extern crate hyper;

use aws_sdk_rust::aws::common::credentials::DefaultCredentialsProvider;
use aws_sdk_rust::aws::common::region::Region;
use aws_sdk_rust::aws::s3::endpoint::Endpoint;
use aws_sdk_rust::aws::s3::s3client::S3Client;

fn main() {
    // DefaultCredentialsProvider will end up cycling through the credentials provider list in
    // the following order:
    // 1. Environment - Checks the envrionment variables:
    //      AWS_ACCESS_KEY_ID
    //      AWS_SECRET_ACCESS_KEY
    //      AWS_SESSION_TOKEN
    // 2. Parameters passed in via ParametersProvider (see example below)
    // 3. Profile provider - ~/.aws/credentials
    // 4. IAM Provider

    // Option to initialize the ParametersProvider
    /* Example of using parameters for passing the credentials.
    use aws_sdk_rust::aws::common::credentials::{DefaultCredentialsProvider, ParametersProvider};

    let param_provider: Option<ParametersProvider>;
    param_provider = Some(
        ParametersProvider::with_params(
            "<whatever your access_key_id>",
            "<whatever your secret_access_key>",
            None).unwrap()
    );

    let provider = DefaultCredentialsProvider::new(param_provider).unwrap();
    */

    // Allow the defaults w/o ParametersProvider - pass in 'None' in ::new(None)
    let provider = DefaultCredentialsProvider::new(None).unwrap();

    // V4 is the default signature for AWS. However, other systems also use V2.
    let endpoint = Endpoint::new(Region::UsEast1, Signature::V4, None, None);
    let client = S3Client::new(provider, endpoint);

    match client.list_buckets() {
      Ok(output) => {
        println!("{:#?}", output);
      }
      Err(error) => {
        println!("Error: {:#?}", error);
      }
    }
}
```

## Inspiration
If you need access to other AWS systems instead of S3 then take a look at rusoto (https://github.com/rusoto/rusoto). Initial inspiration comes from there, Python Boto3 and aws-sdk-go.

## Library Development
### Paths
Structure is broken down into the following paths:

common

errors

s3

### XML
AWS S3 uses XML for transmitting and receiving (headers too). At this time there are no good implementations of
XML Serialization (serde works for Deserialization) so each XML construct for sending and receiving from S3
had to be defined with a parser/writer (project rusoto did a lot of the heavy lifting in this area).

Other AWS Services primarily use JSON and De/Serialization is possible for those service.
