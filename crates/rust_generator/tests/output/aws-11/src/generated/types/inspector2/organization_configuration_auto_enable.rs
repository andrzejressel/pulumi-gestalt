#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OrganizationConfigurationAutoEnable {
    /// Whether Amazon EC2 scans are automatically enabled for new members of your Amazon Inspector organization.
    #[builder(into)]
    #[serde(rename = "ec2")]
    pub r#ec_2: bool,
    /// Whether Amazon ECR scans are automatically enabled for new members of your Amazon Inspector organization.
    #[builder(into)]
    #[serde(rename = "ecr")]
    pub r#ecr: bool,
    /// Whether Lambda Function scans are automatically enabled for new members of your Amazon Inspector organization.
    #[builder(into)]
    #[serde(rename = "lambda")]
    pub r#lambda: Option<bool>,
    /// Whether AWS Lambda code scans are automatically enabled for new members of your Amazon Inspector organization. **Note:** Lambda code scanning requires Lambda standard scanning to be activated. Consequently, if you are setting this argument to `true`, you must also set the `lambda` argument to `true`. See [Scanning AWS Lambda functions with Amazon Inspector](https://docs.aws.amazon.com/inspector/latest/user/scanning-lambda.html#lambda-code-scans) for more information.
    #[builder(into)]
    #[serde(rename = "lambdaCode")]
    pub r#lambda_code: Option<bool>,
}
