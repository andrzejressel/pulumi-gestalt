#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryGitRemoteSettings {
    /// The name of the Secret Manager secret version to use as an authentication token for Git operations. This secret is for assigning with HTTPS only(for SSH use `ssh_authentication_config`). Must be in the format projects/*/secrets/*/versions/*.
    #[builder(into)]
    #[serde(rename = "authenticationTokenSecretVersion")]
    pub r#authentication_token_secret_version: Option<String>,
    /// The Git remote's default branch name.
    #[builder(into)]
    #[serde(rename = "defaultBranch")]
    pub r#default_branch: String,
    /// Authentication fields for remote uris using SSH protocol.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sshAuthenticationConfig")]
    pub r#ssh_authentication_config: Option<Box<super::super::types::dataform::RepositoryGitRemoteSettingsSshAuthenticationConfig>>,
    /// (Output)
    /// Indicates the status of the Git access token. https://cloud.google.com/dataform/reference/rest/v1beta1/projects.locations.repositories#TokenStatus
    #[builder(into)]
    #[serde(rename = "tokenStatus")]
    pub r#token_status: Option<String>,
    /// The Git remote's URL.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RepositoryGitRemoteSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "authentication_token_secret_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authentication_token_secret_version,
                )
                .await,
            );
            map.insert(
                "default_branch".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_branch,
                )
                .await,
            );
            map.insert(
                "ssh_authentication_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssh_authentication_config,
                )
                .await,
            );
            map.insert(
                "token_status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#token_status,
                )
                .await,
            );
            map.insert(
                "url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#url,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RepositoryGitRemoteSettings {
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
                    r#authentication_token_secret_version: {
                        let field_value = match fields_map.get("authentication_token_secret_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication_token_secret_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_branch: {
                        let field_value = match fields_map.get("default_branch") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_branch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_authentication_config: {
                        let field_value = match fields_map.get("ssh_authentication_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_authentication_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_status: {
                        let field_value = match fields_map.get("token_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'token_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url: {
                        let field_value = match fields_map.get("url") {
                            Some(value) => value,
                            None => bail!("Missing field 'url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
