#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceSourceConfiguration {
    /// Describes resources needed to authenticate access to some source repositories. See Authentication Configuration below for more details.
    #[builder(into)]
    #[serde(rename = "authenticationConfiguration")]
    pub r#authentication_configuration: Option<Box<super::super::types::apprunner::ServiceSourceConfigurationAuthenticationConfiguration>>,
    /// Whether continuous integration from the source repository is enabled for the App Runner service. If set to `true`, each repository change (source code commit or new image version) starts a deployment. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "autoDeploymentsEnabled")]
    pub r#auto_deployments_enabled: Option<bool>,
    /// Description of a source code repository. See Code Repository below for more details.
    #[builder(into)]
    #[serde(rename = "codeRepository")]
    pub r#code_repository: Option<Box<super::super::types::apprunner::ServiceSourceConfigurationCodeRepository>>,
    /// Description of a source image repository. See Image Repository below for more details.
    #[builder(into)]
    #[serde(rename = "imageRepository")]
    pub r#image_repository: Option<Box<super::super::types::apprunner::ServiceSourceConfigurationImageRepository>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceSourceConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("authentication_configuration".to_string(), self.r#authentication_configuration.to_pulumi_value().await);
            map.insert("auto_deployments_enabled".to_string(), self.r#auto_deployments_enabled.to_pulumi_value().await);
            map.insert("code_repository".to_string(), self.r#code_repository.to_pulumi_value().await);
            map.insert("image_repository".to_string(), self.r#image_repository.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceSourceConfiguration {
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
                    r#authentication_configuration: {
                        let field_value = match fields_map.get("authentication_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::apprunner::ServiceSourceConfigurationAuthenticationConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#auto_deployments_enabled: {
                        let field_value = match fields_map.get("auto_deployments_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_deployments_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#code_repository: {
                        let field_value = match fields_map.get("code_repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'code_repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::apprunner::ServiceSourceConfigurationCodeRepository>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#image_repository: {
                        let field_value = match fields_map.get("image_repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::apprunner::ServiceSourceConfigurationImageRepository>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
