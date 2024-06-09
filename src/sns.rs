use crate::model::{Request, Response};
use aws_sdk_sns::client::Client;
use lambda_runtime::{Error, LambdaEvent};

pub async fn push_report_to_sns(
    event: LambdaEvent<Request>,
    sns_topic_arn: String,
    sns_client: Client,
    sns_body: String,
) -> Result<Response, Error> {
    sns_client
        .publish()
        .topic_arn(&sns_topic_arn)
        .message(&sns_body)
        .send()
        .await?;

    let resp = Response {
        req_id: event.context.request_id,
        msg: String::from("Security Hub report generated and sent."),
    };
    Ok(resp)
}
