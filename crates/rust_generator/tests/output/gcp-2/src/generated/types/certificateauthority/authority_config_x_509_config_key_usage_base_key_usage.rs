#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthorityConfigX509ConfigKeyUsageBaseKeyUsage {
    /// The key may be used to sign certificates.
    #[builder(into)]
    #[serde(rename = "certSign")]
    pub r#cert_sign: Option<bool>,
    /// The key may be used for cryptographic commitments. Note that this may also be referred to as "non-repudiation".
    #[builder(into)]
    #[serde(rename = "contentCommitment")]
    pub r#content_commitment: Option<bool>,
    /// The key may be used sign certificate revocation lists.
    #[builder(into)]
    #[serde(rename = "crlSign")]
    pub r#crl_sign: Option<bool>,
    /// The key may be used to encipher data.
    #[builder(into)]
    #[serde(rename = "dataEncipherment")]
    pub r#data_encipherment: Option<bool>,
    /// The key may be used to decipher only.
    #[builder(into)]
    #[serde(rename = "decipherOnly")]
    pub r#decipher_only: Option<bool>,
    /// The key may be used for digital signatures.
    #[builder(into)]
    #[serde(rename = "digitalSignature")]
    pub r#digital_signature: Option<bool>,
    /// The key may be used to encipher only.
    #[builder(into)]
    #[serde(rename = "encipherOnly")]
    pub r#encipher_only: Option<bool>,
    /// The key may be used in a key agreement protocol.
    #[builder(into)]
    #[serde(rename = "keyAgreement")]
    pub r#key_agreement: Option<bool>,
    /// The key may be used to encipher other keys.
    #[builder(into)]
    #[serde(rename = "keyEncipherment")]
    pub r#key_encipherment: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AuthorityConfigX509ConfigKeyUsageBaseKeyUsage {
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
                "cert_sign".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cert_sign,
                )
                .await,
            );
            map.insert(
                "content_commitment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#content_commitment,
                )
                .await,
            );
            map.insert(
                "crl_sign".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#crl_sign,
                )
                .await,
            );
            map.insert(
                "data_encipherment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_encipherment,
                )
                .await,
            );
            map.insert(
                "decipher_only".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#decipher_only,
                )
                .await,
            );
            map.insert(
                "digital_signature".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#digital_signature,
                )
                .await,
            );
            map.insert(
                "encipher_only".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encipher_only,
                )
                .await,
            );
            map.insert(
                "key_agreement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_agreement,
                )
                .await,
            );
            map.insert(
                "key_encipherment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_encipherment,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AuthorityConfigX509ConfigKeyUsageBaseKeyUsage {
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
                    r#cert_sign: {
                        let field_value = match fields_map.get("cert_sign") {
                            Some(value) => value,
                            None => bail!("Missing field 'cert_sign' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#content_commitment: {
                        let field_value = match fields_map.get("content_commitment") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_commitment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#crl_sign: {
                        let field_value = match fields_map.get("crl_sign") {
                            Some(value) => value,
                            None => bail!("Missing field 'crl_sign' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_encipherment: {
                        let field_value = match fields_map.get("data_encipherment") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_encipherment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#decipher_only: {
                        let field_value = match fields_map.get("decipher_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'decipher_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#digital_signature: {
                        let field_value = match fields_map.get("digital_signature") {
                            Some(value) => value,
                            None => bail!("Missing field 'digital_signature' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encipher_only: {
                        let field_value = match fields_map.get("encipher_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'encipher_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_agreement: {
                        let field_value = match fields_map.get("key_agreement") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_agreement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_encipherment: {
                        let field_value = match fields_map.get("key_encipherment") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_encipherment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
