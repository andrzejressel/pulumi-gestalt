#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionEksPropertyPodPropertyVolume {
    /// Specifies the configuration of a Kubernetes emptyDir volume.
    #[builder(into)]
    #[serde(rename = "emptyDirs")]
    pub r#empty_dirs: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyVolumeEmptyDir>,
    /// The path for the device on the host container instance.
    #[builder(into)]
    #[serde(rename = "hostPaths")]
    pub r#host_paths: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyVolumeHostPath>,
    /// The name of the job definition to register. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies the configuration of a Kubernetes secret volume.
    #[builder(into)]
    #[serde(rename = "secrets")]
    pub r#secrets: Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyVolumeSecret>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobDefinitionEksPropertyPodPropertyVolume {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("empty_dirs".to_string(), self.r#empty_dirs.to_pulumi_value().await);
            map.insert("host_paths".to_string(), self.r#host_paths.to_pulumi_value().await);
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);
            map.insert("secrets".to_string(), self.r#secrets.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobDefinitionEksPropertyPodPropertyVolume {
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
                    r#empty_dirs: {
                        let field_value = match fields_map.get("empty_dirs") {
                            Some(value) => value,
                            None => bail!("Missing field 'empty_dirs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyVolumeEmptyDir> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#host_paths: {
                        let field_value = match fields_map.get("host_paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyVolumeHostPath> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#secrets: {
                        let field_value = match fields_map.get("secrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'secrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyVolumeSecret> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
