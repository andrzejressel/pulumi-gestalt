#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FluxConfigurationGitRepository {
    /// Specifies the Base64-encoded HTTPS certificate authority contents used to access git private git repositories over HTTPS.
    #[builder(into)]
    #[serde(rename = "httpsCaCertBase64")]
    pub r#https_ca_cert_base_64: Option<String>,
    /// Specifies the Base64-encoded HTTPS personal access token or password that will be used to access the repository.
    #[builder(into)]
    #[serde(rename = "httpsKeyBase64")]
    pub r#https_key_base_64: Option<String>,
    /// Specifies the plaintext HTTPS username used to access private git repositories over HTTPS.
    #[builder(into)]
    #[serde(rename = "httpsUser")]
    pub r#https_user: Option<String>,
    /// Specifies the name of a local secret on the Kubernetes cluster to use as the authentication secret rather than the managed or user-provided configuration secrets. It must be between 1 and 63 characters. It can contain only lowercase letters, numbers, and hyphens (-). It must start and end with a lowercase letter or number.
    #[builder(into)]
    #[serde(rename = "localAuthReference")]
    pub r#local_auth_reference: Option<String>,
    /// Specifies the source reference type for the GitRepository object. Possible values are `branch`, `commit`, `semver` and `tag`.
    #[builder(into)]
    #[serde(rename = "referenceType")]
    pub r#reference_type: String,
    /// Specifies the source reference value for the GitRepository object.
    #[builder(into)]
    #[serde(rename = "referenceValue")]
    pub r#reference_value: String,
    /// Specifies the Base64-encoded known_hosts value containing public SSH keys required to access private git repositories over SSH.
    #[builder(into)]
    #[serde(rename = "sshKnownHostsBase64")]
    pub r#ssh_known_hosts_base_64: Option<String>,
    /// Specifies the Base64-encoded SSH private key in PEM format.
    #[builder(into)]
    #[serde(rename = "sshPrivateKeyBase64")]
    pub r#ssh_private_key_base_64: Option<String>,
    /// Specifies the interval at which to re-reconcile the cluster git repository source with the remote. Defaults to `600`.
    #[builder(into)]
    #[serde(rename = "syncIntervalInSeconds")]
    pub r#sync_interval_in_seconds: Option<i32>,
    /// Specifies the maximum time to attempt to reconcile the cluster git repository source with the remote. Defaults to `600`.
    #[builder(into)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Option<i32>,
    /// Specifies the URL to sync for the flux configuration git repository. It must start with `http://`, `https://`, `git@` or `ssh://`.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FluxConfigurationGitRepository {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "https_ca_cert_base_64".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#https_ca_cert_base_64,
                )
                .await,
            );
            map.insert(
                "https_key_base_64".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#https_key_base_64,
                )
                .await,
            );
            map.insert(
                "https_user".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#https_user,
                )
                .await,
            );
            map.insert(
                "local_auth_reference".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_auth_reference,
                )
                .await,
            );
            map.insert(
                "reference_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reference_type,
                )
                .await,
            );
            map.insert(
                "reference_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reference_value,
                )
                .await,
            );
            map.insert(
                "ssh_known_hosts_base_64".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssh_known_hosts_base_64,
                )
                .await,
            );
            map.insert(
                "ssh_private_key_base_64".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssh_private_key_base_64,
                )
                .await,
            );
            map.insert(
                "sync_interval_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sync_interval_in_seconds,
                )
                .await,
            );
            map.insert(
                "timeout_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeout_in_seconds,
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
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FluxConfigurationGitRepository {
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
                    r#https_ca_cert_base_64: {
                        let field_value = match fields_map.get("https_ca_cert_base_64") {
                            Some(value) => value,
                            None => bail!("Missing field 'https_ca_cert_base_64' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#https_key_base_64: {
                        let field_value = match fields_map.get("https_key_base_64") {
                            Some(value) => value,
                            None => bail!("Missing field 'https_key_base_64' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#https_user: {
                        let field_value = match fields_map.get("https_user") {
                            Some(value) => value,
                            None => bail!("Missing field 'https_user' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_auth_reference: {
                        let field_value = match fields_map.get("local_auth_reference") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_auth_reference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reference_type: {
                        let field_value = match fields_map.get("reference_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'reference_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reference_value: {
                        let field_value = match fields_map.get("reference_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'reference_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_known_hosts_base_64: {
                        let field_value = match fields_map.get("ssh_known_hosts_base_64") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_known_hosts_base_64' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_private_key_base_64: {
                        let field_value = match fields_map.get("ssh_private_key_base_64") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_private_key_base_64' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_interval_in_seconds: {
                        let field_value = match fields_map.get("sync_interval_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'sync_interval_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_in_seconds: {
                        let field_value = match fields_map.get("timeout_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
