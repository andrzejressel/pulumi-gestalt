#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionGithubConfig {
    /// Optional. GitHub App installation id.
    #[builder(into)]
    #[serde(rename = "appInstallationId")]
    pub r#app_installation_id: Option<String>,
    /// Represents an OAuth token of the account that authorized the Connection,and
    /// associated metadata.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "authorizerCredential")]
    pub r#authorizer_credential: Option<Box<super::super::types::developerconnect::ConnectionGithubConfigAuthorizerCredential>>,
    /// Required. Immutable. The GitHub Application that was installed to
    /// the GitHub user or organization.
    /// Possible values:
    /// GIT_HUB_APP_UNSPECIFIED
    /// DEVELOPER_CONNECT
    /// FIREBASE"
    #[builder(into)]
    #[serde(rename = "githubApp")]
    pub r#github_app: String,
    /// (Output)
    /// Output only. The URI to navigate to in order to manage the installation
    /// associated with this GitHubConfig.
    #[builder(into)]
    #[serde(rename = "installationUri")]
    pub r#installation_uri: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionGithubConfig {
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
                    "app_installation_id",
                    &self.r#app_installation_id,
                ),
                to_pulumi_object_field(
                    "authorizer_credential",
                    &self.r#authorizer_credential,
                ),
                to_pulumi_object_field(
                    "github_app",
                    &self.r#github_app,
                ),
                to_pulumi_object_field(
                    "installation_uri",
                    &self.r#installation_uri,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionGithubConfig {
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
                    r#app_installation_id: {
                        let field_value = match fields_map.get("app_installation_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_installation_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authorizer_credential: {
                        let field_value = match fields_map.get("authorizer_credential") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorizer_credential' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#github_app: {
                        let field_value = match fields_map.get("github_app") {
                            Some(value) => value,
                            None => bail!("Missing field 'github_app' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#installation_uri: {
                        let field_value = match fields_map.get("installation_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'installation_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
