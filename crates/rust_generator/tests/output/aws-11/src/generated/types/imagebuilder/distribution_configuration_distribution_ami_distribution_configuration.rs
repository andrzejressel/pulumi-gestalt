#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
    pub r#launch_permission: Option<Box<super::super::types::imagebuilder::DistributionConfigurationDistributionAmiDistributionConfigurationLaunchPermission>>,
    /// Name to apply to the distributed AMI.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Set of AWS Account identifiers to distribute the AMI.
    #[builder(into)]
    #[serde(rename = "targetAccountIds")]
    pub r#target_account_ids: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionConfigurationDistributionAmiDistributionConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("ami_tags".to_string(), self.r#ami_tags.to_pulumi_value().await);
            map.insert("description".to_string(), self.r#description.to_pulumi_value().await);
            map.insert("kms_key_id".to_string(), self.r#kms_key_id.to_pulumi_value().await);
            map.insert("launch_permission".to_string(), self.r#launch_permission.to_pulumi_value().await);
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);
            map.insert("target_account_ids".to_string(), self.r#target_account_ids.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionConfigurationDistributionAmiDistributionConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#ami_tags: {
                        let field_value = match fields_map.get("ami_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'ami_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#kms_key_id: {
                        let field_value = match fields_map.get("kms_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#launch_permission: {
                        let field_value = match fields_map.get("launch_permission") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_permission' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::imagebuilder::DistributionConfigurationDistributionAmiDistributionConfigurationLaunchPermission>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#target_account_ids: {
                        let field_value = match fields_map.get("target_account_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_account_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
