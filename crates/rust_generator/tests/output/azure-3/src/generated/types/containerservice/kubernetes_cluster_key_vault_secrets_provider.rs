#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterKeyVaultSecretsProvider {
    /// An `secret_identity` block is exported. The exported attributes are defined below.
    #[builder(into)]
    #[serde(rename = "secretIdentities")]
    pub r#secret_identities: Option<Vec<super::super::types::containerservice::KubernetesClusterKeyVaultSecretsProviderSecretIdentity>>,
    /// Should the secret store CSI driver on the AKS cluster be enabled?
    #[builder(into)]
    #[serde(rename = "secretRotationEnabled")]
    pub r#secret_rotation_enabled: Option<bool>,
    /// The interval to poll for secret rotation. This attribute is only set when `secret_rotation` is true. Defaults to `2m`.
    /// 
    /// > **Note:** To enable`key_vault_secrets_provider` either `secret_rotation_enabled` or `secret_rotation_interval` must be specified.
    #[builder(into)]
    #[serde(rename = "secretRotationInterval")]
    pub r#secret_rotation_interval: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterKeyVaultSecretsProvider {
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
                "secret_identities".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret_identities,
                )
                .await,
            );
            map.insert(
                "secret_rotation_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret_rotation_enabled,
                )
                .await,
            );
            map.insert(
                "secret_rotation_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret_rotation_interval,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterKeyVaultSecretsProvider {
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
                    r#secret_identities: {
                        let field_value = match fields_map.get("secret_identities") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_identities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_rotation_enabled: {
                        let field_value = match fields_map.get("secret_rotation_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_rotation_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_rotation_interval: {
                        let field_value = match fields_map.get("secret_rotation_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_rotation_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
