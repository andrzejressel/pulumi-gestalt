#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineKeyVaultCredential {
    /// The Azure Key Vault url. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "keyVaultUrl")]
    pub r#key_vault_url: String,
    /// The credential name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The service principal name to access key vault. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "servicePrincipalName")]
    pub r#service_principal_name: String,
    /// The service principal name secret to access key vault. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "servicePrincipalSecret")]
    pub r#service_principal_secret: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineKeyVaultCredential {
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
                "key_vault_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_vault_url,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "service_principal_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_principal_name,
                )
                .await,
            );
            map.insert(
                "service_principal_secret".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_principal_secret,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualMachineKeyVaultCredential {
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
                    r#key_vault_url: {
                        let field_value = match fields_map.get("key_vault_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_vault_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_principal_name: {
                        let field_value = match fields_map.get("service_principal_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_principal_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_principal_secret: {
                        let field_value = match fields_map.get("service_principal_secret") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_principal_secret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
