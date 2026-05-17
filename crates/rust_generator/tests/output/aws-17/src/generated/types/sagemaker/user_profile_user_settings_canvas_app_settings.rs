#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UserProfileUserSettingsCanvasAppSettings {
    /// The model deployment settings for the SageMaker Canvas application. See Direct Deploy Settings below.
    #[builder(into)]
    #[serde(rename = "directDeploySettings")]
    pub r#direct_deploy_settings: Option<Box<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsDirectDeploySettings>>,
    /// The settings for running Amazon EMR Serverless jobs in SageMaker Canvas. See `emr_serverless_settings` Block below.
    #[builder(into)]
    #[serde(rename = "emrServerlessSettings")]
    pub r#emr_serverless_settings: Option<Box<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsEmrServerlessSettings>>,
    #[builder(into)]
    #[serde(rename = "generativeAiSettings")]
    pub r#generative_ai_settings: Option<Box<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsGenerativeAiSettings>>,
    /// The settings for connecting to an external data source with OAuth. See Identity Provider OAuth Settings below.
    #[builder(into)]
    #[serde(rename = "identityProviderOauthSettings")]
    pub r#identity_provider_oauth_settings: Option<Vec<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsIdentityProviderOauthSetting>>,
    /// The settings for document querying. See Kendra Settings below.
    #[builder(into)]
    #[serde(rename = "kendraSettings")]
    pub r#kendra_settings: Option<Box<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsKendraSettings>>,
    /// The model registry settings for the SageMaker Canvas application. See Model Register Settings below.
    #[builder(into)]
    #[serde(rename = "modelRegisterSettings")]
    pub r#model_register_settings: Option<Box<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsModelRegisterSettings>>,
    /// Time series forecast settings for the Canvas app. See Time Series Forecasting Settings below.
    #[builder(into)]
    #[serde(rename = "timeSeriesForecastingSettings")]
    pub r#time_series_forecasting_settings: Option<Box<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsTimeSeriesForecastingSettings>>,
    /// The workspace settings for the SageMaker Canvas application. See Workspace Settings below.
    #[builder(into)]
    #[serde(rename = "workspaceSettings")]
    pub r#workspace_settings: Option<Box<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettingsWorkspaceSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UserProfileUserSettingsCanvasAppSettings {
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
                    "direct_deploy_settings",
                    &self.r#direct_deploy_settings,
                ),
                to_pulumi_object_field(
                    "emr_serverless_settings",
                    &self.r#emr_serverless_settings,
                ),
                to_pulumi_object_field(
                    "generative_ai_settings",
                    &self.r#generative_ai_settings,
                ),
                to_pulumi_object_field(
                    "identity_provider_oauth_settings",
                    &self.r#identity_provider_oauth_settings,
                ),
                to_pulumi_object_field(
                    "kendra_settings",
                    &self.r#kendra_settings,
                ),
                to_pulumi_object_field(
                    "model_register_settings",
                    &self.r#model_register_settings,
                ),
                to_pulumi_object_field(
                    "time_series_forecasting_settings",
                    &self.r#time_series_forecasting_settings,
                ),
                to_pulumi_object_field(
                    "workspace_settings",
                    &self.r#workspace_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("direct_deploy_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'direct_deploy_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#emr_serverless_settings: {
                        let field_value = match fields_map.get("emr_serverless_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'emr_serverless_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#generative_ai_settings: {
                        let field_value = match fields_map.get("generative_ai_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'generative_ai_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identity_provider_oauth_settings: {
                        let field_value = match fields_map.get("identity_provider_oauth_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'identity_provider_oauth_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kendra_settings: {
                        let field_value = match fields_map.get("kendra_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'kendra_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_register_settings: {
                        let field_value = match fields_map.get("model_register_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'model_register_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_series_forecasting_settings: {
                        let field_value = match fields_map.get("time_series_forecasting_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_series_forecasting_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workspace_settings: {
                        let field_value = match fields_map.get("workspace_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'workspace_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
