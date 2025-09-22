#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetImagesImage {
    /// One or more `data_disk` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "dataDisks")]
    pub r#data_disks: Vec<super::super::types::compute::GetImagesImageDataDisk>,
    /// The supported Azure location where the Image exists.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// The name of the Image.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// An `os_disk` block as defined below.
    #[builder(into)]
    #[serde(rename = "osDisks")]
    pub r#os_disks: Vec<super::super::types::compute::GetImagesImageOsDisk>,
    /// A mapping of tags assigned to the Image.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: std::collections::HashMap<String, String>,
    /// Is zone resiliency enabled?
    #[builder(into)]
    #[serde(rename = "zoneResilient")]
    pub r#zone_resilient: bool,
}
