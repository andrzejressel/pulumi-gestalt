#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ModelContainerModelDataSourceS3DataSource {
    /// How the model data is prepared. Allowed values are: `None` and `Gzip`.
    #[builder(into)]
    #[serde(rename = "compressionType")]
    pub r#compression_type: String,
    /// Specifies the access configuration file for the ML model. You can explicitly accept the model end-user license agreement (EULA) within the [`model_access_config` configuration block]. see Model Access Config.
    #[builder(into)]
    #[serde(rename = "modelAccessConfig")]
    pub r#model_access_config: Option<Box<super::super::types::sagemaker::ModelContainerModelDataSourceS3DataSourceModelAccessConfig>>,
    /// The type of model data to deploy. Allowed values are: `S3Object` and `S3Prefix`.
    #[builder(into)]
    #[serde(rename = "s3DataType")]
    pub r#s_3_data_type: String,
    /// The S3 path of model data to deploy.
    #[builder(into)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ModelContainerModelDataSourceS3DataSource {
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
                "compression_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#compression_type,
                )
                .await,
            );
            map.insert(
                "model_access_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#model_access_config,
                )
                .await,
            );
            map.insert(
                "s_3_data_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_data_type,
                )
                .await,
            );
            map.insert(
                "s_3_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_uri,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ModelContainerModelDataSourceS3DataSource {
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
                    r#compression_type: {
                        let field_value = match fields_map.get("compression_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'compression_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_access_config: {
                        let field_value = match fields_map.get("model_access_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'model_access_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_data_type: {
                        let field_value = match fields_map.get("s_3_data_type") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_data_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_uri: {
                        let field_value = match fields_map.get("s_3_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
