#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpotFleetRequestLaunchTemplateConfigOverrideInstanceRequirementsNetworkInterfaceCount {
    /// Maximum.
    #[builder(into)]
    #[serde(rename = "max")]
    pub r#max: Option<i32>,
    /// Minimum.
    #[builder(into)]
    #[serde(rename = "min")]
    pub r#min: Option<i32>,
}
