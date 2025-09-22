#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionLockConfig {
    /// Indicates whether or not the connection is locked.
    #[builder(into)]
    #[serde(rename = "locked")]
    pub r#locked: bool,
    /// Describes why a connection is locked.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: Option<String>,
}
