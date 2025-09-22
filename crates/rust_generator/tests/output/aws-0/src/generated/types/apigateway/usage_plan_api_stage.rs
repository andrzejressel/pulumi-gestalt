#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UsagePlanApiStage {
    /// API Id of the associated API stage in a usage plan.
    #[builder(into)]
    #[serde(rename = "apiId")]
    pub r#api_id: String,
    /// API stage name of the associated API stage in a usage plan.
    #[builder(into)]
    #[serde(rename = "stage")]
    pub r#stage: String,
    /// The throttling limits of the usage plan.
    #[builder(into)]
    #[serde(rename = "throttles")]
    pub r#throttles: Option<Vec<super::super::types::apigateway::UsagePlanApiStageThrottle>>,
}
