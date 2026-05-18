#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionGitlabConfig {
    /// Required. A GitLab personal access token with the `api` scope access.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "authorizerCredential")]
    pub r#authorizer_credential: Box<super::super::types::cloudbuildv2::ConnectionGitlabConfigAuthorizerCredential>,
    /// The URI of the GitLab Enterprise host this connection is for. If not specified, the default value is https://gitlab.com.
    #[builder(into)]
    #[serde(rename = "hostUri")]
    pub r#host_uri: Option<String>,
    /// Required. A GitLab personal access token with the minimum `read_api` scope access.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "readAuthorizerCredential")]
    pub r#read_authorizer_credential: Box<super::super::types::cloudbuildv2::ConnectionGitlabConfigReadAuthorizerCredential>,
    /// (Output)
    /// Output only. Version of the GitLab Enterprise server running on the `host_uri`.
    #[builder(into)]
    #[serde(rename = "serverVersion")]
    pub r#server_version: Option<String>,
    /// Configuration for using Service Directory to privately connect to a GitLab Enterprise server. This should only be set if the GitLab Enterprise server is hosted on-premises and not reachable by public internet. If this field is left empty, calls to the GitLab Enterprise server will be made over the public internet.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "serviceDirectoryConfig")]
    pub r#service_directory_config: Option<Box<super::super::types::cloudbuildv2::ConnectionGitlabConfigServiceDirectoryConfig>>,
    /// SSL certificate to use for requests to GitLab Enterprise.
    #[builder(into)]
    #[serde(rename = "sslCa")]
    pub r#ssl_ca: Option<String>,
    /// Required. Immutable. SecretManager resource containing the webhook secret of a GitLab Enterprise project, formatted as `projects/*/secrets/*/versions/*`.
    #[builder(into)]
    #[serde(rename = "webhookSecretSecretVersion")]
    pub r#webhook_secret_secret_version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionGitlabConfig {
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
                    "authorizer_credential",
                    &self.r#authorizer_credential,
                ),
                to_pulumi_object_field(
                    "host_uri",
                    &self.r#host_uri,
                ),
                to_pulumi_object_field(
                    "read_authorizer_credential",
                    &self.r#read_authorizer_credential,
                ),
                to_pulumi_object_field(
                    "server_version",
                    &self.r#server_version,
                ),
                to_pulumi_object_field(
                    "service_directory_config",
                    &self.r#service_directory_config,
                ),
                to_pulumi_object_field(
                    "ssl_ca",
                    &self.r#ssl_ca,
                ),
                to_pulumi_object_field(
                    "webhook_secret_secret_version",
                    &self.r#webhook_secret_secret_version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionGitlabConfig {
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
                    r#authorizer_credential: {
                        let field_value = match fields_map.get("authorizer_credential") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorizer_credential' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_uri: {
                        let field_value = match fields_map.get("host_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_authorizer_credential: {
                        let field_value = match fields_map.get("read_authorizer_credential") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_authorizer_credential' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_version: {
                        let field_value = match fields_map.get("server_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_directory_config: {
                        let field_value = match fields_map.get("service_directory_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_directory_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_ca: {
                        let field_value = match fields_map.get("ssl_ca") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_ca' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#webhook_secret_secret_version: {
                        let field_value = match fields_map.get("webhook_secret_secret_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'webhook_secret_secret_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
