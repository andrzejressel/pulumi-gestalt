#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppAutoBranchCreationConfig {
    /// Basic authorization credentials for the autocreated branch.
    #[builder(into)]
    #[serde(rename = "basicAuthCredentials")]
    pub r#basic_auth_credentials: Option<String>,
    /// Build specification (build spec) for the autocreated branch.
    #[builder(into)]
    #[serde(rename = "buildSpec")]
    pub r#build_spec: Option<String>,
    /// Enables auto building for the autocreated branch.
    #[builder(into)]
    #[serde(rename = "enableAutoBuild")]
    pub r#enable_auto_build: Option<bool>,
    /// Enables basic authorization for the autocreated branch.
    #[builder(into)]
    #[serde(rename = "enableBasicAuth")]
    pub r#enable_basic_auth: Option<bool>,
    /// Enables performance mode for the branch.
    #[builder(into)]
    #[serde(rename = "enablePerformanceMode")]
    pub r#enable_performance_mode: Option<bool>,
    /// Enables pull request previews for the autocreated branch.
    #[builder(into)]
    #[serde(rename = "enablePullRequestPreview")]
    pub r#enable_pull_request_preview: Option<bool>,
    /// Environment variables for the autocreated branch.
    #[builder(into)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Option<std::collections::HashMap<String, String>>,
    /// Framework for the autocreated branch.
    #[builder(into)]
    #[serde(rename = "framework")]
    pub r#framework: Option<String>,
    /// Amplify environment name for the pull request.
    #[builder(into)]
    #[serde(rename = "pullRequestEnvironmentName")]
    pub r#pull_request_environment_name: Option<String>,
    /// Describes the current stage for the autocreated branch. Valid values: `PRODUCTION`, `BETA`, `DEVELOPMENT`, `EXPERIMENTAL`, `PULL_REQUEST`.
    #[builder(into)]
    #[serde(rename = "stage")]
    pub r#stage: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AppAutoBranchCreationConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("basic_auth_credentials".to_string(), self.r#basic_auth_credentials.to_pulumi_value().await);
            map.insert("build_spec".to_string(), self.r#build_spec.to_pulumi_value().await);
            map.insert("enable_auto_build".to_string(), self.r#enable_auto_build.to_pulumi_value().await);
            map.insert("enable_basic_auth".to_string(), self.r#enable_basic_auth.to_pulumi_value().await);
            map.insert("enable_performance_mode".to_string(), self.r#enable_performance_mode.to_pulumi_value().await);
            map.insert("enable_pull_request_preview".to_string(), self.r#enable_pull_request_preview.to_pulumi_value().await);
            map.insert("environment_variables".to_string(), self.r#environment_variables.to_pulumi_value().await);
            map.insert("framework".to_string(), self.r#framework.to_pulumi_value().await);
            map.insert("pull_request_environment_name".to_string(), self.r#pull_request_environment_name.to_pulumi_value().await);
            map.insert("stage".to_string(), self.r#stage.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AppAutoBranchCreationConfig {
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
                    r#basic_auth_credentials: {
                        let field_value = match fields_map.get("basic_auth_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'basic_auth_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#build_spec: {
                        let field_value = match fields_map.get("build_spec") {
                            Some(value) => value,
                            None => bail!("Missing field 'build_spec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enable_auto_build: {
                        let field_value = match fields_map.get("enable_auto_build") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_auto_build' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enable_basic_auth: {
                        let field_value = match fields_map.get("enable_basic_auth") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_basic_auth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enable_performance_mode: {
                        let field_value = match fields_map.get("enable_performance_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_performance_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enable_pull_request_preview: {
                        let field_value = match fields_map.get("enable_pull_request_preview") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_pull_request_preview' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#environment_variables: {
                        let field_value = match fields_map.get("environment_variables") {
                            Some(value) => value,
                            None => bail!("Missing field 'environment_variables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#framework: {
                        let field_value = match fields_map.get("framework") {
                            Some(value) => value,
                            None => bail!("Missing field 'framework' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#pull_request_environment_name: {
                        let field_value = match fields_map.get("pull_request_environment_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'pull_request_environment_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#stage: {
                        let field_value = match fields_map.get("stage") {
                            Some(value) => value,
                            None => bail!("Missing field 'stage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
