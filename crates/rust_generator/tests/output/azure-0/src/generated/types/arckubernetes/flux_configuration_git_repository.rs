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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "httpsCaCertBase64",
                    &self.r#https_ca_cert_base_64,
                ),
                to_pulumi_object_field(
                    "httpsKeyBase64",
                    &self.r#https_key_base_64,
                ),
                to_pulumi_object_field(
                    "httpsUser",
                    &self.r#https_user,
                ),
                to_pulumi_object_field(
                    "localAuthReference",
                    &self.r#local_auth_reference,
                ),
                to_pulumi_object_field(
                    "referenceType",
                    &self.r#reference_type,
                ),
                to_pulumi_object_field(
                    "referenceValue",
                    &self.r#reference_value,
                ),
                to_pulumi_object_field(
                    "sshKnownHostsBase64",
                    &self.r#ssh_known_hosts_base_64,
                ),
                to_pulumi_object_field(
                    "sshPrivateKeyBase64",
                    &self.r#ssh_private_key_base_64,
                ),
                to_pulumi_object_field(
                    "syncIntervalInSeconds",
                    &self.r#sync_interval_in_seconds,
                ),
                to_pulumi_object_field(
                    "timeoutInSeconds",
                    &self.r#timeout_in_seconds,
                ),
                to_pulumi_object_field(
                    "url",
                    &self.r#url,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("httpsCaCertBase64") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpsCaCertBase64' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#https_key_base_64: {
                        let field_value = match fields_map.get("httpsKeyBase64") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpsKeyBase64' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#https_user: {
                        let field_value = match fields_map.get("httpsUser") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpsUser' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_auth_reference: {
                        let field_value = match fields_map.get("localAuthReference") {
                            Some(value) => value,
                            None => bail!("Missing field 'localAuthReference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reference_type: {
                        let field_value = match fields_map.get("referenceType") {
                            Some(value) => value,
                            None => bail!("Missing field 'referenceType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reference_value: {
                        let field_value = match fields_map.get("referenceValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'referenceValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_known_hosts_base_64: {
                        let field_value = match fields_map.get("sshKnownHostsBase64") {
                            Some(value) => value,
                            None => bail!("Missing field 'sshKnownHostsBase64' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_private_key_base_64: {
                        let field_value = match fields_map.get("sshPrivateKeyBase64") {
                            Some(value) => value,
                            None => bail!("Missing field 'sshPrivateKeyBase64' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sync_interval_in_seconds: {
                        let field_value = match fields_map.get("syncIntervalInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'syncIntervalInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_in_seconds: {
                        let field_value = match fields_map.get("timeoutInSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeoutInSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
