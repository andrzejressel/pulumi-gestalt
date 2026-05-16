#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelHlsIngestIngestEndpoint {
    /// The password
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// The URL
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// The username
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
