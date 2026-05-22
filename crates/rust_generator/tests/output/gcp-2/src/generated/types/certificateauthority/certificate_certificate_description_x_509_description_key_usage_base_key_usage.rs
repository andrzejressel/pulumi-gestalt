#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateCertificateDescriptionX509DescriptionKeyUsageBaseKeyUsage {
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateCertificateDescriptionX509DescriptionKeyUsageBaseKeyUsage {
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
                    "certSign",
                    &self.r#cert_sign,
                ),
                to_pulumi_object_field(
                    "contentCommitment",
                    &self.r#content_commitment,
                ),
                to_pulumi_object_field(
                    "crlSign",
                    &self.r#crl_sign,
                ),
                to_pulumi_object_field(
                    "dataEncipherment",
                    &self.r#data_encipherment,
                ),
                to_pulumi_object_field(
                    "decipherOnly",
                    &self.r#decipher_only,
                ),
                to_pulumi_object_field(
                    "digitalSignature",
                    &self.r#digital_signature,
                ),
                to_pulumi_object_field(
                    "encipherOnly",
                    &self.r#encipher_only,
                ),
                to_pulumi_object_field(
                    "keyAgreement",
                    &self.r#key_agreement,
                ),
                to_pulumi_object_field(
                    "keyEncipherment",
                    &self.r#key_encipherment,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateCertificateDescriptionX509DescriptionKeyUsageBaseKeyUsage {
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
                        let field_value = match fields_map.get("certSign") {
                            Some(value) => value,
                            None => bail!("Missing field 'certSign' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#content_commitment: {
                        let field_value = match fields_map.get("contentCommitment") {
                            Some(value) => value,
                            None => bail!("Missing field 'contentCommitment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#crl_sign: {
                        let field_value = match fields_map.get("crlSign") {
                            Some(value) => value,
                            None => bail!("Missing field 'crlSign' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_encipherment: {
                        let field_value = match fields_map.get("dataEncipherment") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataEncipherment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#decipher_only: {
                        let field_value = match fields_map.get("decipherOnly") {
                            Some(value) => value,
                            None => bail!("Missing field 'decipherOnly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#digital_signature: {
                        let field_value = match fields_map.get("digitalSignature") {
                            Some(value) => value,
                            None => bail!("Missing field 'digitalSignature' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encipher_only: {
                        let field_value = match fields_map.get("encipherOnly") {
                            Some(value) => value,
                            None => bail!("Missing field 'encipherOnly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_agreement: {
                        let field_value = match fields_map.get("keyAgreement") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyAgreement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_encipherment: {
                        let field_value = match fields_map.get("keyEncipherment") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyEncipherment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
