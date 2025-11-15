#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceMemcacheParameters {
    /// (Output)
    /// This is a unique ID associated with this set of parameters.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// User-defined set of parameters to use in the memcache process.
    #[builder(into)]
    #[serde(rename = "params")]
    pub r#params: Option<std::collections::HashMap<String, String>>,
}
