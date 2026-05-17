#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeEfsVolumeConfiguration {
    /// The authorization configuration details for the Amazon EFS file system.
    #[builder(into)]
    #[serde(rename = "authorizationConfigs")]
    pub r#authorization_configs: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeEfsVolumeConfigurationAuthorizationConfig>,
    /// The Amazon EFS file system ID to use.
    #[builder(into)]
    #[serde(rename = "fileSystemId")]
    pub r#file_system_id: String,
    /// The directory within the Amazon EFS file system to mount as the root directory inside the host.
    #[builder(into)]
    #[serde(rename = "rootDirectory")]
    pub r#root_directory: String,
    /// Determines whether to enable encryption for Amazon EFS data in transit between the Amazon ECS host and the Amazon EFS server
    #[builder(into)]
    #[serde(rename = "transitEncryption")]
    pub r#transit_encryption: String,
    /// The port to use when sending encrypted data between the Amazon ECS host and the Amazon EFS server.
    #[builder(into)]
    #[serde(rename = "transitEncryptionPort")]
    pub r#transit_encryption_port: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeEfsVolumeConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "authorization_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authorization_configs,
                )
                .await,
            );
            map.insert(
                "file_system_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_system_id,
                )
                .await,
            );
            map.insert(
                "root_directory".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#root_directory,
                )
                .await,
            );
            map.insert(
                "transit_encryption".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transit_encryption,
                )
                .await,
            );
            map.insert(
                "transit_encryption_port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transit_encryption_port,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeEfsVolumeConfiguration {
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
                    r#authorization_configs: {
                        let field_value = match fields_map.get("authorization_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorization_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_system_id: {
                        let field_value = match fields_map.get("file_system_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_system_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_directory: {
                        let field_value = match fields_map.get("root_directory") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_directory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transit_encryption: {
                        let field_value = match fields_map.get("transit_encryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'transit_encryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transit_encryption_port: {
                        let field_value = match fields_map.get("transit_encryption_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'transit_encryption_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
