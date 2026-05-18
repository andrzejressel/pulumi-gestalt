#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceSourceConfigurationImageRepositoryImageConfiguration {
    /// Port that your application listens to in the container. Defaults to `"8080"`.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<String>,
    /// Secrets and parameters available to your service as environment variables. A map of key/value pairs, where the key is the desired name of the Secret in the environment (i.e. it does not have to match the name of the secret in Secrets Manager or SSM Parameter Store), and the value is the ARN of the secret from AWS Secrets Manager or the ARN of the parameter in AWS SSM Parameter Store.
    #[builder(into)]
    #[serde(rename = "runtimeEnvironmentSecrets")]
    pub r#runtime_environment_secrets: Option<std::collections::HashMap<String, String>>,
    /// Environment variables available to your running App Runner service. A map of key/value pairs. Keys with a prefix of `AWSAPPRUNNER` are reserved for system use and aren't valid.
    #[builder(into)]
    #[serde(rename = "runtimeEnvironmentVariables")]
    pub r#runtime_environment_variables: Option<std::collections::HashMap<String, String>>,
    /// Command App Runner runs to start the application in the source image. If specified, this command overrides the Docker image’s default start command.
    #[builder(into)]
    #[serde(rename = "startCommand")]
    pub r#start_command: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceSourceConfigurationImageRepositoryImageConfiguration {
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
                    "port",
                    &self.r#port,
                ),
                to_pulumi_object_field(
                    "runtime_environment_secrets",
                    &self.r#runtime_environment_secrets,
                ),
                to_pulumi_object_field(
                    "runtime_environment_variables",
                    &self.r#runtime_environment_variables,
                ),
                to_pulumi_object_field(
                    "start_command",
                    &self.r#start_command,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceSourceConfigurationImageRepositoryImageConfiguration {
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
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#runtime_environment_secrets: {
                        let field_value = match fields_map.get("runtime_environment_secrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'runtime_environment_secrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#runtime_environment_variables: {
                        let field_value = match fields_map.get("runtime_environment_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'runtime_environment_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_command: {
                        let field_value = match fields_map.get("start_command") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_command' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
