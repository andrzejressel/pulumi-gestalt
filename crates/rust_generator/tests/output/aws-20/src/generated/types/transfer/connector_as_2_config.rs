#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorAs2Config {
    /// Specifies weather AS2 file is compressed. The valud values are ZLIB and  DISABLED.
    #[builder(into)]
    #[serde(rename = "compression")]
    pub r#compression: String,
    /// The algorithm that is used to encrypt the file. The valid values are AES128_CBC | AES192_CBC | AES256_CBC | NONE.
    #[builder(into)]
    #[serde(rename = "encryptionAlgorithm")]
    pub r#encryption_algorithm: String,
    /// The unique identifier for the AS2 local profile.
    #[builder(into)]
    #[serde(rename = "localProfileId")]
    pub r#local_profile_id: String,
    /// Used for outbound requests to determine if a partner response for transfers is synchronous or asynchronous. The valid values are SYNC and NONE.
    #[builder(into)]
    #[serde(rename = "mdnResponse")]
    pub r#mdn_response: String,
    /// The signing algorithm for the Mdn response. The valid values are SHA256 | SHA384 | SHA512 | SHA1 | NONE | DEFAULT.
    #[builder(into)]
    #[serde(rename = "mdnSigningAlgorithm")]
    pub r#mdn_signing_algorithm: Option<String>,
    /// Used as the subject HTTP header attribute in AS2 messages that are being sent with the connector.
    #[builder(into)]
    #[serde(rename = "messageSubject")]
    pub r#message_subject: Option<String>,
    /// The unique identifier for the AS2 partner profile.
    #[builder(into)]
    #[serde(rename = "partnerProfileId")]
    pub r#partner_profile_id: String,
    /// The algorithm that is used to sign AS2 messages sent with the connector. The valid values are SHA256 | SHA384 | SHA512 | SHA1 | NONE .
    #[builder(into)]
    #[serde(rename = "signingAlgorithm")]
    pub r#signing_algorithm: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectorAs2Config {
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
                "compression".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#compression,
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
                "local_profile_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_profile_id,
                )
                .await,
            );
            map.insert(
                "mdn_response".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mdn_response,
                )
                .await,
            );
            map.insert(
                "mdn_signing_algorithm".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mdn_signing_algorithm,
                )
                .await,
            );
            map.insert(
                "message_subject".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#message_subject,
                )
                .await,
            );
            map.insert(
                "partner_profile_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#partner_profile_id,
                )
                .await,
            );
            map.insert(
                "signing_algorithm".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#signing_algorithm,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectorAs2Config {
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
                    r#compression: {
                        let field_value = match fields_map.get("compression") {
                            Some(value) => value,
                            None => bail!("Missing field 'compression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#local_profile_id: {
                        let field_value = match fields_map.get("local_profile_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_profile_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mdn_response: {
                        let field_value = match fields_map.get("mdn_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'mdn_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mdn_signing_algorithm: {
                        let field_value = match fields_map.get("mdn_signing_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'mdn_signing_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#message_subject: {
                        let field_value = match fields_map.get("message_subject") {
                            Some(value) => value,
                            None => bail!("Missing field 'message_subject' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#partner_profile_id: {
                        let field_value = match fields_map.get("partner_profile_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'partner_profile_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signing_algorithm: {
                        let field_value = match fields_map.get("signing_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'signing_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
