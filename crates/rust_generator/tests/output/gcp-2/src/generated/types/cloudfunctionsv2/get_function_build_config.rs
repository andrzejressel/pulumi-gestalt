#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFunctionBuildConfig {
    /// Security patches are applied automatically to the runtime without requiring
    /// the function to be redeployed.
    #[builder(into)]
    #[serde(rename = "automaticUpdatePolicies")]
    pub r#automatic_update_policies: Vec<super::super::types::cloudfunctionsv2::GetFunctionBuildConfigAutomaticUpdatePolicy>,
    /// The Cloud Build name of the latest successful
    /// deployment of the function.
    #[builder(into)]
    #[serde(rename = "build")]
    pub r#build: String,
    /// User managed repository created in Artifact Registry optionally with a customer managed encryption key.
    #[builder(into)]
    #[serde(rename = "dockerRepository")]
    pub r#docker_repository: String,
    /// The name of the function (as defined in source code) that will be executed.
    /// Defaults to the resource name suffix, if not specified. For backward
    /// compatibility, if function with given name is not found, then the system
    /// will try to use function named "function". For Node.js this is name of a
    /// function exported by the module specified in source_location.
    #[builder(into)]
    #[serde(rename = "entryPoint")]
    pub r#entry_point: String,
    /// User-provided build-time environment variables for the function.
    #[builder(into)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: std::collections::HashMap<String, String>,
    /// Security patches are only applied when a function is redeployed.
    #[builder(into)]
    #[serde(rename = "onDeployUpdatePolicies")]
    pub r#on_deploy_update_policies: Vec<super::super::types::cloudfunctionsv2::GetFunctionBuildConfigOnDeployUpdatePolicy>,
    /// The runtime in which to run the function. Required when deploying a new
    /// function, optional when updating an existing function.
    #[builder(into)]
    #[serde(rename = "runtime")]
    pub r#runtime: String,
    /// The fully-qualified name of the service account to be used for building the container.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: String,
    /// The location of the function source code.
    #[builder(into)]
    #[serde(rename = "sources")]
    pub r#sources: Vec<super::super::types::cloudfunctionsv2::GetFunctionBuildConfigSource>,
    /// Name of the Cloud Build Custom Worker Pool that should be used to build the function.
    #[builder(into)]
    #[serde(rename = "workerPool")]
    pub r#worker_pool: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetFunctionBuildConfig {
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
                    "automatic_update_policies",
                    &self.r#automatic_update_policies,
                ),
                to_pulumi_object_field(
                    "build",
                    &self.r#build,
                ),
                to_pulumi_object_field(
                    "docker_repository",
                    &self.r#docker_repository,
                ),
                to_pulumi_object_field(
                    "entry_point",
                    &self.r#entry_point,
                ),
                to_pulumi_object_field(
                    "environment_variables",
                    &self.r#environment_variables,
                ),
                to_pulumi_object_field(
                    "on_deploy_update_policies",
                    &self.r#on_deploy_update_policies,
                ),
                to_pulumi_object_field(
                    "runtime",
                    &self.r#runtime,
                ),
                to_pulumi_object_field(
                    "service_account",
                    &self.r#service_account,
                ),
                to_pulumi_object_field(
                    "sources",
                    &self.r#sources,
                ),
                to_pulumi_object_field(
                    "worker_pool",
                    &self.r#worker_pool,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetFunctionBuildConfig {
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
                    r#automatic_update_policies: {
                        let field_value = match fields_map.get("automatic_update_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'automatic_update_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#build: {
                        let field_value = match fields_map.get("build") {
                            Some(value) => value,
                            None => bail!("Missing field 'build' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docker_repository: {
                        let field_value = match fields_map.get("docker_repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'docker_repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#entry_point: {
                        let field_value = match fields_map.get("entry_point") {
                            Some(value) => value,
                            None => bail!("Missing field 'entry_point' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#on_deploy_update_policies: {
                        let field_value = match fields_map.get("on_deploy_update_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_deploy_update_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#runtime: {
                        let field_value = match fields_map.get("runtime") {
                            Some(value) => value,
                            None => bail!("Missing field 'runtime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#sources: {
                        let field_value = match fields_map.get("sources") {
                            Some(value) => value,
                            None => bail!("Missing field 'sources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#worker_pool: {
                        let field_value = match fields_map.get("worker_pool") {
                            Some(value) => value,
                            None => bail!("Missing field 'worker_pool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
