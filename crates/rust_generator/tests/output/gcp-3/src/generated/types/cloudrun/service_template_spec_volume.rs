#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTemplateSpecVolume {
    /// A filesystem specified by the Container Storage Interface (CSI).
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "csi")]
    pub r#csi: Option<Box<super::super::types::cloudrun::ServiceTemplateSpecVolumeCsi>>,
    /// Ephemeral storage which can be backed by real disks (HD, SSD), network storage or memory (i.e. tmpfs). For now only in memory (tmpfs) is supported. It is ephemeral in the sense that when the sandbox is taken down, the data is destroyed with it (it does not persist across sandbox runs).
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "emptyDir")]
    pub r#empty_dir: Option<Box<super::super::types::cloudrun::ServiceTemplateSpecVolumeEmptyDir>>,
    /// Volume's name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A filesystem backed by a Network File System share. This filesystem requires the
    /// run.googleapis.com/execution-environment annotation to be unset or set to "gen2"
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "nfs")]
    pub r#nfs: Option<Box<super::super::types::cloudrun::ServiceTemplateSpecVolumeNfs>>,
    /// The secret's value will be presented as the content of a file whose
    /// name is defined in the item path. If no items are defined, the name of
    /// the file is the secret_name.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Option<Box<super::super::types::cloudrun::ServiceTemplateSpecVolumeSecret>>,
}
