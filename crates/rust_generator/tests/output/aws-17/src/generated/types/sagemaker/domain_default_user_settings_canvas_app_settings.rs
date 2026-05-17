#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDefaultUserSettingsCanvasAppSettings {
    /// The model deployment settings for the SageMaker Canvas application. See `direct_deploy_settings` Block below.
    #[builder(into)]
    #[serde(rename = "directDeploySettings")]
    pub r#direct_deploy_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsDirectDeploySettings>>,
    /// The settings for running Amazon EMR Serverless jobs in SageMaker Canvas. See `emr_serverless_settings` Block below.
    #[builder(into)]
    #[serde(rename = "emrServerlessSettings")]
    pub r#emr_serverless_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsEmrServerlessSettings>>,
    #[builder(into)]
    #[serde(rename = "generativeAiSettings")]
    pub r#generative_ai_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsGenerativeAiSettings>>,
    /// The settings for connecting to an external data source with OAuth. See `identity_provider_oauth_settings` Block below.
    #[builder(into)]
    #[serde(rename = "identityProviderOauthSettings")]
    pub r#identity_provider_oauth_settings: Option<Vec<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsIdentityProviderOauthSetting>>,
    /// The settings for document querying. See `kendra_settings` Block below.
    #[builder(into)]
    #[serde(rename = "kendraSettings")]
    pub r#kendra_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsKendraSettings>>,
    /// The model registry settings for the SageMaker Canvas application. See `model_register_settings` Block below.
    #[builder(into)]
    #[serde(rename = "modelRegisterSettings")]
    pub r#model_register_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsModelRegisterSettings>>,
    /// Time series forecast settings for the Canvas app. See `time_series_forecasting_settings` Block below.
    #[builder(into)]
    #[serde(rename = "timeSeriesForecastingSettings")]
    pub r#time_series_forecasting_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsTimeSeriesForecastingSettings>>,
    /// The workspace settings for the SageMaker Canvas application. See `workspace_settings` Block below.
    #[builder(into)]
    #[serde(rename = "workspaceSettings")]
    pub r#workspace_settings: Option<Box<super::super::types::sagemaker::DomainDefaultUserSettingsCanvasAppSettingsWorkspaceSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainDefaultUserSettingsCanvasAppSettings {
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
                "direct_deploy_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#direct_deploy_settings,
                )
                .await,
            );
            map.insert(
                "emr_serverless_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#emr_serverless_settings,
                )
                .await,
            );
            map.insert(
                "generative_ai_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#generative_ai_settings,
                )
                .await,
            );
            map.insert(
                "identity_provider_oauth_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#identity_provider_oauth_settings,
                )
                .await,
            );
            map.insert(
                "kendra_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kendra_settings,
                )
                .await,
            );
            map.insert(
                "model_register_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#model_register_settings,
                )
                .await,
            );
            map.insert(
                "time_series_forecasting_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_series_forecasting_settings,
                )
                .await,
            );
            map.insert(
                "workspace_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#workspace_settings,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainDefaultUserSettingsCanvasAppSettings {
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
