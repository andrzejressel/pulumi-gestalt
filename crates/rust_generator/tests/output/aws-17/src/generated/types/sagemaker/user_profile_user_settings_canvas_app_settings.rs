#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserProfileUserSettingsCanvasAppSettings {
    /// The model deployment settings for the SageMaker Canvas application. See Direct Deploy Settings below.
    #[builder(into)]
    pub r#direct_deploy_settings: Option<Box<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsDirectDeploySettings>>,
    /// The settings for running Amazon EMR Serverless jobs in SageMaker Canvas. See `emr_serverless_settings` Block below.
    #[builder(into)]
    pub r#emr_serverless_settings: Option<Box<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsEmrServerlessSettings>>,
    #[builder(into)]
    pub r#generative_ai_settings: Option<Box<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsGenerativeAiSettings>>,
    /// The settings for connecting to an external data source with OAuth. See Identity Provider OAuth Settings below.
    #[builder(into)]
    pub r#identity_provider_oauth_settings: Option<Vec<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsIdentityProviderOauthSetting>>,
    /// The settings for document querying. See Kendra Settings below.
    #[builder(into)]
    pub r#kendra_settings: Option<Box<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsKendraSettings>>,
    /// The model registry settings for the SageMaker Canvas application. See Model Register Settings below.
    #[builder(into)]
    pub r#model_register_settings: Option<Box<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsModelRegisterSettings>>,
    /// Time series forecast settings for the Canvas app. See Time Series Forecasting Settings below.
    #[builder(into)]
    pub r#time_series_forecasting_settings: Option<Box<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsTimeSeriesForecastingSettings>>,
    /// The workspace settings for the SageMaker Canvas application. See Workspace Settings below.
    #[builder(into)]
    pub r#workspace_settings: Option<Box<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsWorkspaceSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserProfileUserSettingsCanvasAppSettings {
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
                    "directDeploySettings",
                    &self.r#direct_deploy_settings,
                ),
                to_pulumi_object_field(
                    "emrServerlessSettings",
                    &self.r#emr_serverless_settings,
                ),
                to_pulumi_object_field(
                    "generativeAiSettings",
                    &self.r#generative_ai_settings,
                ),
                to_pulumi_object_field(
                    "identityProviderOauthSettings",
                    &self.r#identity_provider_oauth_settings,
                ),
                to_pulumi_object_field(
                    "kendraSettings",
                    &self.r#kendra_settings,
                ),
                to_pulumi_object_field(
                    "modelRegisterSettings",
                    &self.r#model_register_settings,
                ),
                to_pulumi_object_field(
                    "timeSeriesForecastingSettings",
                    &self.r#time_series_forecasting_settings,
                ),
                to_pulumi_object_field(
                    "workspaceSettings",
                    &self.r#workspace_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UserProfileUserSettingsCanvasAppSettings {
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
                    r#direct_deploy_settings: {
                        let field_value = match fields_map.get("directDeploySettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'directDeploySettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#emr_serverless_settings: {
                        let field_value = match fields_map.get("emrServerlessSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'emrServerlessSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#generative_ai_settings: {
                        let field_value = match fields_map.get("generativeAiSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'generativeAiSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identity_provider_oauth_settings: {
                        let field_value = match fields_map.get("identityProviderOauthSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'identityProviderOauthSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kendra_settings: {
                        let field_value = match fields_map.get("kendraSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'kendraSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_register_settings: {
                        let field_value = match fields_map.get("modelRegisterSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'modelRegisterSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_series_forecasting_settings: {
                        let field_value = match fields_map.get("timeSeriesForecastingSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeSeriesForecastingSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workspace_settings: {
                        let field_value = match fields_map.get("workspaceSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'workspaceSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
