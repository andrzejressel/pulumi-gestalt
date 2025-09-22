#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ParameterGroupParameter {
    /// The apply method of the Neptune parameter. Valid values are `immediate` and `pending-reboot`. Defaults to `pending-reboot`.
    #[builder(into)]
    #[serde(rename = "applyMethod")]
    pub r#apply_method: Option<String>,
    /// The name of the Neptune parameter.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The value of the Neptune parameter.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
