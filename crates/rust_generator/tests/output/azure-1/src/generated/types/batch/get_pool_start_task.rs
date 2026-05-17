#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPoolStartTask {
    /// The command line executed by the start task.
    #[builder(into)]
    #[serde(rename = "commandLine")]
    pub r#command_line: String,
    /// A map of strings (key,value) that represents the environment variables to set in the start task.
    #[builder(into)]
    #[serde(rename = "commonEnvironmentProperties")]
    pub r#common_environment_properties: Option<std::collections::HashMap<String, String>>,
    /// The settings for the container under which the start task runs.
    #[builder(into)]
    #[serde(rename = "containers")]
    pub r#containers: Vec<super::super::types::batch::GetPoolStartTaskContainer>,
    /// One or more `resource_file` blocks that describe the files to be downloaded to a compute node.
    #[builder(into)]
    #[serde(rename = "resourceFiles")]
    pub r#resource_files: Vec<super::super::types::batch::GetPoolStartTaskResourceFile>,
    /// The number of retry count
    #[builder(into)]
    #[serde(rename = "taskRetryMaximum")]
    pub r#task_retry_maximum: i32,
    /// A `user_identity` block that describes the user identity under which the start task runs.
    #[builder(into)]
    #[serde(rename = "userIdentities")]
    pub r#user_identities: Vec<super::super::types::batch::GetPoolStartTaskUserIdentity>,
    /// A flag that indicates if the Batch pool should wait for the start task to be completed.
    #[builder(into)]
    #[serde(rename = "waitForSuccess")]
    pub r#wait_for_success: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPoolStartTask {
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
                "command_line".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#command_line,
                )
                .await,
            );
            map.insert(
                "common_environment_properties".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#common_environment_properties,
                )
                .await,
            );
            map.insert(
                "containers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#containers,
                )
                .await,
            );
            map.insert(
                "resource_files".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_files,
                )
                .await,
            );
            map.insert(
                "task_retry_maximum".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#task_retry_maximum,
                )
                .await,
            );
            map.insert(
                "user_identities".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_identities,
                )
                .await,
            );
            map.insert(
                "wait_for_success".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#wait_for_success,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPoolStartTask {
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
                    r#command_line: {
                        let field_value = match fields_map.get("command_line") {
                            Some(value) => value,
                            None => bail!("Missing field 'command_line' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#common_environment_properties: {
                        let field_value = match fields_map.get("common_environment_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'common_environment_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#containers: {
                        let field_value = match fields_map.get("containers") {
                            Some(value) => value,
                            None => bail!("Missing field 'containers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_files: {
                        let field_value = match fields_map.get("resource_files") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_files' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#task_retry_maximum: {
                        let field_value = match fields_map.get("task_retry_maximum") {
                            Some(value) => value,
                            None => bail!("Missing field 'task_retry_maximum' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_identities: {
                        let field_value = match fields_map.get("user_identities") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_identities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#wait_for_success: {
                        let field_value = match fields_map.get("wait_for_success") {
                            Some(value) => value,
                            None => bail!("Missing field 'wait_for_success' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
