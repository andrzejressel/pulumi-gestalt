#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SparkClusterDiskEncryption {
    /// This is an algorithm identifier for encryption. Possible values are `RSA1_5`, `RSA-OAEP`, `RSA-OAEP-256`.
    #[builder(into)]
    #[serde(rename = "encryptionAlgorithm")]
    pub r#encryption_algorithm: Option<String>,
    /// This is indicator to show whether resource disk encryption is enabled.
    #[builder(into)]
    #[serde(rename = "encryptionAtHostEnabled")]
    pub r#encryption_at_host_enabled: Option<bool>,
    /// The ID of the key vault key.
    #[builder(into)]
    #[serde(rename = "keyVaultKeyId")]
    pub r#key_vault_key_id: Option<String>,
    /// This is the resource ID of Managed Identity used to access the key vault.
    #[builder(into)]
    #[serde(rename = "keyVaultManagedIdentityId")]
    pub r#key_vault_managed_identity_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SparkClusterDiskEncryption {
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
                "encryption_algorithm".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption_algorithm,
                )
                .await,
            );
            map.insert(
                "encryption_at_host_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption_at_host_enabled,
                )
                .await,
            );
            map.insert(
                "key_vault_key_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_vault_key_id,
                )
                .await,
            );
            map.insert(
                "key_vault_managed_identity_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_vault_managed_identity_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SparkClusterDiskEncryption {
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
                    r#encryption_algorithm: {
                        let field_value = match fields_map.get("encryption_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_at_host_enabled: {
                        let field_value = match fields_map.get("encryption_at_host_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_at_host_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_vault_key_id: {
                        let field_value = match fields_map.get("key_vault_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_vault_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_vault_managed_identity_id: {
                        let field_value = match fields_map.get("key_vault_managed_identity_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_vault_managed_identity_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
