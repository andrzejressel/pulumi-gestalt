#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppAutoBranchCreationConfig {
    /// Basic authorization credentials for the autocreated branch.
    #[builder(into)]
    pub r#basic_auth_credentials: Option<String>,
    /// Build specification (build spec) for the autocreated branch.
    #[builder(into)]
    pub r#build_spec: Option<String>,
    /// Enables auto building for the autocreated branch.
    #[builder(into)]
    pub r#enable_auto_build: Option<bool>,
    /// Enables basic authorization for the autocreated branch.
    #[builder(into)]
    pub r#enable_basic_auth: Option<bool>,
    /// Enables performance mode for the branch.
    #[builder(into)]
    pub r#enable_performance_mode: Option<bool>,
    /// Enables pull request previews for the autocreated branch.
    #[builder(into)]
    pub r#enable_pull_request_preview: Option<bool>,
    /// Environment variables for the autocreated branch.
    #[builder(into)]
    pub r#environment_variables: Option<std::collections::BTreeMap<String, String>>,
    /// Framework for the autocreated branch.
    #[builder(into)]
    pub r#framework: Option<String>,
    /// Amplify environment name for the pull request.
    #[builder(into)]
    pub r#pull_request_environment_name: Option<String>,
    /// Describes the current stage for the autocreated branch. Valid values: `PRODUCTION`, `BETA`, `DEVELOPMENT`, `EXPERIMENTAL`, `PULL_REQUEST`.
    #[builder(into)]
    pub r#stage: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AppAutoBranchCreationConfig {
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
                    "basicAuthCredentials",
                    &self.r#basic_auth_credentials,
                ),
                to_pulumi_object_field(
                    "buildSpec",
                    &self.r#build_spec,
                ),
                to_pulumi_object_field(
                    "enableAutoBuild",
                    &self.r#enable_auto_build,
                ),
                to_pulumi_object_field(
                    "enableBasicAuth",
                    &self.r#enable_basic_auth,
                ),
                to_pulumi_object_field(
                    "enablePerformanceMode",
                    &self.r#enable_performance_mode,
                ),
                to_pulumi_object_field(
                    "enablePullRequestPreview",
                    &self.r#enable_pull_request_preview,
                ),
                to_pulumi_object_field(
                    "environmentVariables",
                    &self.r#environment_variables,
                ),
                to_pulumi_object_field(
                    "framework",
                    &self.r#framework,
                ),
                to_pulumi_object_field(
                    "pullRequestEnvironmentName",
                    &self.r#pull_request_environment_name,
                ),
                to_pulumi_object_field(
                    "stage",
                    &self.r#stage,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AppAutoBranchCreationConfig {
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
                    r#basic_auth_credentials: {
                        let field_value = match fields_map.get("basicAuthCredentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'basicAuthCredentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#build_spec: {
                        let field_value = match fields_map.get("buildSpec") {
                            Some(value) => value,
                            None => bail!("Missing field 'buildSpec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_auto_build: {
                        let field_value = match fields_map.get("enableAutoBuild") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableAutoBuild' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_basic_auth: {
                        let field_value = match fields_map.get("enableBasicAuth") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableBasicAuth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_performance_mode: {
                        let field_value = match fields_map.get("enablePerformanceMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'enablePerformanceMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_pull_request_preview: {
                        let field_value = match fields_map.get("enablePullRequestPreview") {
                            Some(value) => value,
                            None => bail!("Missing field 'enablePullRequestPreview' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#environment_variables: {
                        let field_value = match fields_map.get("environmentVariables") {
                            Some(value) => value,
                            None => bail!("Missing field 'environmentVariables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#framework: {
                        let field_value = match fields_map.get("framework") {
                            Some(value) => value,
                            None => bail!("Missing field 'framework' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pull_request_environment_name: {
                        let field_value = match fields_map.get("pullRequestEnvironmentName") {
                            Some(value) => value,
                            None => bail!("Missing field 'pullRequestEnvironmentName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stage: {
                        let field_value = match fields_map.get("stage") {
                            Some(value) => value,
                            None => bail!("Missing field 'stage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
