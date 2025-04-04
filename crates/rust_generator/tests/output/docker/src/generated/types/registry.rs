#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct Registry {
    /// The password to authenticate to the registry. Does not cause image rebuild when changed.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The URL of the Docker registry server
    #[builder(into, default)]
    #[serde(rename = "server")]
    pub r#server: Box<Option<String>>,
    /// The username to authenticate to the registry. Does not cause image rebuild when changed.
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
