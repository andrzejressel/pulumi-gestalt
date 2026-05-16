#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
