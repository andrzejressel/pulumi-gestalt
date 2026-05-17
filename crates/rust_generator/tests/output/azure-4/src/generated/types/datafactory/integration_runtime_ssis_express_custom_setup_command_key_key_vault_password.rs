#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IntegrationRuntimeSsisExpressCustomSetupCommandKeyKeyVaultPassword {
    #[builder(into)]
    #[serde(rename = "linkedServiceName")]
    pub r#linked_service_name: String,
    /// A map of parameters to associate with the Key Vault Data Factory Linked Service.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<std::collections::HashMap<String, String>>,
    /// Specifies the secret name in Azure Key Vault.
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: String,
    /// Specifies the secret version in Azure Key Vault.
    #[builder(into)]
    #[serde(rename = "secretVersion")]
    pub r#secret_version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IntegrationRuntimeSsisExpressCustomSetupCommandKeyKeyVaultPassword {
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
                "linked_service_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#linked_service_name,
                )
                .await,
            );
            map.insert(
                "parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parameters,
                )
                .await,
            );
            map.insert(
                "secret_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret_name,
                )
                .await,
            );
            map.insert(
                "secret_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret_version,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IntegrationRuntimeSsisExpressCustomSetupCommandKeyKeyVaultPassword {
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
                    r#linked_service_name: {
                        let field_value = match fields_map.get("linked_service_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'linked_service_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parameters: {
                        let field_value = match fields_map.get("parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_name: {
                        let field_value = match fields_map.get("secret_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_version: {
                        let field_value = match fields_map.get("secret_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
