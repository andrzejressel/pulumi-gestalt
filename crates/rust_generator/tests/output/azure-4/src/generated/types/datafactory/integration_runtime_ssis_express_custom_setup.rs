#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IntegrationRuntimeSsisExpressCustomSetup {
    /// One or more `command_key` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "commandKeys")]
    pub r#command_keys: Option<Vec<super::super::types::datafactory::IntegrationRuntimeSsisExpressCustomSetupCommandKey>>,
    /// One or more `component` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Option<Vec<super::super::types::datafactory::IntegrationRuntimeSsisExpressCustomSetupComponent>>,
    /// The Environment Variables for the Azure-SSIS Integration Runtime.
    #[builder(into)]
    #[serde(rename = "environment")]
    pub r#environment: Option<std::collections::HashMap<String, String>>,
    /// The version of Azure Powershell installed for the Azure-SSIS Integration Runtime.
    /// 
    /// > **NOTE** At least one of `env`, `powershell_version`, `component` and `command_key` should be specified.
    #[builder(into)]
    #[serde(rename = "powershellVersion")]
    pub r#powershell_version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IntegrationRuntimeSsisExpressCustomSetup {
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
                "command_keys".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#command_keys,
                )
                .await,
            );
            map.insert(
                "components".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#components,
                )
                .await,
            );
            map.insert(
                "environment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#environment,
                )
                .await,
            );
            map.insert(
                "powershell_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#powershell_version,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IntegrationRuntimeSsisExpressCustomSetup {
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
                    r#command_keys: {
                        let field_value = match fields_map.get("command_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'command_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#components: {
                        let field_value = match fields_map.get("components") {
                            Some(value) => value,
                            None => bail!("Missing field 'components' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#environment: {
                        let field_value = match fields_map.get("environment") {
                            Some(value) => value,
                            None => bail!("Missing field 'environment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#powershell_version: {
                        let field_value = match fields_map.get("powershell_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'powershell_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
