#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSpringCloudServiceConfigServerGitSettingRepositorySshAuth {
    /// The host key of the Git repository server.
    #[builder(into)]
    #[serde(rename = "hostKey")]
    pub r#host_key: String,
    /// The host key algorithm.
    #[builder(into)]
    #[serde(rename = "hostKeyAlgorithm")]
    pub r#host_key_algorithm: String,
    /// The SSH private key to access the Git repository, needed when the URI starts with `git@` or `ssh://`.
    #[builder(into)]
    #[serde(rename = "privateKey")]
    pub r#private_key: String,
    /// Indicates whether the Config Server instance will fail to start if the host_key does not match.
    #[builder(into)]
    #[serde(rename = "strictHostKeyCheckingEnabled")]
    pub r#strict_host_key_checking_enabled: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSpringCloudServiceConfigServerGitSettingRepositorySshAuth {
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
                "host_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_key,
                )
                .await,
            );
            map.insert(
                "host_key_algorithm".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_key_algorithm,
                )
                .await,
            );
            map.insert(
                "private_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_key,
                )
                .await,
            );
            map.insert(
                "strict_host_key_checking_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#strict_host_key_checking_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSpringCloudServiceConfigServerGitSettingRepositorySshAuth {
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
