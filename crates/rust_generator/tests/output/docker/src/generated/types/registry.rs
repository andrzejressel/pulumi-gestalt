#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct Registry {
    /// The password to authenticate to the registry. Does not cause image rebuild when changed.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// The URL of the Docker registry server
    #[builder(into)]
    #[serde(rename = "server")]
    pub r#server: Option<String>,
    /// The username to authenticate to the registry. Does not cause image rebuild when changed.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
