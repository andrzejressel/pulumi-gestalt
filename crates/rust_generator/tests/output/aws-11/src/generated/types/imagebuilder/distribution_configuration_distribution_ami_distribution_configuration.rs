#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DistributionConfigurationDistributionAmiDistributionConfiguration {
    /// Key-value map of tags to apply to the distributed AMI.
    #[builder(into)]
    #[serde(rename = "amiTags")]
    pub r#ami_tags: Option<std::collections::HashMap<String, String>>,
    /// Description to apply to the distributed AMI.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Amazon Resource Name (ARN) of the Key Management Service (KMS) Key to encrypt the distributed AMI.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// Configuration block of EC2 launch permissions to apply to the distributed AMI. Detailed below.
    #[builder(into)]
    #[serde(rename = "launchPermission")]
    pub r#launch_permission: Box<Option<super::super::types::imagebuilder::DistributionConfigurationDistributionAmiDistributionConfigurationLaunchPermission>>,
    /// Name to apply to the distributed AMI.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Set of AWS Account identifiers to distribute the AMI.
    #[builder(into)]
    #[serde(rename = "targetAccountIds")]
    pub r#target_account_ids: Option<Vec<String>>,
}
