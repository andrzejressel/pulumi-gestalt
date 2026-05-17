#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataCatalogEncryptionSettingsDataCatalogEncryptionSetting {
    /// When connection password protection is enabled, the Data Catalog uses a customer-provided key to encrypt the password as part of CreateConnection or UpdateConnection and store it in the ENCRYPTED_PASSWORD field in the connection properties. You can enable catalog encryption or only password encryption. see Connection Password Encryption.
    #[builder(into)]
    #[serde(rename = "connectionPasswordEncryptions")]
    pub r#connection_password_encryptions: Vec<super::super::types::glue::GetDataCatalogEncryptionSettingsDataCatalogEncryptionSettingConnectionPasswordEncryption>,
    /// Encryption-at-rest configuration for the Data Catalog. see Encryption At Rest.
    #[builder(into)]
    #[serde(rename = "encryptionAtRests")]
    pub r#encryption_at_rests: Vec<super::super::types::glue::GetDataCatalogEncryptionSettingsDataCatalogEncryptionSettingEncryptionAtRest>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDataCatalogEncryptionSettingsDataCatalogEncryptionSetting {
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
                "connection_password_encryptions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection_password_encryptions,
                )
                .await,
            );
            map.insert(
                "encryption_at_rests".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption_at_rests,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDataCatalogEncryptionSettingsDataCatalogEncryptionSetting {
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
                    r#connection_password_encryptions: {
                        let field_value = match fields_map.get("connection_password_encryptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_password_encryptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_at_rests: {
                        let field_value = match fields_map.get("encryption_at_rests") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_at_rests' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
