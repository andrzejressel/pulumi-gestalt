#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstancePersistenceConfigAofConfig {
    /// Optional. The fsync mode.
    /// Possible values:
    /// NEVER
    /// EVERY_SEC
    /// ALWAYS
    #[builder(into)]
    #[serde(rename = "appendFsync")]
    pub r#append_fsync: Option<String>,
}
