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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "compression",
                    &self.r#compression,
                ),
                to_pulumi_object_field(
                    "encryptionAlgorithm",
                    &self.r#encryption_algorithm,
                ),
                to_pulumi_object_field(
                    "localProfileId",
                    &self.r#local_profile_id,
                ),
                to_pulumi_object_field(
                    "mdnResponse",
                    &self.r#mdn_response,
                ),
                to_pulumi_object_field(
                    "mdnSigningAlgorithm",
                    &self.r#mdn_signing_algorithm,
                ),
                to_pulumi_object_field(
                    "messageSubject",
                    &self.r#message_subject,
                ),
                to_pulumi_object_field(
                    "partnerProfileId",
                    &self.r#partner_profile_id,
                ),
                to_pulumi_object_field(
                    "signingAlgorithm",
                    &self.r#signing_algorithm,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("encryptionAlgorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryptionAlgorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_profile_id: {
                        let field_value = match fields_map.get("localProfileId") {
                            Some(value) => value,
                            None => bail!("Missing field 'localProfileId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mdn_response: {
                        let field_value = match fields_map.get("mdnResponse") {
                            Some(value) => value,
                            None => bail!("Missing field 'mdnResponse' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mdn_signing_algorithm: {
                        let field_value = match fields_map.get("mdnSigningAlgorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'mdnSigningAlgorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#message_subject: {
                        let field_value = match fields_map.get("messageSubject") {
                            Some(value) => value,
                            None => bail!("Missing field 'messageSubject' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#partner_profile_id: {
                        let field_value = match fields_map.get("partnerProfileId") {
                            Some(value) => value,
                            None => bail!("Missing field 'partnerProfileId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#signing_algorithm: {
                        let field_value = match fields_map.get("signingAlgorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'signingAlgorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
