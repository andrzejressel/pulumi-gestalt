#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceGroupManagerStatusAllInstancesConfig {
    /// Current all-instances configuration revision. This value is in RFC3339 text format.
    #[builder(into)]
    #[serde(rename = "currentRevision")]
    pub r#current_revision: Option<String>,
    /// A bit indicating whether this configuration has been applied to all managed instances in the group.
    #[builder(into)]
    #[serde(rename = "effective")]
    pub r#effective: Option<bool>,
}
