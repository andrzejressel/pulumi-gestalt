#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationAppSource {
    /// Password to use when authenticating to the source. This provider cannot perform drift detection of this configuration.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// For sources that are version-aware, the revision to use.
    #[builder(into)]
    #[serde(rename = "revision")]
    pub r#revision: Option<String>,
    /// SSH key to use when authenticating to the source. This provider cannot perform drift detection of this configuration.
    #[builder(into)]
    #[serde(rename = "sshKey")]
    pub r#ssh_key: Option<String>,
    /// The type of source to use. For example, "archive".
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// The URL where the app resource can be found.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// Username to use when authenticating to the source.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
