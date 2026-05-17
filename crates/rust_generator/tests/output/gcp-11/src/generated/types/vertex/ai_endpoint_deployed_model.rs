#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiEndpointDeployedModel {
    /// (Output)
    /// A description of resources that to large degree are decided by Vertex AI, and require only a modest additional configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "automaticResources")]
    pub r#automatic_resources: Option<Vec<super::super::types::vertex::AiEndpointDeployedModelAutomaticResource>>,
    /// (Output)
    /// Output only. Timestamp when the DeployedModel was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Option<String>,
    /// (Output)
    /// A description of resources that are dedicated to the DeployedModel, and that need a higher degree of manual configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dedicatedResources")]
    pub r#dedicated_resources: Option<Vec<super::super::types::vertex::AiEndpointDeployedModelDedicatedResource>>,
    /// Required. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// (Output)
    /// These logs are like standard server access logs, containing information like timestamp and latency for each prediction request. Note that Stackdriver logs may incur a cost, especially if your project receives prediction requests at a high queries per second rate (QPS). Estimate your costs before enabling this option.
    #[builder(into)]
    #[serde(rename = "enableAccessLogging")]
    pub r#enable_access_logging: Option<bool>,
    /// (Output)
    /// If true, the container of the DeployedModel instances will send `stderr` and `stdout` streams to Stackdriver Logging. Only supported for custom-trained Models and AutoML Tabular Models.
    #[builder(into)]
    #[serde(rename = "enableContainerLogging")]
    pub r#enable_container_logging: Option<bool>,
    /// (Output)
    /// The ID of the DeployedModel. If not provided upon deployment, Vertex AI will generate a value for this ID. This value should be 1-10 characters, and valid characters are /[0-9]/.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// (Output)
    /// The name of the Model that this is the deployment of. Note that the Model may be in a different location than the DeployedModel's Endpoint.
    #[builder(into)]
    #[serde(rename = "model")]
    pub r#model: Option<String>,
    /// (Output)
    /// Output only. The version ID of the model that is deployed.
    #[builder(into)]
    #[serde(rename = "modelVersionId")]
    pub r#model_version_id: Option<String>,
    /// (Output)
    /// Output only. Provide paths for users to send predict/explain/health requests directly to the deployed model services running on Cloud via private services access. This field is populated if network is configured.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "privateEndpoints")]
    pub r#private_endpoints: Option<Vec<super::super::types::vertex::AiEndpointDeployedModelPrivateEndpoint>>,
    /// (Output)
    /// The service account that the DeployedModel's container runs as. Specify the email address of the service account. If this service account is not specified, the container runs as a service account that doesn't have access to the resource project. Users deploying the Model must have the `iam.serviceAccounts.actAs` permission on this service account.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Option<String>,
    /// (Output)
    /// The resource name of the shared DeploymentResourcePool to deploy on. Format: projects/{project}/locations/{location}/deploymentResourcePools/{deployment_resource_pool}
    #[builder(into)]
    #[serde(rename = "sharedResources")]
    pub r#shared_resources: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AiEndpointDeployedModel {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "automatic_resources",
                    &self.r#automatic_resources,
                ),
                to_pulumi_object_field(
                    "create_time",
                    &self.r#create_time,
                ),
                to_pulumi_object_field(
                    "dedicated_resources",
                    &self.r#dedicated_resources,
                ),
                to_pulumi_object_field(
                    "display_name",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "enable_access_logging",
                    &self.r#enable_access_logging,
                ),
                to_pulumi_object_field(
                    "enable_container_logging",
                    &self.r#enable_container_logging,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "model",
                    &self.r#model,
                ),
                to_pulumi_object_field(
                    "model_version_id",
                    &self.r#model_version_id,
                ),
                to_pulumi_object_field(
                    "private_endpoints",
                    &self.r#private_endpoints,
                ),
                to_pulumi_object_field(
                    "service_account",
                    &self.r#service_account,
                ),
                to_pulumi_object_field(
                    "shared_resources",
                    &self.r#shared_resources,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AiEndpointDeployedModel {
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
                    r#automatic_resources: {
                        let field_value = match fields_map.get("automatic_resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'automatic_resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_time: {
                        let field_value = match fields_map.get("create_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'create_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dedicated_resources: {
                        let field_value = match fields_map.get("dedicated_resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'dedicated_resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("display_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_access_logging: {
                        let field_value = match fields_map.get("enable_access_logging") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_access_logging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_container_logging: {
                        let field_value = match fields_map.get("enable_container_logging") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_container_logging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model: {
                        let field_value = match fields_map.get("model") {
                            Some(value) => value,
                            None => bail!("Missing field 'model' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#model_version_id: {
                        let field_value = match fields_map.get("model_version_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'model_version_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_endpoints: {
                        let field_value = match fields_map.get("private_endpoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_endpoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account: {
                        let field_value = match fields_map.get("service_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shared_resources: {
                        let field_value = match fields_map.get("shared_resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'shared_resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
