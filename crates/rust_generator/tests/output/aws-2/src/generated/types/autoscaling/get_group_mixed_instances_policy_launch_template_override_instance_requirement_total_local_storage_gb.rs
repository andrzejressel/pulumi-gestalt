#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGroupMixedInstancesPolicyLaunchTemplateOverrideInstanceRequirementTotalLocalStorageGb {
    /// Maximum.
    #[builder(into)]
    #[serde(rename = "max")]
    pub r#max: f64,
    /// Minimum.
    #[builder(into)]
    #[serde(rename = "min")]
    pub r#min: f64,
}
