#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSecretsSecret {
    /// An optional mapping that makes up the Encryption Context for the secret.
    #[builder(into)]
    #[serde(rename = "context")]
    pub r#context: Option<std::collections::HashMap<String, String>>,
    /// The encryption algorithm that will be used to decrypt the ciphertext. This parameter is required only when the ciphertext was encrypted under an asymmetric KMS key. Valid Values: SYMMETRIC_DEFAULT | RSAES_OAEP_SHA_1 | RSAES_OAEP_SHA_256 | SM2PKE
    #[builder(into)]
    #[serde(rename = "encryptionAlgorithm")]
    pub r#encryption_algorithm: Option<String>,
    /// An optional list of Grant Tokens for the secret.
    #[builder(into)]
    #[serde(rename = "grantTokens")]
    pub r#grant_tokens: Option<Vec<String>>,
    /// Specifies the KMS key that AWS KMS uses to decrypt the ciphertext. This parameter is required only when the ciphertext was encrypted under an asymmetric KMS key.
    /// 
    /// For more information on `context` and `grant_tokens` see the [KMS
    /// Concepts](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html)
    #[builder(into)]
    #[serde(rename = "keyId")]
    pub r#key_id: Option<String>,
    /// Name to export this secret under in the attributes.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Base64 encoded payload, as returned from a KMS encrypt operation.
    #[builder(into)]
    #[serde(rename = "payload")]
    pub r#payload: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSecretsSecret {
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
                "context".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#context,
                )
                .await,
            );
            map.insert(
                "encryption_algorithm".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption_algorithm,
                )
                .await,
            );
            map.insert(
                "grant_tokens".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#grant_tokens,
                )
                .await,
            );
            map.insert(
                "key_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_id,
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
                "payload".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#payload,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSecretsSecret {
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
                    r#context: {
                        let field_value = match fields_map.get("context") {
                            Some(value) => value,
                            None => bail!("Missing field 'context' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_algorithm: {
                        let field_value = match fields_map.get("encryption_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#grant_tokens: {
                        let field_value = match fields_map.get("grant_tokens") {
                            Some(value) => value,
                            None => bail!("Missing field 'grant_tokens' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_id: {
                        let field_value = match fields_map.get("key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#payload: {
                        let field_value = match fields_map.get("payload") {
                            Some(value) => value,
                            None => bail!("Missing field 'payload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
