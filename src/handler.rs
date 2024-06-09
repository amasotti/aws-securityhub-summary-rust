use aws_config::BehaviorVersion;
use aws_sdk_securityhub::client::Client as SecurityHubClient;
use aws_sdk_sns::client::Client as SnsClient;
use lambda_runtime::{Error, LambdaEvent};

use crate::env::get_env_vars;
use crate::model::{Request, Response};
use crate::securityhub::process_insights;
use crate::sns::push_report_to_sns;
use crate::utils::{add_horizontal_line, HorizontalLineType};

/// Initialize the report header
pub async fn init_report_header() -> String {
    let mut sns_body = String::new();
    add_horizontal_line(&mut sns_body, HorizontalLineType::Single).await;
    sns_body.push_str("Security Hub Report \n");
    add_horizontal_line(&mut sns_body, HorizontalLineType::Single).await;
    sns_body.push_str("\n\n");
    sns_body
}

/// Initialize a new section in the report
///
/// # Arguments
///
/// * `sns_body` - The SNS message body
/// * `i` - The index of the section, which must correspond to the INSIGHTS_LABEL
pub async fn init_report_section(sns_body: &mut String, i: usize) {
    let insight_labels: Vec<&str> = vec![
        "AWS Foundational Security Best Practices security checks:",
        "AWS Foundational Security Best Practices failed security checks by severity:",
        "GuardDuty threat detection findings by severity:",
        "IAM Access Analyzer findings by severity:",
        "Unresolved findings by severity:",
        "New findings in the last 7 days:",
        "Top 10 Resource Types with findings:",
    ];

    sns_body.push_str(insight_labels[i]);
    sns_body.push('\n');
    add_horizontal_line(sns_body, HorizontalLineType::Single).await;
}

/// Add a horizontal line to the given text body to separate sections
pub async fn finalize_report(sns_body: &mut String) {
    sns_body.push('\n');
    add_horizontal_line(sns_body, HorizontalLineType::Single).await;
}

pub async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Get the environment variables
    let env_vars = get_env_vars();

    let sns_topic_arn = env_vars.sns_topic_arn;
    let insight_arns = env_vars.insight_arns;
    let region = env_vars.region;

    // Init AWS clients
    let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;
    let sec_hub_client = SecurityHubClient::new(&config);
    let sns_client = SnsClient::new(&config);

    // Initialize the SNS message body
    let mut sns_body = init_report_header().await;

    process_insights(insight_arns, region, &sec_hub_client, &mut sns_body).await?;

    finalize_report(&mut sns_body).await;
    log::trace!("SNS Body: {}", sns_body);
    println!("SNS Body: {}", sns_body);

    let resp = push_report_to_sns(event, sns_topic_arn, sns_client, sns_body).await?;
    Ok(resp)
}
