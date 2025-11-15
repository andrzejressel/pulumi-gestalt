#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInstanceBootDiskInitializeParam {
    /// A flag to enable confidential compute mode on boot disk
    #[builder(into)]
    #[serde(rename = "enableConfidentialCompute")]
    pub r#enable_confidential_compute: bool,
    /// The image from which this disk was initialised.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// A set of key/value label pairs assigned to the disk.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: std::collections::HashMap<String, String>,
    /// Indicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle.
    #[builder(into)]
    #[serde(rename = "provisionedIops")]
    pub r#provisioned_iops: i32,
    /// Indicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle.
    #[builder(into)]
    #[serde(rename = "provisionedThroughput")]
    pub r#provisioned_throughput: i32,
    /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty.
    #[builder(into)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: std::collections::HashMap<String, String>,
    /// A list of self_links to resource policies attached to the selected `boot_disk`
    #[builder(into)]
    #[serde(rename = "resourcePolicies")]
    pub r#resource_policies: Vec<String>,
    /// The size of the image in gigabytes.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: i32,
    /// The URL of the storage pool in which the new disk is created
    #[builder(into)]
    #[serde(rename = "storagePool")]
    pub r#storage_pool: String,
    /// The accelerator type resource exposed to this instance. E.g. `nvidia-tesla-k80`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
