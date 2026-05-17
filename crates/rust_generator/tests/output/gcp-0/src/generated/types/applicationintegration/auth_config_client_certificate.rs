#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthConfigClientCertificate {
    /// The ssl certificate encoded in PEM format. This string must include the begin header and end footer lines.
    #[builder(into)]
    #[serde(rename = "encryptedPrivateKey")]
    pub r#encrypted_private_key: String,
    /// 'passphrase' should be left unset if private key is not encrypted.
    /// Note that 'passphrase' is not the password for web server, but an extra layer of security to protected private key.
    #[builder(into)]
    #[serde(rename = "passphrase")]
    pub r#passphrase: Option<String>,
    /// The ssl certificate encoded in PEM format. This string must include the begin header and end footer lines.
    #[builder(into)]
    #[serde(rename = "sslCertificate")]
    pub r#ssl_certificate: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AuthConfigClientCertificate {
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
                "encrypted_private_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encrypted_private_key,
                )
                .await,
            );
            map.insert(
                "passphrase".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#passphrase,
                )
                .await,
            );
            map.insert(
                "ssl_certificate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssl_certificate,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AuthConfigClientCertificate {
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
                    r#encrypted_private_key: {
                        let field_value = match fields_map.get("encrypted_private_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'encrypted_private_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#passphrase: {
                        let field_value = match fields_map.get("passphrase") {
                            Some(value) => value,
                            None => bail!("Missing field 'passphrase' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssl_certificate: {
                        let field_value = match fields_map.get("ssl_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
