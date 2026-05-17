#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KeystoresAliasesPkcs12CertsInfoCertInfo {
    /// (Output)
    /// X.509 basic constraints extension.
    #[builder(into)]
    #[serde(rename = "basicConstraints")]
    pub r#basic_constraints: Option<String>,
    /// (Output)
    /// X.509 notAfter validity period in milliseconds since epoch.
    #[builder(into)]
    #[serde(rename = "expiryDate")]
    pub r#expiry_date: Option<String>,
    /// (Output)
    /// Flag that specifies whether the certificate is valid.
    /// Flag is set to Yes if the certificate is valid, No if expired, or Not yet if not yet valid.
    #[builder(into)]
    #[serde(rename = "isValid")]
    pub r#is_valid: Option<String>,
    /// (Output)
    /// X.509 issuer.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Option<String>,
    /// (Output)
    /// Public key component of the X.509 subject public key info.
    #[builder(into)]
    #[serde(rename = "publicKey")]
    pub r#public_key: Option<String>,
    /// (Output)
    /// X.509 serial number.
    #[builder(into)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Option<String>,
    /// (Output)
    /// X.509 signatureAlgorithm.
    #[builder(into)]
    #[serde(rename = "sigAlgName")]
    pub r#sig_alg_name: Option<String>,
    /// (Output)
    /// X.509 subject.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Option<String>,
    /// (Output)
    /// X.509 subject alternative names (SANs) extension.
    #[builder(into)]
    #[serde(rename = "subjectAlternativeNames")]
    pub r#subject_alternative_names: Option<Vec<String>>,
    /// (Output)
    /// X.509 notBefore validity period in milliseconds since epoch.
    #[builder(into)]
    #[serde(rename = "validFrom")]
    pub r#valid_from: Option<String>,
    /// (Output)
    /// X.509 version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KeystoresAliasesPkcs12CertsInfoCertInfo {
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
                "basic_constraints".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#basic_constraints,
                )
                .await,
            );
            map.insert(
                "expiry_date".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expiry_date,
                )
                .await,
            );
            map.insert(
                "is_valid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_valid,
                )
                .await,
            );
            map.insert(
                "issuer".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#issuer,
                )
                .await,
            );
            map.insert(
                "public_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_key,
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
                "sig_alg_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sig_alg_name,
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
                "subject_alternative_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subject_alternative_names,
                )
                .await,
            );
            map.insert(
                "valid_from".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#valid_from,
                )
                .await,
            );
            map.insert(
                "version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#version,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KeystoresAliasesPkcs12CertsInfoCertInfo {
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
                    r#basic_constraints: {
                        let field_value = match fields_map.get("basic_constraints") {
                            Some(value) => value,
                            None => bail!("Missing field 'basic_constraints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expiry_date: {
                        let field_value = match fields_map.get("expiry_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'expiry_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_valid: {
                        let field_value = match fields_map.get("is_valid") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_valid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#issuer: {
                        let field_value = match fields_map.get("issuer") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_key: {
                        let field_value = match fields_map.get("public_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#sig_alg_name: {
                        let field_value = match fields_map.get("sig_alg_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'sig_alg_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#subject_alternative_names: {
                        let field_value = match fields_map.get("subject_alternative_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject_alternative_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#valid_from: {
                        let field_value = match fields_map.get("valid_from") {
                            Some(value) => value,
                            None => bail!("Missing field 'valid_from' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
