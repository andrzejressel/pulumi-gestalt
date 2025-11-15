#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTrafficStatus {
    /// (Output)
    /// Specifies percent of the traffic to this Revision.
    #[builder(into)]
    #[serde(rename = "percent")]
    pub r#percent: Option<i32>,
    /// (Output)
    /// Revision to which this traffic is sent.
    #[builder(into)]
    #[serde(rename = "revision")]
    pub r#revision: Option<String>,
    /// (Output)
    /// Indicates the string used in the URI to exclusively reference this target.
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Option<String>,
    /// (Output)
    /// The allocation type for this traffic target.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    /// (Output)
    /// Displays the target URI.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Option<String>,
}
