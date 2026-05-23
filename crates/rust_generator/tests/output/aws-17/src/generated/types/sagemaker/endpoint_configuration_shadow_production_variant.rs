#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointConfigurationShadowProductionVariant {
    /// The size of the Elastic Inference (EI) instance to use for the production variant.
    #[builder(into)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: Option<String>,
    /// The timeout value, in seconds, for your inference container to pass health check by SageMaker Hosting. For more information about health check, see [How Your Container Should Respond to Health Check (Ping) Requests](https://docs.aws.amazon.com/sagemaker/latest/dg/your-algorithms-inference-code.html#your-algorithms-inference-algo-ping-requests). Valid values between `60` and `3600`.
    #[builder(into)]
    #[serde(rename = "containerStartupHealthCheckTimeoutInSeconds")]
    pub r#container_startup_health_check_timeout_in_seconds: Option<i32>,
    /// Specifies configuration for a core dump from the model container when the process crashes. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "coreDumpConfig")]
    pub r#core_dump_config: Option<Box<super::super::types::sagemaker::EndpointConfigurationShadowProductionVariantCoreDumpConfig>>,
    /// You can use this parameter to turn on native Amazon Web Services Systems Manager (SSM) access for a production variant behind an endpoint. By default, SSM access is disabled for all production variants behind an endpoints.
    #[builder(into)]
    #[serde(rename = "enableSsmAccess")]
    pub r#enable_ssm_access: Option<bool>,
    /// Specifies an option from a collection of preconfigured Amazon Machine Image (AMI) images. Each image is configured by Amazon Web Services with a set of software and driver versions. Amazon Web Services optimizes these configurations for different machine learning workloads.
    #[builder(into)]
    #[serde(rename = "inferenceAmiVersion")]
    pub r#inference_ami_version: Option<String>,
    /// Initial number of instances used for auto-scaling.
    #[builder(into)]
    #[serde(rename = "initialInstanceCount")]
    pub r#initial_instance_count: Option<i32>,
    /// Determines initial traffic distribution among all of the models that you specify in the endpoint configuration. If unspecified, it defaults to `1.0`.
    #[builder(into)]
    #[serde(rename = "initialVariantWeight")]
    pub r#initial_variant_weight: Option<f64>,
    /// The type of instance to start.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Option<String>,
    /// Settings that control the range in the number of instances that the endpoint provisions as it scales up or down to accommodate traffic.
    #[builder(into)]
    #[serde(rename = "managedInstanceScaling")]
    pub r#managed_instance_scaling: Option<Box<super::super::types::sagemaker::EndpointConfigurationShadowProductionVariantManagedInstanceScaling>>,
    /// The timeout value, in seconds, to download and extract the model that you want to host from Amazon S3 to the individual inference instance associated with this production variant. Valid values between `60` and `3600`.
    #[builder(into)]
    #[serde(rename = "modelDataDownloadTimeoutInSeconds")]
    pub r#model_data_download_timeout_in_seconds: Option<i32>,
    /// The name of the model to use.
    #[builder(into)]
    #[serde(rename = "modelName")]
    pub r#model_name: String,
    /// Sets how the endpoint routes incoming traffic. See routing_config below.
    #[builder(into)]
    #[serde(rename = "routingConfigs")]
    pub r#routing_configs: Option<Vec<super::super::types::sagemaker::EndpointConfigurationShadowProductionVariantRoutingConfig>>,
    /// Specifies configuration for how an endpoint performs asynchronous inference.
    #[builder(into)]
    #[serde(rename = "serverlessConfig")]
    pub r#serverless_config: Option<Box<super::super::types::sagemaker::EndpointConfigurationShadowProductionVariantServerlessConfig>>,
    /// The name of the variant. If omitted, this provider will assign a random, unique name.
    #[builder(into)]
    #[serde(rename = "variantName")]
    pub r#variant_name: Option<String>,
    /// The size, in GB, of the ML storage volume attached to individual inference instance associated with the production variant. Valid values between `1` and `512`.
    #[builder(into)]
    #[serde(rename = "volumeSizeInGb")]
    pub r#volume_size_in_gb: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointConfigurationShadowProductionVariant {
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
                    "acceleratorType",
                    &self.r#accelerator_type,
                ),
                to_pulumi_object_field(
                    "containerStartupHealthCheckTimeoutInSeconds",
                    &self.r#container_startup_health_check_timeout_in_seconds,
                ),
                to_pulumi_object_field(
                    "coreDumpConfig",
                    &self.r#core_dump_config,
                ),
                to_pulumi_object_field(
                    "enableSsmAccess",
                    &self.r#enable_ssm_access,
                ),
                to_pulumi_object_field(
                    "inferenceAmiVersion",
                    &self.r#inference_ami_version,
                ),
                to_pulumi_object_field(
                    "initialInstanceCount",
                    &self.r#initial_instance_count,
                ),
                to_pulumi_object_field(
                    "initialVariantWeight",
                    &self.r#initial_variant_weight,
                ),
                to_pulumi_object_field(
                    "instanceType",
                    &self.r#instance_type,
                ),
                to_pulumi_object_field(
                    "managedInstanceScaling",
                    &self.r#managed_instance_scaling,
                ),
                to_pulumi_object_field(
                    "modelDataDownloadTimeoutInSeconds",
                    &self.r#model_data_download_timeout_in_seconds,
                ),
                to_pulumi_object_field(
                    "modelName",
                    &self.r#model_name,
                ),
                to_pulumi_object_field(
                    "routingConfigs",
                    &self.r#routing_configs,
                ),
                to_pulumi_object_field(
                    "serverlessConfig",
                    &self.r#serverless_config,
                ),
                to_pulumi_object_field(
                    "variantName",
                    &self.r#variant_name,
                ),
                to_pulumi_object_field(
                    "volumeSizeInGb",
                    &self.r#volume_size_in_gb,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointConfigurationShadowProductionVariant {
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
                    r#accelerator_type: {
                        let field_value = match fields_map.get("acceleratorType") {
                            Some(value) => value,
                            None => bail!("Missing field 'acceleratorType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_startup_health_check_timeout_in_seconds: {
                        let field_value = match fields_map.get("containerStartupHealthCheckTimeoutInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerStartupHealthCheckTimeoutInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#core_dump_config: {
                        let field_value = match fields_map.get("coreDumpConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'coreDumpConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_ssm_access: {
                        let field_value = match fields_map.get("enableSsmAccess") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableSsmAccess' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inference_ami_version: {
                        let field_value = match fields_map.get("inferenceAmiVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'inferenceAmiVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#initial_instance_count: {
                        let field_value = match fields_map.get("initialInstanceCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'initialInstanceCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#initial_variant_weight: {
                        let field_value = match fields_map.get("initialVariantWeight") {
                            Some(value) => value,
                            None => bail!("Missing field 'initialVariantWeight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_type: {
                        let field_value = match fields_map.get("instanceType") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_instance_scaling: {
                        let field_value = match fields_map.get("managedInstanceScaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'managedInstanceScaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_data_download_timeout_in_seconds: {
                        let field_value = match fields_map.get("modelDataDownloadTimeoutInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'modelDataDownloadTimeoutInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_name: {
                        let field_value = match fields_map.get("modelName") {
                            Some(value) => value,
                            None => bail!("Missing field 'modelName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#routing_configs: {
                        let field_value = match fields_map.get("routingConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'routingConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serverless_config: {
                        let field_value = match fields_map.get("serverlessConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'serverlessConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#variant_name: {
                        let field_value = match fields_map.get("variantName") {
                            Some(value) => value,
                            None => bail!("Missing field 'variantName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_size_in_gb: {
                        let field_value = match fields_map.get("volumeSizeInGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'volumeSizeInGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
