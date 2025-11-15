#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceMode {
    /// When `true`, tasks will run on every worker node. Conflicts with `replicated`
    #[builder(into)]
    #[serde(rename = "global")]
    pub r#global: Option<bool>,
    /// The replicated service mode
    #[builder(into)]
    #[serde(rename = "replicated")]
    pub r#replicated: Option<Box<super::types::ServiceModeReplicated>>,
}
