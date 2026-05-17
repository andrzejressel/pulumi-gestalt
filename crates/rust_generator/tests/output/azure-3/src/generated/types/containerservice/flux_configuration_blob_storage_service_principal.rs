#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FluxConfigurationBlobStorageServicePrincipal {
    /// Base64-encoded certificate used to authenticate a Service Principal .
    #[builder(into)]
    #[serde(rename = "clientCertificateBase64")]
    pub r#client_certificate_base_64: Option<String>,
    /// Specifies the password for the certificate used to authenticate a Service Principal .
    #[builder(into)]
    #[serde(rename = "clientCertificatePassword")]
    pub r#client_certificate_password: Option<String>,
    /// Specifies whether to include x5c header in client claims when acquiring a token to enable subject name / issuer based authentication for the client certificate.
    #[builder(into)]
    #[serde(rename = "clientCertificateSendChain")]
    pub r#client_certificate_send_chain: Option<bool>,
    /// Specifies the client ID for authenticating a Service Principal.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// Specifies the client secret for authenticating a Service Principal.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    /// Specifies the tenant ID for authenticating a Service Principal.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FluxConfigurationBlobStorageServicePrincipal {
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
                "client_certificate_base_64".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_certificate_base_64,
                )
                .await,
            );
            map.insert(
                "client_certificate_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_certificate_password,
                )
                .await,
            );
            map.insert(
                "client_certificate_send_chain".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_certificate_send_chain,
                )
                .await,
            );
            map.insert(
                "client_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_id,
                )
                .await,
            );
            map.insert(
                "client_secret".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_secret,
                )
                .await,
            );
            map.insert(
                "tenant_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tenant_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FluxConfigurationBlobStorageServicePrincipal {
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
                    r#client_certificate_base_64: {
                        let field_value = match fields_map.get("client_certificate_base_64") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_certificate_base_64' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_certificate_password: {
                        let field_value = match fields_map.get("client_certificate_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_certificate_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_certificate_send_chain: {
                        let field_value = match fields_map.get("client_certificate_send_chain") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_certificate_send_chain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_id: {
                        let field_value = match fields_map.get("client_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_secret: {
                        let field_value = match fields_map.get("client_secret") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_secret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tenant_id: {
                        let field_value = match fields_map.get("tenant_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'tenant_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
