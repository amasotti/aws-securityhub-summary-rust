use crate::handler::init_report_section;
use crate::utils::{add_horizontal_line, HorizontalLineType};
use aws_sdk_securityhub::client::Client as SecurityHubClient;
use aws_sdk_securityhub::types::InsightResultValue;
use lambda_runtime::Error;

/// Get the insight results for the given insight ARN
///
/// # Arguments
///
/// * `sec_hub_client` - The Security Hub client
/// * `arn` - The insight ARN
///
/// # Returns
///
/// * A vector of InsightResultValue
pub async fn get_insight_results(
    sec_hub_client: &SecurityHubClient,
    arn: &String,
) -> Result<Vec<InsightResultValue>, Error> {
    let response = sec_hub_client
        .get_insight_results()
        .insight_arn(arn)
        .send()
        .await?;

    let insight_results = response.insight_results().unwrap().result_values().to_vec();
    Ok(insight_results)
}

pub async fn process_insights(
    insight_arns: Vec<String>,
    region: String,
    sec_hub_client: &SecurityHubClient,
    sns_body: &mut String,
) -> Result<(), Error> {
    for (i, arn) in insight_arns.iter().enumerate() {
        // Get the insight results

        let insight_results = get_insight_results(sec_hub_client, arn).await?;

        init_report_section(sns_body, i).await;

        if insight_results.is_empty() {
            sns_body.push_str("NO RESULTS \n");
        }

        let total_rows = if insight_results.len() > 15 {
            15
        } else {
            insight_results.len()
        };
        let first_section = i == 0;
        let insight_results = insight_results.into_iter().rev().collect::<Vec<_>>();

        for result in insight_results.iter().take(total_rows) {
            let count_str = match result.count() {
                Some(count) => count.to_string(),
                None => "None".to_string(),
            };

            let group_by_value = result
                .group_by_attribute_value()
                .unwrap_or("No result to map");

            sns_body.push_str(&format!(
                "{}\t - \t{}{}\n",
                count_str,
                if first_section { "TOTAL " } else { "" },
                group_by_value
            ));
        }

        add_horizontal_line(sns_body, HorizontalLineType::Single).await;
        sns_body.push_str(" \n");

        let insight_link = format!(
            "View on AWS: https://{}.console.aws.amazon.com/securityhub/home?region={}#/insights/{}",
            region, region, arn
        );
        sns_body.push_str(&insight_link);
        sns_body.push_str(" \n\n");
    }
    Ok(())
}
