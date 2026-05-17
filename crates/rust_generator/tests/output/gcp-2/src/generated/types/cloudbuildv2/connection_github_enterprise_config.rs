#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionGithubEnterpriseConfig {
    /// Id of the GitHub App created from the manifest.
    #[builder(into)]
    #[serde(rename = "appId")]
    pub r#app_id: Option<i32>,
    /// ID of the installation of the GitHub App.
    #[builder(into)]
    #[serde(rename = "appInstallationId")]
    pub r#app_installation_id: Option<i32>,
    /// The URL-friendly name of the GitHub App.
    #[builder(into)]
    #[serde(rename = "appSlug")]
    pub r#app_slug: Option<String>,
    /// Required. The URI of the GitHub Enterprise host this connection is for.
    #[builder(into)]
    #[serde(rename = "hostUri")]
    pub r#host_uri: String,
    /// SecretManager resource containing the private key of the GitHub App, formatted as `projects/*/secrets/*/versions/*`.
    #[builder(into)]
    #[serde(rename = "privateKeySecretVersion")]
    pub r#private_key_secret_version: Option<String>,
    /// Configuration for using Service Directory to privately connect to a GitHub Enterprise server. This should only be set if the GitHub Enterprise server is hosted on-premises and not reachable by public internet. If this field is left empty, calls to the GitHub Enterprise server will be made over the public internet.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "serviceDirectoryConfig")]
    pub r#service_directory_config: Option<Box<super::super::types::cloudbuildv2::ConnectionGithubEnterpriseConfigServiceDirectoryConfig>>,
    /// SSL certificate to use for requests to GitHub Enterprise.
    #[builder(into)]
    #[serde(rename = "sslCa")]
    pub r#ssl_ca: Option<String>,
    /// SecretManager resource containing the webhook secret of the GitHub App, formatted as `projects/*/secrets/*/versions/*`.
    #[builder(into)]
    #[serde(rename = "webhookSecretSecretVersion")]
    pub r#webhook_secret_secret_version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionGithubEnterpriseConfig {
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
                    "app_id",
                    &self.r#app_id,
                ),
                to_pulumi_object_field(
                    "app_installation_id",
                    &self.r#app_installation_id,
                ),
                to_pulumi_object_field(
                    "app_slug",
                    &self.r#app_slug,
                ),
                to_pulumi_object_field(
                    "host_uri",
                    &self.r#host_uri,
                ),
                to_pulumi_object_field(
                    "private_key_secret_version",
                    &self.r#private_key_secret_version,
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
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionGithubEnterpriseConfig {
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
                    r#app_id: {
                        let field_value = match fields_map.get("app_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_installation_id: {
                        let field_value = match fields_map.get("app_installation_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_installation_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_slug: {
                        let field_value = match fields_map.get("app_slug") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_slug' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#private_key_secret_version: {
                        let field_value = match fields_map.get("private_key_secret_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_key_secret_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
