#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValues {
    /// Command App Runner runs to build your application.
    #[builder(into)]
    #[serde(rename = "buildCommand")]
    pub r#build_command: Option<String>,
    /// Port that your application listens to in the container. Defaults to `"8080"`.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<String>,
    /// Runtime environment type for building and running an App Runner service. Represents a programming language runtime. Valid values: `PYTHON_3`, `NODEJS_12`, `NODEJS_14`, `NODEJS_16`, `CORRETTO_8`, `CORRETTO_11`, `GO_1`, `DOTNET_6`, `PHP_81`, `RUBY_31`.
    #[builder(into)]
    #[serde(rename = "runtime")]
    pub r#runtime: String,
    /// Secrets and parameters available to your service as environment variables. A map of key/value pairs, where the key is the desired name of the Secret in the environment (i.e. it does not have to match the name of the secret in Secrets Manager or SSM Parameter Store), and the value is the ARN of the secret from AWS Secrets Manager or the ARN of the parameter in AWS SSM Parameter Store.
    #[builder(into)]
    #[serde(rename = "runtimeEnvironmentSecrets")]
    pub r#runtime_environment_secrets: Option<std::collections::HashMap<String, String>>,
    /// Environment variables available to your running App Runner service. A map of key/value pairs. Keys with a prefix of `AWSAPPRUNNER` are reserved for system use and aren't valid.
    #[builder(into)]
    #[serde(rename = "runtimeEnvironmentVariables")]
    pub r#runtime_environment_variables: Option<std::collections::HashMap<String, String>>,
    /// Command App Runner runs to start your application.
    #[builder(into)]
    #[serde(rename = "startCommand")]
    pub r#start_command: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValues {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("build_command".to_string(), self.r#build_command.to_pulumi_value().await);
            map.insert("port".to_string(), self.r#port.to_pulumi_value().await);
            map.insert("runtime".to_string(), self.r#runtime.to_pulumi_value().await);
            map.insert("runtime_environment_secrets".to_string(), self.r#runtime_environment_secrets.to_pulumi_value().await);
            map.insert("runtime_environment_variables".to_string(), self.r#runtime_environment_variables.to_pulumi_value().await);
            map.insert("start_command".to_string(), self.r#start_command.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceSourceConfigurationCodeRepositoryCodeConfigurationCodeConfigurationValues {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#build_command: {
                        let field_value = match fields_map.get("build_command") {
                            Some(value) => value,
                            None => bail!("Missing field 'build_command' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#runtime: {
                        let field_value = match fields_map.get("runtime") {
                            Some(value) => value,
                            None => bail!("Missing field 'runtime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#runtime_environment_secrets: {
                        let field_value = match fields_map.get("runtime_environment_secrets") {
                            Some(value) => value,
                            None => bail!("Missing field 'runtime_environment_secrets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#runtime_environment_variables: {
                        let field_value = match fields_map.get("runtime_environment_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'runtime_environment_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#start_command: {
                        let field_value = match fields_map.get("start_command") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_command' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
