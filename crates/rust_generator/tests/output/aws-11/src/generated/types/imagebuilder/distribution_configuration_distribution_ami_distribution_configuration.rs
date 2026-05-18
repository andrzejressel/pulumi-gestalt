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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "ami_tags",
                    &self.r#ami_tags,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "kms_key_id",
                    &self.r#kms_key_id,
                ),
                to_pulumi_object_field(
                    "launch_permission",
                    &self.r#launch_permission,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "target_account_ids",
                    &self.r#target_account_ids,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionConfigurationDistributionAmiDistributionConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#ami_tags: {
                        let field_value = match fields_map.get("ami_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'ami_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_id: {
                        let field_value = match fields_map.get("kms_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_permission: {
                        let field_value = match fields_map.get("launch_permission") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_permission' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_account_ids: {
                        let field_value = match fields_map.get("target_account_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_account_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
