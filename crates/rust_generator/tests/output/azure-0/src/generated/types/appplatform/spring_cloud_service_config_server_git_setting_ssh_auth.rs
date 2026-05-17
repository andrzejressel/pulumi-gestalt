#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudServiceConfigServerGitSettingSshAuth {
    /// The host key of the Git repository server, should not include the algorithm prefix as covered by `host-key-algorithm`.
    #[builder(into)]
    #[serde(rename = "hostKey")]
    pub r#host_key: Option<String>,
    /// The host key algorithm, should be `ssh-dss`, `ssh-rsa`, `ecdsa-sha2-nistp256`, `ecdsa-sha2-nistp384`, or `ecdsa-sha2-nistp521`. Required only if `host-key` exists.
    #[builder(into)]
    #[serde(rename = "hostKeyAlgorithm")]
    pub r#host_key_algorithm: Option<String>,
    /// The SSH private key to access the Git repository, required when the URI starts with `git@` or `ssh://`.
    #[builder(into)]
    #[serde(rename = "privateKey")]
    pub r#private_key: String,
    /// Indicates whether the Config Server instance will fail to start if the host_key does not match. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "strictHostKeyCheckingEnabled")]
    pub r#strict_host_key_checking_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpringCloudServiceConfigServerGitSettingSshAuth {
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
                    "host_key",
                    &self.r#host_key,
                ),
                to_pulumi_object_field(
                    "host_key_algorithm",
                    &self.r#host_key_algorithm,
                ),
                to_pulumi_object_field(
                    "private_key",
                    &self.r#private_key,
                ),
                to_pulumi_object_field(
                    "strict_host_key_checking_enabled",
                    &self.r#strict_host_key_checking_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpringCloudServiceConfigServerGitSettingSshAuth {
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
                    r#host_key: {
                        let field_value = match fields_map.get("host_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_key_algorithm: {
                        let field_value = match fields_map.get("host_key_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_key_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_key: {
                        let field_value = match fields_map.get("private_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#strict_host_key_checking_enabled: {
                        let field_value = match fields_map.get("strict_host_key_checking_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'strict_host_key_checking_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
