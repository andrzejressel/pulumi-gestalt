#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KeyKeyAttributesKeyModesOfUse {
    /// Whether an AWS Payment Cryptography key can be used to decrypt data.
    #[builder(into)]
    #[serde(rename = "decrypt")]
    pub r#decrypt: Option<bool>,
    /// Whether an AWS Payment Cryptography key can be used to derive new keys.
    #[builder(into)]
    #[serde(rename = "deriveKey")]
    pub r#derive_key: Option<bool>,
    /// Whether an AWS Payment Cryptography key can be used to encrypt data.
    #[builder(into)]
    #[serde(rename = "encrypt")]
    pub r#encrypt: Option<bool>,
    /// Whether an AWS Payment Cryptography key can be used to generate and verify other card and PIN verification keys.
    #[builder(into)]
    #[serde(rename = "generate")]
    pub r#generate: Option<bool>,
    /// Whether an AWS Payment Cryptography key has no special restrictions other than the restrictions implied by KeyUsage.
    #[builder(into)]
    #[serde(rename = "noRestrictions")]
    pub r#no_restrictions: Option<bool>,
    /// Whether an AWS Payment Cryptography key can be used for signing.
    #[builder(into)]
    #[serde(rename = "sign")]
    pub r#sign: Option<bool>,
    /// Whether an AWS Payment Cryptography key can be used to unwrap other keys.
    #[builder(into)]
    #[serde(rename = "unwrap")]
    pub r#unwrap: Option<bool>,
    /// Whether an AWS Payment Cryptography key can be used to verify signatures.
    #[builder(into)]
    #[serde(rename = "verify")]
    pub r#verify: Option<bool>,
    /// Whether an AWS Payment Cryptography key can be used to wrap other keys.
    #[builder(into)]
    #[serde(rename = "wrap")]
    pub r#wrap: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KeyKeyAttributesKeyModesOfUse {
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
                "decrypt".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#decrypt,
                )
                .await,
            );
            map.insert(
                "derive_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#derive_key,
                )
                .await,
            );
            map.insert(
                "encrypt".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encrypt,
                )
                .await,
            );
            map.insert(
                "generate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#generate,
                )
                .await,
            );
            map.insert(
                "no_restrictions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#no_restrictions,
                )
                .await,
            );
            map.insert(
                "sign".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sign,
                )
                .await,
            );
            map.insert(
                "unwrap".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unwrap,
                )
                .await,
            );
            map.insert(
                "verify".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#verify,
                )
                .await,
            );
            map.insert(
                "wrap".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#wrap,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KeyKeyAttributesKeyModesOfUse {
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
                    r#decrypt: {
                        let field_value = match fields_map.get("decrypt") {
                            Some(value) => value,
                            None => bail!("Missing field 'decrypt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#derive_key: {
                        let field_value = match fields_map.get("derive_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'derive_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encrypt: {
                        let field_value = match fields_map.get("encrypt") {
                            Some(value) => value,
                            None => bail!("Missing field 'encrypt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#generate: {
                        let field_value = match fields_map.get("generate") {
                            Some(value) => value,
                            None => bail!("Missing field 'generate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_restrictions: {
                        let field_value = match fields_map.get("no_restrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'no_restrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sign: {
                        let field_value = match fields_map.get("sign") {
                            Some(value) => value,
                            None => bail!("Missing field 'sign' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unwrap: {
                        let field_value = match fields_map.get("unwrap") {
                            Some(value) => value,
                            None => bail!("Missing field 'unwrap' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#verify: {
                        let field_value = match fields_map.get("verify") {
                            Some(value) => value,
                            None => bail!("Missing field 'verify' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#wrap: {
                        let field_value = match fields_map.get("wrap") {
                            Some(value) => value,
                            None => bail!("Missing field 'wrap' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
