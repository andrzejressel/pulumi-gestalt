#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TaskDefinitionVolumeFsxWindowsFileServerVolumeConfiguration {
    /// Configuration block for authorization for the Amazon FSx for Windows File Server file system detailed below.
    #[builder(into)]
    #[serde(rename = "authorizationConfig")]
    pub r#authorization_config: Box<super::super::types::ecs::TaskDefinitionVolumeFsxWindowsFileServerVolumeConfigurationAuthorizationConfig>,
    /// The Amazon FSx for Windows File Server file system ID to use.
    #[builder(into)]
    #[serde(rename = "fileSystemId")]
    pub r#file_system_id: String,
    /// The directory within the Amazon FSx for Windows File Server file system to mount as the root directory inside the host.
    #[builder(into)]
    #[serde(rename = "rootDirectory")]
    pub r#root_directory: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TaskDefinitionVolumeFsxWindowsFileServerVolumeConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "authorization_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authorization_config,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TaskDefinitionVolumeFsxWindowsFileServerVolumeConfiguration {
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
                    r#authorization_config: {
                        let field_value = match fields_map.get("authorization_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorization_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
