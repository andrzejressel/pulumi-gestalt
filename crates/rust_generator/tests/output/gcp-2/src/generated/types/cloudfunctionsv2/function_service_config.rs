#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FunctionServiceConfig {
    /// Whether 100% of traffic is routed to the latest revision. Defaults to true.
    #[builder(into)]
    #[serde(rename = "allTrafficOnLatestRevision")]
    pub r#all_traffic_on_latest_revision: Option<bool>,
    /// The number of CPUs used in a single container instance. Default value is calculated from available memory.
    #[builder(into)]
    #[serde(rename = "availableCpu")]
    pub r#available_cpu: Option<String>,
    /// The amount of memory available for a function.
    /// Defaults to 256M. Supported units are k, M, G, Mi, Gi. If no unit is
    /// supplied the value is interpreted as bytes.
    #[builder(into)]
    #[serde(rename = "availableMemory")]
    pub r#available_memory: Option<String>,
    /// Environment variables that shall be available during function execution.
    #[builder(into)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Option<std::collections::HashMap<String, String>>,
    /// (Output)
    /// URIs of the Service deployed
    #[builder(into)]
    #[serde(rename = "gcfUri")]
    pub r#gcf_uri: Option<String>,
    /// Available ingress settings. Defaults to "ALLOW_ALL" if unspecified.
    /// Default value is `ALLOW_ALL`.
    /// Possible values are: `ALLOW_ALL`, `ALLOW_INTERNAL_ONLY`, `ALLOW_INTERNAL_AND_GCLB`.
    #[builder(into)]
    #[serde(rename = "ingressSettings")]
    pub r#ingress_settings: Option<String>,
    /// The limit on the maximum number of function instances that may coexist at a
    /// given time.
    #[builder(into)]
    #[serde(rename = "maxInstanceCount")]
    pub r#max_instance_count: Option<i32>,
    /// Sets the maximum number of concurrent requests that each instance can receive. Defaults to 1.
    #[builder(into)]
    #[serde(rename = "maxInstanceRequestConcurrency")]
    pub r#max_instance_request_concurrency: Option<i32>,
    /// The limit on the minimum number of function instances that may coexist at a
    /// given time.
    #[builder(into)]
    #[serde(rename = "minInstanceCount")]
    pub r#min_instance_count: Option<i32>,
    /// Secret environment variables configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "secretEnvironmentVariables")]
    pub r#secret_environment_variables: Option<Vec<super::super::types::cloudfunctionsv2::FunctionServiceConfigSecretEnvironmentVariable>>,
    /// Secret volumes configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "secretVolumes")]
    pub r#secret_volumes: Option<Vec<super::super::types::cloudfunctionsv2::FunctionServiceConfigSecretVolume>>,
    /// Name of the service associated with a Function.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Option<String>,
    /// The email of the service account for this function.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Option<String>,
    /// The function execution timeout. Execution is considered failed and
    /// can be terminated if the function is not completed at the end of the
    /// timeout period. Defaults to 60 seconds.
    #[builder(into)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Option<i32>,
    /// (Output)
    /// URI of the Service deployed.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Option<String>,
    /// The Serverless VPC Access connector that this cloud function can connect to.
    #[builder(into)]
    #[serde(rename = "vpcConnector")]
    pub r#vpc_connector: Option<String>,
    /// Available egress settings.
    /// Possible values are: `VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED`, `PRIVATE_RANGES_ONLY`, `ALL_TRAFFIC`.
    #[builder(into)]
    #[serde(rename = "vpcConnectorEgressSettings")]
    pub r#vpc_connector_egress_settings: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FunctionServiceConfig {
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
                    "all_traffic_on_latest_revision",
                    &self.r#all_traffic_on_latest_revision,
                ),
                to_pulumi_object_field(
                    "available_cpu",
                    &self.r#available_cpu,
                ),
                to_pulumi_object_field(
                    "available_memory",
                    &self.r#available_memory,
                ),
                to_pulumi_object_field(
                    "environment_variables",
                    &self.r#environment_variables,
                ),
                to_pulumi_object_field(
                    "gcf_uri",
                    &self.r#gcf_uri,
                ),
                to_pulumi_object_field(
                    "ingress_settings",
                    &self.r#ingress_settings,
                ),
                to_pulumi_object_field(
                    "max_instance_count",
                    &self.r#max_instance_count,
                ),
                to_pulumi_object_field(
                    "max_instance_request_concurrency",
                    &self.r#max_instance_request_concurrency,
                ),
                to_pulumi_object_field(
                    "min_instance_count",
                    &self.r#min_instance_count,
                ),
                to_pulumi_object_field(
                    "secret_environment_variables",
                    &self.r#secret_environment_variables,
                ),
                to_pulumi_object_field(
                    "secret_volumes",
                    &self.r#secret_volumes,
                ),
                to_pulumi_object_field(
                    "service",
                    &self.r#service,
                ),
                to_pulumi_object_field(
                    "service_account_email",
                    &self.r#service_account_email,
                ),
                to_pulumi_object_field(
                    "timeout_seconds",
                    &self.r#timeout_seconds,
                ),
                to_pulumi_object_field(
                    "uri",
                    &self.r#uri,
                ),
                to_pulumi_object_field(
                    "vpc_connector",
                    &self.r#vpc_connector,
                ),
                to_pulumi_object_field(
                    "vpc_connector_egress_settings",
                    &self.r#vpc_connector_egress_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FunctionServiceConfig {
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
                    r#all_traffic_on_latest_revision: {
                        let field_value = match fields_map.get("all_traffic_on_latest_revision") {
                            Some(value) => value,
                            None => bail!("Missing field 'all_traffic_on_latest_revision' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_cpu: {
                        let field_value = match fields_map.get("available_cpu") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_cpu' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#available_memory: {
                        let field_value = match fields_map.get("available_memory") {
                            Some(value) => value,
                            None => bail!("Missing field 'available_memory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#environment_variables: {
                        let field_value = match fields_map.get("environment_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'environment_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcf_uri: {
                        let field_value = match fields_map.get("gcf_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcf_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ingress_settings: {
                        let field_value = match fields_map.get("ingress_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingress_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_instance_count: {
                        let field_value = match fields_map.get("max_instance_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_instance_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_instance_request_concurrency: {
                        let field_value = match fields_map.get("max_instance_request_concurrency") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_instance_request_concurrency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_instance_count: {
                        let field_value = match fields_map.get("min_instance_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_instance_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_environment_variables: {
                        let field_value = match fields_map.get("secret_environment_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_environment_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_volumes: {
                        let field_value = match fields_map.get("secret_volumes") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_volumes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service: {
                        let field_value = match fields_map.get("service") {
                            Some(value) => value,
                            None => bail!("Missing field 'service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_email: {
                        let field_value = match fields_map.get("service_account_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_seconds: {
                        let field_value = match fields_map.get("timeout_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uri: {
                        let field_value = match fields_map.get("uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_connector: {
                        let field_value = match fields_map.get("vpc_connector") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_connector' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_connector_egress_settings: {
                        let field_value = match fields_map.get("vpc_connector_egress_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_connector_egress_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
