#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataLakeConfiguration {
    /// Provides encryption details of Amazon Security Lake object.
    #[builder(into)]
    #[serde(rename = "encryptionConfigurations")]
    pub r#encryption_configurations: Option<Vec<super::super::types::securitylake::DataLakeConfigurationEncryptionConfiguration>>,
    /// Provides lifecycle details of Amazon Security Lake object.
    #[builder(into)]
    #[serde(rename = "lifecycleConfiguration")]
    pub r#lifecycle_configuration: Option<Box<super::super::types::securitylake::DataLakeConfigurationLifecycleConfiguration>>,
    /// The AWS Regions where Security Lake is automatically enabled.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
    /// Provides replication details of Amazon Security Lake object.
    #[builder(into)]
    #[serde(rename = "replicationConfiguration")]
    pub r#replication_configuration: Option<Box<super::super::types::securitylake::DataLakeConfigurationReplicationConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataLakeConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("encryption_configurations".to_string(), self.r#encryption_configurations.to_pulumi_value().await);
            map.insert("lifecycle_configuration".to_string(), self.r#lifecycle_configuration.to_pulumi_value().await);
            map.insert("region".to_string(), self.r#region.to_pulumi_value().await);
            map.insert("replication_configuration".to_string(), self.r#replication_configuration.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataLakeConfiguration {
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
                    r#encryption_configurations: {
                        let field_value = match fields_map.get("encryption_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::securitylake::DataLakeConfigurationEncryptionConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#lifecycle_configuration: {
                        let field_value = match fields_map.get("lifecycle_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycle_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::securitylake::DataLakeConfigurationLifecycleConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#region: {
                        let field_value = match fields_map.get("region") {
                            Some(value) => value,
                            None => bail!("Missing field 'region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#replication_configuration: {
                        let field_value = match fields_map.get("replication_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'replication_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::securitylake::DataLakeConfigurationReplicationConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
