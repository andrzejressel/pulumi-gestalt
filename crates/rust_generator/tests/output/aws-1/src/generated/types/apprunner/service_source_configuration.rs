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
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "authentication_configuration",
                    &self.r#authentication_configuration,
                ),
                to_pulumi_object_field(
                    "auto_deployments_enabled",
                    &self.r#auto_deployments_enabled,
                ),
                to_pulumi_object_field(
                    "code_repository",
                    &self.r#code_repository,
                ),
                to_pulumi_object_field(
                    "image_repository",
                    &self.r#image_repository,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceSourceConfiguration {
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
                    r#authentication_configuration: {
                        let field_value = match fields_map.get("authentication_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_deployments_enabled: {
                        let field_value = match fields_map.get("auto_deployments_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_deployments_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#code_repository: {
                        let field_value = match fields_map.get("code_repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'code_repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_repository: {
                        let field_value = match fields_map.get("image_repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
