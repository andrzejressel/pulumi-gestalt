#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerVolume {
    /// This parameter is specified when you're using an Amazon Elastic File System file system for job storage.
    #[builder(into)]
    #[serde(rename = "efsVolumeConfigurations")]
    pub r#efs_volume_configurations: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeEfsVolumeConfiguration>,
    /// The contents of the host parameter determine whether your data volume persists on the host container instance and where it's stored.
    #[builder(into)]
    #[serde(rename = "hosts")]
    pub r#hosts: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeHost>,
    /// The name of the job definition to register. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobDefinitionNodePropertyNodeRangePropertyContainerVolume {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("efs_volume_configurations".to_string(), self.r#efs_volume_configurations.to_pulumi_value().await);
            map.insert("hosts".to_string(), self.r#hosts.to_pulumi_value().await);
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobDefinitionNodePropertyNodeRangePropertyContainerVolume {
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
                    r#efs_volume_configurations: {
                        let field_value = match fields_map.get("efs_volume_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'efs_volume_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeEfsVolumeConfiguration> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hosts: {
                        let field_value = match fields_map.get("hosts") {
                            Some(value) => value,
                            None => bail!("Missing field 'hosts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeHost> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
