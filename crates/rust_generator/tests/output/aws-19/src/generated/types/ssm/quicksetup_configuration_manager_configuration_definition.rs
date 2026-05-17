#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct QuicksetupConfigurationManagerConfigurationDefinition {
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    #[builder(into)]
    #[serde(rename = "localDeploymentAdministrationRoleArn")]
    pub r#local_deployment_administration_role_arn: Option<String>,
    /// Name of the IAM role used to deploy local configurations.
    #[builder(into)]
    #[serde(rename = "localDeploymentExecutionRoleName")]
    pub r#local_deployment_execution_role_name: Option<String>,
    /// Parameters for the configuration definition type. Parameters for configuration definitions vary based the configuration type. See the [AWS API documentation](https://docs.aws.amazon.com/quick-setup/latest/APIReference/API_ConfigurationDefinitionInput.html) for a complete list of parameters for each configuration type.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: std::collections::HashMap<String, String>,
    /// Type of the Quick Setup configuration.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// Version of the Quick Setup type to use.
    #[builder(into)]
    #[serde(rename = "typeVersion")]
    pub r#type_version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for QuicksetupConfigurationManagerConfigurationDefinition {
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
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "local_deployment_administration_role_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_deployment_administration_role_arn,
                )
                .await,
            );
            map.insert(
                "local_deployment_execution_role_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_deployment_execution_role_name,
                )
                .await,
            );
            map.insert(
                "parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parameters,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
                )
                .await,
            );
            map.insert(
                "type_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_version,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for QuicksetupConfigurationManagerConfigurationDefinition {
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
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_deployment_administration_role_arn: {
                        let field_value = match fields_map.get("local_deployment_administration_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_deployment_administration_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_deployment_execution_role_name: {
                        let field_value = match fields_map.get("local_deployment_execution_role_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_deployment_execution_role_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parameters: {
                        let field_value = match fields_map.get("parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_version: {
                        let field_value = match fields_map.get("type_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
