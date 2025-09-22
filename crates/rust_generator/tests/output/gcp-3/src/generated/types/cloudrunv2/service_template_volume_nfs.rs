#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTemplateVolumeNfs {
    /// Path that is exported by the NFS server.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// If true, mount the NFS volume as read only
    #[builder(into)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Option<bool>,
    /// Hostname or IP address of the NFS server
    #[builder(into)]
    #[serde(rename = "server")]
    pub r#server: String,
}
