#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceBootDiskInitializeParams {
    /// Whether this disk is using confidential compute mode.
    /// Note: Only supported on hyperdisk skus, disk_encryption_key is required when setting to true.
    #[builder(into)]
    #[serde(rename = "enableConfidentialCompute")]
    pub r#enable_confidential_compute: Option<bool>,
    /// The image from which to initialize this disk. This can be
    /// one of: the image's `self_link`, `projects/{project}/global/images/{image}`,
    /// `projects/{project}/global/images/family/{family}`, `global/images/{image}`,
    /// `global/images/family/{family}`, `family/{family}`, `{project}/{family}`,
    /// `{project}/{image}`, `{family}`, or `{image}`. If referred by family, the
    /// images names must include the family name. If they don't, use the
    /// [gcp.compute.Image data source](https://www.terraform.io/docs/providers/google/d/compute_image.html).
    /// For instance, the image `centos-6-v20180104` includes its family name `centos-6`.
    /// These images can be referred by family name here.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Option<String>,
    /// A set of key/value label pairs assigned to the disk. This
    /// field is only applicable for persistent disks.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// Indicates how many IOPS to provision for the disk.
    /// This sets the number of I/O operations per second that the disk can handle.
    /// For more details,see the [Hyperdisk documentation](https://cloud.google.com/compute/docs/disks/hyperdisks).
    /// Note: Updating currently is only supported for hyperdisk skus via disk update
    /// api/gcloud without the need to delete and recreate the disk, hyperdisk allows
    /// for an update of IOPS every 4 hours. To update your hyperdisk more frequently,
    /// you'll need to manually delete and recreate it.
    #[builder(into)]
    #[serde(rename = "provisionedIops")]
    pub r#provisioned_iops: Option<i32>,
    /// Indicates how much throughput to provision for the disk.
    /// This sets the number of throughput mb per second that the disk can handle.
    /// For more details,see the [Hyperdisk documentation](https://cloud.google.com/compute/docs/disks/hyperdisks).
    /// Note: Updating currently is only supported for hyperdisk skus via disk update
    /// api/gcloud without the need to delete and recreate the disk, hyperdisk allows
    /// for an update of throughput every 4 hours. To update your hyperdisk more
    /// frequently, you'll need to manually delete and recreate it.
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
    /// The size of the image in gigabytes. If not specified, it
    /// will inherit the size of its base image.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Option<i32>,
    /// The URL or the name of the storage pool in which the new disk is created.
    /// For example:
    /// * https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/storagePools/{storagePool}
    /// * /projects/{project}/zones/{zone}/storagePools/{storagePool}
    /// * /zones/{zone}/storagePools/{storagePool}
    /// * /{storagePool}
    #[builder(into)]
    #[serde(rename = "storagePool")]
    pub r#storage_pool: Option<String>,
    /// The GCE disk type. Such as pd-standard, pd-balanced or pd-ssd.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
