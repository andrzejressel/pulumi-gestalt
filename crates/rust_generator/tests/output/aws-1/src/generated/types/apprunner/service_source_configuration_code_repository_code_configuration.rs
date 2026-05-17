#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceSourceConfigurationCodeRepositoryCodeConfiguration {
    /// Basic configuration for building and running the App Runner service. Use this parameter to quickly launch an App Runner service without providing an apprunner.yaml file in the source code repository (or ignoring the file if it exists). See Code Configuration Values below for more details.
    #[builder(into)]
    #[serde(rename = "codeConfigurationValues")]
    pub r#code_configuration_values: Option<Box<super::super::types::apprunner::ServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValues>>,
    /// Source of the App Runner configuration. Valid values: `REPOSITORY`, `API`. Values are interpreted as follows:
    /// * `REPOSITORY` - App Runner reads configuration values from the apprunner.yaml file in the
    /// source code repository and ignores the CodeConfigurationValues parameter.
    /// * `API` - App Runner uses configuration values provided in the CodeConfigurationValues
    /// parameter and ignores the apprunner.yaml file in the source code repository.
    #[builder(into)]
    #[serde(rename = "configurationSource")]
    pub r#configuration_source: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceSourceConfigurationCodeRepositoryCodeConfiguration {
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
                "code_configuration_values".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#code_configuration_values,
                )
                .await,
            );
            map.insert(
                "configuration_source".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#configuration_source,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceSourceConfigurationCodeRepositoryCodeConfiguration {
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
                    r#code_configuration_values: {
                        let field_value = match fields_map.get("code_configuration_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'code_configuration_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#configuration_source: {
                        let field_value = match fields_map.get("configuration_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'configuration_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
