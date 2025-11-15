#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IpGroupRule {
    /// The description of the IP group.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The IP address range, in CIDR notation, e.g., `10.0.0.0/16`
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: String,
}
