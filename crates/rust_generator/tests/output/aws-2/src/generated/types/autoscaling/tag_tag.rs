#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TagTag {
    /// Tag name.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// Whether to propagate the tags to instances launched by the ASG.
    #[builder(into)]
    #[serde(rename = "propagateAtLaunch")]
    pub r#propagate_at_launch: bool,
    /// Tag value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
