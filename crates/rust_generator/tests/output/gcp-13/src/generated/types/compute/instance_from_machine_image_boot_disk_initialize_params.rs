#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceFromMachineImageBootDiskInitializeParams {
    /// A flag to enable confidential compute mode on boot disk
    #[builder(into)]
    #[serde(rename = "enableConfidentialCompute")]
    pub r#enable_confidential_compute: Option<bool>,
    /// The image from which this disk was initialised.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Option<String>,
    /// A set of key/value label pairs assigned to the disk.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// Indicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle.
    #[builder(into)]
    #[serde(rename = "provisionedIops")]
    pub r#provisioned_iops: Option<i32>,
    /// Indicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle.
    #[builder(into)]
    #[serde(rename = "provisionedThroughput")]
    pub r#provisioned_throughput: Option<i32>,
    /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty.
    #[builder(into)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: Option<std::collections::HashMap<String, String>>,
    /// A list of self_links of resource policies to attach to the instance's boot disk. Modifying this list will cause the instance to recreate. Currently a max of 1 resource policy is supported.
    #[builder(into)]
    #[serde(rename = "resourcePolicies")]
    pub r#resource_policies: Option<String>,
    /// The size of the image in gigabytes.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Option<i32>,
    /// The URL of the storage pool in which the new disk is created
    #[builder(into)]
    #[serde(rename = "storagePool")]
    pub r#storage_pool: Option<String>,
    /// The Google Compute Engine disk type. Such as pd-standard, pd-ssd or pd-balanced.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
