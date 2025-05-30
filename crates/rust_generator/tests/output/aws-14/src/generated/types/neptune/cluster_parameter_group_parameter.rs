#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterParameterGroupParameter {
    /// Valid values are `immediate` and `pending-reboot`. Defaults to `pending-reboot`.
    #[builder(into, default)]
    #[serde(rename = "applyMethod")]
    pub r#apply_method: Box<Option<String>>,
    /// The name of the neptune parameter.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value of the neptune parameter.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
