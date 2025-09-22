#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OpenZfsVolumeNfsExports {
    /// A list of configuration objects that contain the client and options for mounting the OpenZFS file system. Maximum of 25 items. See `client_configurations` Block below for details.
    #[builder(into)]
    #[serde(rename = "clientConfigurations")]
    pub r#client_configurations: Vec<super::super::types::fsx::OpenZfsVolumeNfsExportsClientConfiguration>,
}
