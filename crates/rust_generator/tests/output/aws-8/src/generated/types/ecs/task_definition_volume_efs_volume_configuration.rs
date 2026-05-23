#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TaskDefinitionVolumeEfsVolumeConfiguration {
    /// Configuration block for authorization for the Amazon EFS file system. Detailed below.
    #[builder(into)]
    pub r#authorization_config: Option<Box<super::super::types::ecs::TaskDefinitionVolumeEfsVolumeConfigurationAuthorizationConfig>>,
    /// ID of the EFS File System.
    #[builder(into)]
    pub r#file_system_id: String,
    /// Directory within the Amazon EFS file system to mount as the root directory inside the host. If this parameter is omitted, the root of the Amazon EFS volume will be used. Specifying / will have the same effect as omitting this parameter. This argument is ignored when using `authorization_config`.
    #[builder(into)]
    pub r#root_directory: Option<String>,
    /// Whether or not to enable encryption for Amazon EFS data in transit between the Amazon ECS host and the Amazon EFS server. Transit encryption must be enabled if Amazon EFS IAM authorization is used. Valid values: `ENABLED`, `DISABLED`. If this parameter is omitted, the default value of `DISABLED` is used.
    #[builder(into)]
    pub r#transit_encryption: Option<String>,
    /// Port to use for transit encryption. If you do not specify a transit encryption port, it will use the port selection strategy that the Amazon EFS mount helper uses.
    #[builder(into)]
    pub r#transit_encryption_port: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TaskDefinitionVolumeEfsVolumeConfiguration {
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
                    "authorizationConfig",
                    &self.r#authorization_config,
                ),
                to_pulumi_object_field(
                    "fileSystemId",
                    &self.r#file_system_id,
                ),
                to_pulumi_object_field(
                    "rootDirectory",
                    &self.r#root_directory,
                ),
                to_pulumi_object_field(
                    "transitEncryption",
                    &self.r#transit_encryption,
                ),
                to_pulumi_object_field(
                    "transitEncryptionPort",
                    &self.r#transit_encryption_port,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TaskDefinitionVolumeEfsVolumeConfiguration {
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
                        let field_value = match fields_map.get("authorizationConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorizationConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_system_id: {
                        let field_value = match fields_map.get("fileSystemId") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileSystemId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_directory: {
                        let field_value = match fields_map.get("rootDirectory") {
                            Some(value) => value,
                            None => bail!("Missing field 'rootDirectory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transit_encryption: {
                        let field_value = match fields_map.get("transitEncryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitEncryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transit_encryption_port: {
                        let field_value = match fields_map.get("transitEncryptionPort") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitEncryptionPort' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
