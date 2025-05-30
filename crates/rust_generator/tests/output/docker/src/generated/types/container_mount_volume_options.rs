#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ContainerMountVolumeOptions {
    /// Name of the driver to use to create the volume.
    #[builder(into, default)]
    #[serde(rename = "driverName")]
    pub r#driver_name: Box<Option<String>>,
    /// key/value map of driver specific options.
    #[builder(into, default)]
    #[serde(rename = "driverOptions")]
    pub r#driver_options: Box<Option<std::collections::HashMap<String, String>>>,
    /// User-defined key/value metadata.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<Vec<super::types::ContainerMountVolumeOptionsLabel>>>,
    /// Populate volume with data from the target.
    #[builder(into, default)]
    #[serde(rename = "noCopy")]
    pub r#no_copy: Box<Option<bool>>,
}
