use std::env;

pub struct EnvVars {
    pub sns_topic_arn: String,
    pub insight_arns: Vec<String>,
    pub region: String,
}

pub fn get_env_vars() -> EnvVars {
    EnvVars {
        sns_topic_arn: env::var("SNSTopic").expect("SNSTopic environment variable is not set"),
        insight_arns: vec![
            env::var("ARNInsight01").expect("ARNInsight01 environment variable is not set"),
            env::var("ARNInsight02").expect("ARNInsight02 environment variable is not set"),
            env::var("ARNInsight03").expect("ARNInsight03 environment variable is not set"),
            env::var("ARNInsight04").expect("ARNInsight04 environment variable is not set"),
            env::var("ARNInsight05").expect("ARNInsight05 environment variable is not set"),
            env::var("ARNInsight06").expect("ARNInsight06 environment variable is not set"),
            env::var("ARNInsight07").expect("ARNInsight07 environment variable is not set"),
        ],
        region: env::var("AWS_REGION").unwrap_or(String::from("eu-central-1")),
    }
}
