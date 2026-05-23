#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceSourceConfiguration {
    /// Describes resources needed to authenticate access to some source repositories. See Authentication Configuration below for more details.
    #[builder(into)]
    pub r#authentication_configuration: Option<Box<super::super::types::apprunner::ServiceSourceConfigurationAuthenticationConfiguration>>,
    /// Whether continuous integration from the source repository is enabled for the App Runner service. If set to `true`, each repository change (source code commit or new image version) starts a deployment. Defaults to `true`.
    #[builder(into)]
    pub r#auto_deployments_enabled: Option<bool>,
    /// Description of a source code repository. See Code Repository below for more details.
    #[builder(into)]
    pub r#code_repository: Option<Box<super::super::types::apprunner::ServiceSourceConfigurationCodeRepository>>,
    /// Description of a source image repository. See Image Repository below for more details.
    #[builder(into)]
    pub r#image_repository: Option<Box<super::super::types::apprunner::ServiceSourceConfigurationImageRepository>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceSourceConfiguration {
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
                    "authenticationConfiguration",
                    &self.r#authentication_configuration,
                ),
                to_pulumi_object_field(
                    "autoDeploymentsEnabled",
                    &self.r#auto_deployments_enabled,
                ),
                to_pulumi_object_field(
                    "codeRepository",
                    &self.r#code_repository,
                ),
                to_pulumi_object_field(
                    "imageRepository",
                    &self.r#image_repository,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("authenticationConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'authenticationConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_deployments_enabled: {
                        let field_value = match fields_map.get("autoDeploymentsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoDeploymentsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#code_repository: {
                        let field_value = match fields_map.get("codeRepository") {
                            Some(value) => value,
                            None => bail!("Missing field 'codeRepository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_repository: {
                        let field_value = match fields_map.get("imageRepository") {
                            Some(value) => value,
                            None => bail!("Missing field 'imageRepository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
