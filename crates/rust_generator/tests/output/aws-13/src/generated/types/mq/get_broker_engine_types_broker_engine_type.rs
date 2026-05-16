#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBrokerEngineTypesBrokerEngineType {
    /// The MQ engine type to return version details for.
    #[builder(into)]
    #[serde(rename = "engineType")]
    pub r#engine_type: String,
    /// The list of engine versions.
    #[builder(into)]
    #[serde(rename = "engineVersions")]
    pub r#engine_versions: Vec<super::super::types::mq::GetBrokerEngineTypesBrokerEngineTypeEngineVersion>,
}
