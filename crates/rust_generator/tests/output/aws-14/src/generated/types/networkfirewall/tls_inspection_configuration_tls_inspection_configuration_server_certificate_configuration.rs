#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfiguration {
    /// ARN of the imported certificate authority (CA) certificate within Certificate Manager (ACM) to use for outbound SSL/TLS inspection. See [Using SSL/TLS certificates with TLS inspection configurations](https://docs.aws.amazon.com/network-firewall/latest/developerguide/tls-inspection-certificate-requirements.html) for limitations on CA certificates.
    #[builder(into)]
    #[serde(rename = "certificateAuthorityArn")]
    pub r#certificate_authority_arn: Option<String>,
    /// Check Certificate Revocation Status block. Detailed below.
    #[builder(into)]
    #[serde(rename = "checkCertificateRevocationStatus")]
    pub r#check_certificate_revocation_status: Option<Box<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationCheckCertificateRevocationStatus>>,
    /// Scope block. Detailed below.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScope>>,
    /// Server certificates to use for inbound SSL/TLS inspection. See [Using SSL/TLS certificates with TLS inspection configurations](https://docs.aws.amazon.com/network-firewall/latest/developerguide/tls-inspection-certificate-requirements.html).
    #[builder(into)]
    #[serde(rename = "serverCertificates")]
    pub r#server_certificates: Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationServerCertificate>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("certificate_authority_arn".to_string(), self.r#certificate_authority_arn.to_pulumi_value().await);
            map.insert("check_certificate_revocation_status".to_string(), self.r#check_certificate_revocation_status.to_pulumi_value().await);
            map.insert("scopes".to_string(), self.r#scopes.to_pulumi_value().await);
            map.insert("server_certificates".to_string(), self.r#server_certificates.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#certificate_authority_arn: {
                        let field_value = match fields_map.get("certificate_authority_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_authority_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#check_certificate_revocation_status: {
                        let field_value = match fields_map.get("check_certificate_revocation_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'check_certificate_revocation_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationCheckCertificateRevocationStatus>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#scopes: {
                        let field_value = match fields_map.get("scopes") {
                            Some(value) => value,
                            None => bail!("Missing field 'scopes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationScope>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#server_certificates: {
                        let field_value = match fields_map.get("server_certificates") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_certificates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::networkfirewall::TlsInspectionConfigurationTlsInspectionConfigurationServerCertificateConfigurationServerCertificate>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
