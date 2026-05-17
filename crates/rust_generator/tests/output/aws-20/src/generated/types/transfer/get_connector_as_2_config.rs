#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetConnectorAs2Config {
    /// Basic authentication for AS2 connector API. Returns a null value if not set.
    #[builder(into)]
    #[serde(rename = "basicAuthSecretId")]
    pub r#basic_auth_secret_id: String,
    /// Specifies whether AS2 file is compressed. Will be ZLIB or DISABLED
    #[builder(into)]
    #[serde(rename = "compression")]
    pub r#compression: String,
    /// Algorithm used to encrypt file. Will be AES128_CBC or AES192_CBC or AES256_CBC or DES_EDE3_CBC or NONE.
    #[builder(into)]
    #[serde(rename = "encryptionAlgorithm")]
    pub r#encryption_algorithm: String,
    /// Unique identifier for AS2 local profile.
    #[builder(into)]
    #[serde(rename = "localProfileId")]
    pub r#local_profile_id: String,
    /// Used for outbound requests to tell if response is asynchronous or not. Will be either SYNC or NONE.
    #[builder(into)]
    #[serde(rename = "mdnResponse")]
    pub r#mdn_response: String,
    /// Signing algorithm for MDN response. Will be SHA256 or SHA384 or SHA512 or SHA1 or NONE or DEFAULT.
    #[builder(into)]
    #[serde(rename = "mdnSigningAlgorithm")]
    pub r#mdn_signing_algorithm: String,
    /// Subject HTTP header attribute in outbound AS2 messages to the connector.
    #[builder(into)]
    #[serde(rename = "messageSubject")]
    pub r#message_subject: String,
    /// Unique identifier used by connector for partner profile.
    #[builder(into)]
    #[serde(rename = "partnerProfileId")]
    pub r#partner_profile_id: String,
    #[builder(into)]
    #[serde(rename = "singingAlgorithm")]
    pub r#singing_algorithm: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetConnectorAs2Config {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "basic_auth_secret_id",
                    &self.r#basic_auth_secret_id,
                ),
                to_pulumi_object_field(
                    "compression",
                    &self.r#compression,
                ),
                to_pulumi_object_field(
                    "encryption_algorithm",
                    &self.r#encryption_algorithm,
                ),
                to_pulumi_object_field(
                    "local_profile_id",
                    &self.r#local_profile_id,
                ),
                to_pulumi_object_field(
                    "mdn_response",
                    &self.r#mdn_response,
                ),
                to_pulumi_object_field(
                    "mdn_signing_algorithm",
                    &self.r#mdn_signing_algorithm,
                ),
                to_pulumi_object_field(
                    "message_subject",
                    &self.r#message_subject,
                ),
                to_pulumi_object_field(
                    "partner_profile_id",
                    &self.r#partner_profile_id,
                ),
                to_pulumi_object_field(
                    "singing_algorithm",
                    &self.r#singing_algorithm,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetConnectorAs2Config {
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
                    r#basic_auth_secret_id: {
                        let field_value = match fields_map.get("basic_auth_secret_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'basic_auth_secret_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
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
                    r#singing_algorithm: {
                        let field_value = match fields_map.get("singing_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'singing_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
