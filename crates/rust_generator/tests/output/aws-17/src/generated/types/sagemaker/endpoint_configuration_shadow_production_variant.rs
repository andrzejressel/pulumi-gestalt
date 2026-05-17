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
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "accelerator_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#accelerator_type,
                )
                .await,
            );
            map.insert(
                "container_startup_health_check_timeout_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_startup_health_check_timeout_in_seconds,
                )
                .await,
            );
            map.insert(
                "core_dump_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#core_dump_config,
                )
                .await,
            );
            map.insert(
                "enable_ssm_access".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_ssm_access,
                )
                .await,
            );
            map.insert(
                "inference_ami_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#inference_ami_version,
                )
                .await,
            );
            map.insert(
                "initial_instance_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#initial_instance_count,
                )
                .await,
            );
            map.insert(
                "initial_variant_weight".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#initial_variant_weight,
                )
                .await,
            );
            map.insert(
                "instance_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_type,
                )
                .await,
            );
            map.insert(
                "managed_instance_scaling".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#managed_instance_scaling,
                )
                .await,
            );
            map.insert(
                "model_data_download_timeout_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#model_data_download_timeout_in_seconds,
                )
                .await,
            );
            map.insert(
                "model_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#model_name,
                )
                .await,
            );
            map.insert(
                "routing_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#routing_configs,
                )
                .await,
            );
            map.insert(
                "serverless_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#serverless_config,
                )
                .await,
            );
            map.insert(
                "variant_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#variant_name,
                )
                .await,
            );
            map.insert(
                "volume_size_in_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#volume_size_in_gb,
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
                        let field_value = match fields_map.get("accelerator_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerator_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_startup_health_check_timeout_in_seconds: {
                        let field_value = match fields_map.get("container_startup_health_check_timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_startup_health_check_timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#core_dump_config: {
                        let field_value = match fields_map.get("core_dump_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'core_dump_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_ssm_access: {
                        let field_value = match fields_map.get("enable_ssm_access") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_ssm_access' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inference_ami_version: {
                        let field_value = match fields_map.get("inference_ami_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'inference_ami_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#initial_instance_count: {
                        let field_value = match fields_map.get("initial_instance_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'initial_instance_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#initial_variant_weight: {
                        let field_value = match fields_map.get("initial_variant_weight") {
                            Some(value) => value,
                            None => bail!("Missing field 'initial_variant_weight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_type: {
                        let field_value = match fields_map.get("instance_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_instance_scaling: {
                        let field_value = match fields_map.get("managed_instance_scaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_instance_scaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_data_download_timeout_in_seconds: {
                        let field_value = match fields_map.get("model_data_download_timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'model_data_download_timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_name: {
                        let field_value = match fields_map.get("model_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'model_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#routing_configs: {
                        let field_value = match fields_map.get("routing_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'routing_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serverless_config: {
                        let field_value = match fields_map.get("serverless_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'serverless_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#variant_name: {
                        let field_value = match fields_map.get("variant_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'variant_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_size_in_gb: {
                        let field_value = match fields_map.get("volume_size_in_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_size_in_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
