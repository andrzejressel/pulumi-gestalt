#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthorityConfigX509ConfigKeyUsageExtendedKeyUsage {
    /// Corresponds to OID 1.3.6.1.5.5.7.3.2. Officially described as "TLS WWW client authentication", though regularly used for non-WWW TLS.
    #[builder(into)]
    #[serde(rename = "clientAuth")]
    pub r#client_auth: Option<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.3. Officially described as "Signing of downloadable executable code client authentication".
    #[builder(into)]
    #[serde(rename = "codeSigning")]
    pub r#code_signing: Option<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.4. Officially described as "Email protection".
    #[builder(into)]
    #[serde(rename = "emailProtection")]
    pub r#email_protection: Option<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.9. Officially described as "Signing OCSP responses".
    #[builder(into)]
    #[serde(rename = "ocspSigning")]
    pub r#ocsp_signing: Option<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.1. Officially described as "TLS WWW server authentication", though regularly used for non-WWW TLS.
    #[builder(into)]
    #[serde(rename = "serverAuth")]
    pub r#server_auth: Option<bool>,
    /// Corresponds to OID 1.3.6.1.5.5.7.3.8. Officially described as "Binding the hash of an object to a time".
    #[builder(into)]
    #[serde(rename = "timeStamping")]
    pub r#time_stamping: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AuthorityConfigX509ConfigKeyUsageExtendedKeyUsage {
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
                "client_auth".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_auth,
                )
                .await,
            );
            map.insert(
                "code_signing".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#code_signing,
                )
                .await,
            );
            map.insert(
                "email_protection".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_protection,
                )
                .await,
            );
            map.insert(
                "ocsp_signing".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ocsp_signing,
                )
                .await,
            );
            map.insert(
                "server_auth".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#server_auth,
                )
                .await,
            );
            map.insert(
                "time_stamping".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_stamping,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AuthorityConfigX509ConfigKeyUsageExtendedKeyUsage {
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
                    r#client_auth: {
                        let field_value = match fields_map.get("client_auth") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_auth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#code_signing: {
                        let field_value = match fields_map.get("code_signing") {
                            Some(value) => value,
                            None => bail!("Missing field 'code_signing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_protection: {
                        let field_value = match fields_map.get("email_protection") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_protection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ocsp_signing: {
                        let field_value = match fields_map.get("ocsp_signing") {
                            Some(value) => value,
                            None => bail!("Missing field 'ocsp_signing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_auth: {
                        let field_value = match fields_map.get("server_auth") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_auth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_stamping: {
                        let field_value = match fields_map.get("time_stamping") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_stamping' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
