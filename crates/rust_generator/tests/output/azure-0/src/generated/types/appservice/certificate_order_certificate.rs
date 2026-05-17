#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateOrderCertificate {
    /// The name of the App Service Certificate.
    #[builder(into)]
    #[serde(rename = "certificateName")]
    pub r#certificate_name: Option<String>,
    /// Key Vault resource Id.
    #[builder(into)]
    #[serde(rename = "keyVaultId")]
    pub r#key_vault_id: Option<String>,
    /// Key Vault secret name.
    #[builder(into)]
    #[serde(rename = "keyVaultSecretName")]
    pub r#key_vault_secret_name: Option<String>,
    /// Status of the Key Vault secret.
    #[builder(into)]
    #[serde(rename = "provisioningState")]
    pub r#provisioning_state: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateOrderCertificate {
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
                "certificate_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#certificate_name,
                )
                .await,
            );
            map.insert(
                "key_vault_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_vault_id,
                )
                .await,
            );
            map.insert(
                "key_vault_secret_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_vault_secret_name,
                )
                .await,
            );
            map.insert(
                "provisioning_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#provisioning_state,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateOrderCertificate {
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
                    r#certificate_name: {
                        let field_value = match fields_map.get("certificate_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_vault_id: {
                        let field_value = match fields_map.get("key_vault_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_vault_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_vault_secret_name: {
                        let field_value = match fields_map.get("key_vault_secret_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_vault_secret_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioning_state: {
                        let field_value = match fields_map.get("provisioning_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioning_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
