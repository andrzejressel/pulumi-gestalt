#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyBasedRouteWarning {
    /// (Output)
    /// A warning code, if applicable.
    #[builder(into)]
    #[serde(rename = "code")]
    pub r#code: Option<String>,
    /// (Output)
    /// Metadata about this warning in key: value format. The key should provides more detail on the warning being returned. For example, for warnings where there are no results in a list request for a particular zone, this key might be scope and the key value might be the zone name. Other examples might be a key indicating a deprecated resource and a suggested replacement.
    #[builder(into)]
    #[serde(rename = "data")]
    pub r#data: Option<std::collections::HashMap<String, String>>,
    /// (Output)
    /// A human-readable description of the warning code.
    #[builder(into)]
    #[serde(rename = "warningMessage")]
    pub r#warning_message: Option<String>,
}
