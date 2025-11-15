#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInstanceGroupNamedPort {
    /// The name of the instance group. Either `name` or `self_link` must be provided.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
}
