#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceStatusGuestConfigurationService {
    /// The behavior of the service when the Arc-enabled machine starts up.
    #[builder(into)]
    #[serde(rename = "startupType")]
    pub r#startup_type: String,
    /// The current status of the service.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
}
