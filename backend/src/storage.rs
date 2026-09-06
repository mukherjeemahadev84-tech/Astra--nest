use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;

pub async fn upload_report_file(
    client: &Client,
    bucket: &str,
    key: &str,
    bytes: Vec<u8>,
    content_type: &str,
) -> Result<(), aws_sdk_s3::Error> {
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(bytes))
        .content_type(content_type)
        .send()
        .await?;
    Ok(())
}
