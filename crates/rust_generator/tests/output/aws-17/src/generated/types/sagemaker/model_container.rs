#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ModelContainer {
    /// The DNS host name for the container.
    #[builder(into)]
    #[serde(rename = "containerHostname")]
    pub r#container_hostname: Option<String>,
    /// Environment variables for the Docker container.
    /// A list of key value pairs.
    #[builder(into)]
    #[serde(rename = "environment")]
    pub r#environment: Option<std::collections::HashMap<String, String>>,
    /// The registry path where the inference code image is stored in Amazon ECR.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Option<String>,
    /// Specifies whether the model container is in Amazon ECR or a private Docker registry accessible from your Amazon Virtual Private Cloud (VPC). For more information see [Using a Private Docker Registry for Real-Time Inference Containers](https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms-containers-inference-private.html). see Image Config.
    #[builder(into)]
    #[serde(rename = "imageConfig")]
    pub r#image_config: Option<Box<super::super::types::sagemaker::ModelContainerImageConfig>>,
    /// The inference specification name in the model package version.
    #[builder(into)]
    #[serde(rename = "inferenceSpecificationName")]
    pub r#inference_specification_name: Option<String>,
    /// The container hosts value `SingleModel/MultiModel`. The default value is `SingleModel`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// The location of model data to deploy. Use this for uncompressed model deployment. For information about how to deploy an uncompressed model, see [Deploying uncompressed models](https://docs.aws.amazon.com/sagemaker/latest/dg/large-model-inference-uncompressed.html) in the _AWS SageMaker Developer Guide_.
    #[builder(into)]
    #[serde(rename = "modelDataSource")]
    pub r#model_data_source: Option<Box<super::super::types::sagemaker::ModelContainerModelDataSource>>,
    /// The URL for the S3 location where model artifacts are stored.
    #[builder(into)]
    #[serde(rename = "modelDataUrl")]
    pub r#model_data_url: Option<String>,
    /// The Amazon Resource Name (ARN) of the model package to use to create the model.
    #[builder(into)]
    #[serde(rename = "modelPackageName")]
    pub r#model_package_name: Option<String>,
    /// Specifies additional configuration for multi-model endpoints. see Multi Model Config.
    #[builder(into)]
    #[serde(rename = "multiModelConfig")]
    pub r#multi_model_config: Option<Box<super::super::types::sagemaker::ModelContainerMultiModelConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ModelContainer {
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
                "container_hostname".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_hostname,
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
                "image".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image,
                )
                .await,
            );
            map.insert(
                "image_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_config,
                )
                .await,
            );
            map.insert(
                "inference_specification_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#inference_specification_name,
                )
                .await,
            );
            map.insert(
                "mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mode,
                )
                .await,
            );
            map.insert(
                "model_data_source".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#model_data_source,
                )
                .await,
            );
            map.insert(
                "model_data_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#model_data_url,
                )
                .await,
            );
            map.insert(
                "model_package_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#model_package_name,
                )
                .await,
            );
            map.insert(
                "multi_model_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#multi_model_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ModelContainer {
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
                    r#container_hostname: {
                        let field_value = match fields_map.get("container_hostname") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_hostname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#image: {
                        let field_value = match fields_map.get("image") {
                            Some(value) => value,
                            None => bail!("Missing field 'image' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_config: {
                        let field_value = match fields_map.get("image_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inference_specification_name: {
                        let field_value = match fields_map.get("inference_specification_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'inference_specification_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_data_source: {
                        let field_value = match fields_map.get("model_data_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'model_data_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_data_url: {
                        let field_value = match fields_map.get("model_data_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'model_data_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_package_name: {
                        let field_value = match fields_map.get("model_package_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'model_package_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multi_model_config: {
                        let field_value = match fields_map.get("multi_model_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'multi_model_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
