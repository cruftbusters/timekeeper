use aws_sdk_s3::Client;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::create_bucket::CreateBucketError;
use aws_sdk_s3::types::{BucketLocationConstraint, CreateBucketConfiguration};
use aws_sdk_s3::operation::delete_bucket::DeleteBucketError;

pub struct CustomS3Client {
    client: Client,
}

impl CustomS3Client {
    pub fn new(client: Client) -> CustomS3Client {
        CustomS3Client {
            client
        }
    }

    pub async fn create_bucket(&self) -> Result<(), SdkError<CreateBucketError>> {
        let constraint = BucketLocationConstraint::from("us-west-1");

        let create_bucket_configuration = CreateBucketConfiguration::builder()
            .location_constraint(constraint)
            .build();

        let output = self.client
            .create_bucket()
            .create_bucket_configuration(create_bucket_configuration)
            .bucket("timekeeper.cruftbusters.com")
            .send()
            .await?;

        println!("{:?}", output);

        Ok(())
    }

    pub async fn delete_bucket(&self) -> Result<(), SdkError<DeleteBucketError>> {
        let output = self.client
            .delete_bucket()
            .bucket("timekeeper.cruftbusters.com")
            .send()
            .await?;

        println!("{:?}", output);

        Ok(())
    }
}
