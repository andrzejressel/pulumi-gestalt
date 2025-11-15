#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainEbsOptions {
    /// Whether EBS volumes are attached to data nodes in the domain.
    #[builder(into)]
    #[serde(rename = "ebsEnabled")]
    pub r#ebs_enabled: bool,
    /// Baseline input/output (I/O) performance of EBS volumes attached to data nodes. Applicable only for the GP3 and Provisioned IOPS EBS volume types.
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Option<i32>,
    /// Specifies the throughput (in MiB/s) of the EBS volumes attached to data nodes. Applicable only for the gp3 volume type.
    #[builder(into)]
    #[serde(rename = "throughput")]
    pub r#throughput: Option<i32>,
    /// Size of EBS volumes attached to data nodes (in GiB).
    #[builder(into)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: Option<i32>,
    /// Type of EBS volumes attached to data nodes.
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Option<String>,
}
