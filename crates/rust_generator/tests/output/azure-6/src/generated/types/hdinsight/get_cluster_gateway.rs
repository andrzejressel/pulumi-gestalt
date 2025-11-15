#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterGateway {
    /// Is the Ambari Portal enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// The password used for the Ambari Portal.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// The username used for the Ambari Portal.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: String,
}
