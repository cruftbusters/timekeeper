use std::env;
use std::error::Error;

use aws_sdk_s3::Client;
use aws_sdk_s3::config::Region;
use custom_s3_client::CustomS3Client;

mod custom_s3_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        return Ok(());
    }

    let client = CustomS3Client::new({
        let shared_config = aws_config::load_from_env().await;
        let config = aws_sdk_s3::config::Builder::from(&shared_config)
            .region(Region::new("us-west-1"))
            .build();

        Client::from_conf(config)
    });

    match args[1].as_str() {
        "create" => client.create_bucket().await?,
        "delete" => client.delete_bucket().await?,
        _ => print_usage(&args[0])
    };

    Ok(())
}

fn print_usage(bin: &str) {
    println!("usage: ");
    println!("  create deployment (s3):");
    println!("    {bin} create");
    println!();
    println!("  delete deployment (s3):");
    println!("    {bin} delete");
}
