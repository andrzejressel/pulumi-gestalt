#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionLockConfig {
    /// Indicates whether or not the connection is locked.
    #[builder(into)]
    #[serde(rename = "locked")]
    pub r#locked: bool,
    /// Describes why a connection is locked.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: Option<String>,
}
