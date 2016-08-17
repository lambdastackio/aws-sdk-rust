# AWS SDK for Rust
## BETA!
AWS SDK with initial emphasis on S3. Supports both V2 and V4 authentication signatures. Allows for custom app config, environment variables, standard /.aws/credentials and future Iam credentials. No environmental assumptions or opinions are made on how your access to AWS should be handled.

## S3
The initial access is for solid AWS S3 support. The SDK is built to allow non-aws environments that support an S3 API interface such as Ceph and other object stores to accessible from Rust.

## Configuring Credentials

Before using the SDK, ensure that you've configured credentials. The best
way to configure credentials on a development machine is to use the
`~/.aws/credentials` file, which might look like:

```
[default]
aws_access_key_id = AKID1234567890
aws_secret_access_key = MY-SECRET-KEY
```

You can learn more about the credentials file from this
[blog post](http://blogs.aws.amazon.com/security/post/Tx3D6U6WSFGOK2H/A-New-and-Standardized-Way-to-Manage-Credentials-in-the-AWS-SDKs).

Alternatively, you can set the following environment variables:

```
AWS_ACCESS_KEY_ID=AKID1234567890
AWS_SECRET_ACCESS_KEY=MY-SECRET-KEY
```

## Using the Rust SDK

To use a service in the SDK, create a service variable by calling the `S3Client::new()`
function. Once you have a service client, you can call API operations which each
return response data and a possible error.

To list buckets from S3, you could run:

```rust
extern crate aws_sdk_rust;
extern crate url;
extern crate hyper;

use aws_sdk_rust::aws::common::credentials::DefaultCredentialsProvider;
use aws_sdk_rust::aws::common::region::Region;
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
    let client = S3Client::new(provider, Region::UsEast1, "V4", None);

    // If you wish to override the defaults of AWS then you can call the method below before
    // making any requests.
    // client.set_endpoint("<whatever url you want>");
    println!("Endpoint: {}", client.endpoint());

    match client.list_buckets() {
      Ok(output) => {
        println!("{:?}", output);
      }
      Err(error) => {
        println!("Error: {:?}", error);
      }
    }
}
```

## Inspiration
If you need access to other AWS systems instead of S3 then take a look at rusoto (https://github.com/rusoto/rusoto). Initial inspiration comes from there, Python Boto3 and aws-sdk-go.
