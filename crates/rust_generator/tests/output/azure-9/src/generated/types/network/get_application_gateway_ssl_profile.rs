#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationGatewaySslProfile {
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// a `ssl_policy` block as defined below.
    #[builder(into)]
    #[serde(rename = "sslPolicies")]
    pub r#ssl_policies: Vec<super::super::types::network::GetApplicationGatewaySslProfileSslPolicy>,
    /// The name of the Trusted Client Certificate that will be used to authenticate requests from clients.
    #[builder(into)]
    #[serde(rename = "trustedClientCertificateNames")]
    pub r#trusted_client_certificate_names: Vec<String>,
    #[builder(into)]
    #[serde(rename = "verifyClientCertificateIssuerDn")]
    pub r#verify_client_certificate_issuer_dn: bool,
    /// The method used to check client certificate revocation status.
    #[builder(into)]
    #[serde(rename = "verifyClientCertificateRevocation")]
    pub r#verify_client_certificate_revocation: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetApplicationGatewaySslProfile {
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
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
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
                "ssl_policies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssl_policies,
                )
                .await,
            );
            map.insert(
                "trusted_client_certificate_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#trusted_client_certificate_names,
                )
                .await,
            );
            map.insert(
                "verify_client_certificate_issuer_dn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#verify_client_certificate_issuer_dn,
                )
                .await,
            );
            map.insert(
                "verify_client_certificate_revocation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#verify_client_certificate_revocation,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetApplicationGatewaySslProfile {
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
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#ssl_policies: {
                        let field_value = match fields_map.get("ssl_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trusted_client_certificate_names: {
                        let field_value = match fields_map.get("trusted_client_certificate_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'trusted_client_certificate_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#verify_client_certificate_issuer_dn: {
                        let field_value = match fields_map.get("verify_client_certificate_issuer_dn") {
                            Some(value) => value,
                            None => bail!("Missing field 'verify_client_certificate_issuer_dn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#verify_client_certificate_revocation: {
                        let field_value = match fields_map.get("verify_client_certificate_revocation") {
                            Some(value) => value,
                            None => bail!("Missing field 'verify_client_certificate_revocation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
