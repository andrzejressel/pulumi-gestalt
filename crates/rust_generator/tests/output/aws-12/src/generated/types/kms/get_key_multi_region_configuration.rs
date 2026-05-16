#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetKeyMultiRegionConfiguration {
    /// Indicates whether the KMS key is a `PRIMARY` or `REPLICA` key.
    #[builder(into)]
    #[serde(rename = "multiRegionKeyType")]
    pub r#multi_region_key_type: String,
    /// The key ARN and Region of the primary key. This is the current KMS key if it is the primary key.
    #[builder(into)]
    #[serde(rename = "primaryKeys")]
    pub r#primary_keys: Vec<super::super::types::kms::GetKeyMultiRegionConfigurationPrimaryKey>,
    /// The key ARNs and Regions of all replica keys. Includes the current KMS key if it is a replica key.
    #[builder(into)]
    #[serde(rename = "replicaKeys")]
    pub r#replica_keys: Vec<super::super::types::kms::GetKeyMultiRegionConfigurationReplicaKey>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetKeyMultiRegionConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("multi_region_key_type".to_string(), self.r#multi_region_key_type.to_pulumi_value().await);
            map.insert("primary_keys".to_string(), self.r#primary_keys.to_pulumi_value().await);
            map.insert("replica_keys".to_string(), self.r#replica_keys.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetKeyMultiRegionConfiguration {
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
                    r#multi_region_key_type: {
                        let field_value = match fields_map.get("multi_region_key_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'multi_region_key_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#primary_keys: {
                        let field_value = match fields_map.get("primary_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::kms::GetKeyMultiRegionConfigurationPrimaryKey> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#replica_keys: {
                        let field_value = match fields_map.get("replica_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'replica_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::kms::GetKeyMultiRegionConfigurationReplicaKey> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
