# AWS SDK for Rust
## BETA!
AWS SDK with initial emphasis on S3. Supports both V2 and V4 authentication signatures. Allows for custom app config, environment variables, standard /.aws/credentials and future Iam credentials. No environmental assumptions or opinions are made on how your access to AWS should be handled.

## S3
The initial access is for solid AWS S3 support. The SDK is built to allow non-aws environments that support an S3 API interface such as Ceph and other object stores to accessible from Rust.

## Inspiration
If you need access to other AWS systems instead of S3 then take a look at rusoto (https://github.com/rusoto/rusoto). Initial inspiration comes from there, Python Boto3 and aws-sdk-go.
