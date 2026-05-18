#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationGatewaySslProfile {
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The name of the SSL Profile that is unique within this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// a `ssl_policy` block as defined below.
    #[builder(into)]
    #[serde(rename = "sslPolicy")]
    pub r#ssl_policy: Option<Box<super::super::types::network::ApplicationGatewaySslProfileSslPolicy>>,
    /// The name of the Trusted Client Certificate that will be used to authenticate requests from clients.
    #[builder(into)]
    #[serde(rename = "trustedClientCertificateNames")]
    pub r#trusted_client_certificate_names: Option<Vec<String>>,
    /// Should client certificate issuer DN be verified? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "verifyClientCertIssuerDn")]
    pub r#verify_client_cert_issuer_dn: Option<bool>,
    /// Specify the method to check client certificate revocation status. Possible value is `OCSP`.
    #[builder(into)]
    #[serde(rename = "verifyClientCertificateRevocation")]
    pub r#verify_client_certificate_revocation: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationGatewaySslProfile {
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
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "ssl_policy",
                    &self.r#ssl_policy,
                ),
                to_pulumi_object_field(
                    "trusted_client_certificate_names",
                    &self.r#trusted_client_certificate_names,
                ),
                to_pulumi_object_field(
                    "verify_client_cert_issuer_dn",
                    &self.r#verify_client_cert_issuer_dn,
                ),
                to_pulumi_object_field(
                    "verify_client_certificate_revocation",
                    &self.r#verify_client_certificate_revocation,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationGatewaySslProfile {
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
                    r#ssl_policy: {
                        let field_value = match fields_map.get("ssl_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssl_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#verify_client_cert_issuer_dn: {
                        let field_value = match fields_map.get("verify_client_cert_issuer_dn") {
                            Some(value) => value,
                            None => bail!("Missing field 'verify_client_cert_issuer_dn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
