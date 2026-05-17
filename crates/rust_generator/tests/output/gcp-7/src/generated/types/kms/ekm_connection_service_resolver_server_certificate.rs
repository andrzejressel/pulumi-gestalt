#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EkmConnectionServiceResolverServerCertificate {
    /// (Output)
    /// Output only. The issuer distinguished name in RFC 2253 format. Only present if parsed is true.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Option<String>,
    /// (Output)
    /// Output only. The certificate is not valid after this time. Only present if parsed is true.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "notAfterTime")]
    pub r#not_after_time: Option<String>,
    /// (Output)
    /// Output only. The certificate is not valid before this time. Only present if parsed is true.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    #[builder(into)]
    #[serde(rename = "notBeforeTime")]
    pub r#not_before_time: Option<String>,
    /// (Output)
    /// Output only. True if the certificate was parsed successfully.
    #[builder(into)]
    #[serde(rename = "parsed")]
    pub r#parsed: Option<bool>,
    /// Required. The raw certificate bytes in DER format. A base64-encoded string.
    #[builder(into)]
    #[serde(rename = "rawDer")]
    pub r#raw_der: String,
    /// (Output)
    /// Output only. The certificate serial number as a hex string. Only present if parsed is true.
    #[builder(into)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Option<String>,
    /// (Output)
    /// Output only. The SHA-256 certificate fingerprint as a hex string. Only present if parsed is true.
    #[builder(into)]
    #[serde(rename = "sha256Fingerprint")]
    pub r#sha_256_fingerprint: Option<String>,
    /// (Output)
    /// Output only. The subject distinguished name in RFC 2253 format. Only present if parsed is true.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Option<String>,
    /// (Output)
    /// Output only. The subject Alternative DNS names. Only present if parsed is true.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "subjectAlternativeDnsNames")]
    pub r#subject_alternative_dns_names: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EkmConnectionServiceResolverServerCertificate {
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
                "issuer".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#issuer,
                )
                .await,
            );
            map.insert(
                "not_after_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#not_after_time,
                )
                .await,
            );
            map.insert(
                "not_before_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#not_before_time,
                )
                .await,
            );
            map.insert(
                "parsed".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parsed,
                )
                .await,
            );
            map.insert(
                "raw_der".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#raw_der,
                )
                .await,
            );
            map.insert(
                "serial_number".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#serial_number,
                )
                .await,
            );
            map.insert(
                "sha_256_fingerprint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sha_256_fingerprint,
                )
                .await,
            );
            map.insert(
                "subject".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subject,
                )
                .await,
            );
            map.insert(
                "subject_alternative_dns_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subject_alternative_dns_names,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EkmConnectionServiceResolverServerCertificate {
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
                    r#issuer: {
                        let field_value = match fields_map.get("issuer") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#not_after_time: {
                        let field_value = match fields_map.get("not_after_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_after_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#not_before_time: {
                        let field_value = match fields_map.get("not_before_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_before_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parsed: {
                        let field_value = match fields_map.get("parsed") {
                            Some(value) => value,
                            None => bail!("Missing field 'parsed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#raw_der: {
                        let field_value = match fields_map.get("raw_der") {
                            Some(value) => value,
                            None => bail!("Missing field 'raw_der' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serial_number: {
                        let field_value = match fields_map.get("serial_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'serial_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sha_256_fingerprint: {
                        let field_value = match fields_map.get("sha_256_fingerprint") {
                            Some(value) => value,
                            None => bail!("Missing field 'sha_256_fingerprint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subject: {
                        let field_value = match fields_map.get("subject") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subject_alternative_dns_names: {
                        let field_value = match fields_map.get("subject_alternative_dns_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject_alternative_dns_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
