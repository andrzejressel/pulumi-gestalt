#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCertificateAuthorityRevocationConfiguration {
    /// Nested attribute containing configuration of the certificate revocation list (CRL), if any, maintained by the certificate authority.
    #[builder(into)]
    #[serde(rename = "crlConfigurations")]
    pub r#crl_configurations: Vec<super::super::types::acmpca::GetCertificateAuthorityRevocationConfigurationCrlConfiguration>,
    #[builder(into)]
    #[serde(rename = "ocspConfigurations")]
    pub r#ocsp_configurations: Vec<super::super::types::acmpca::GetCertificateAuthorityRevocationConfigurationOcspConfiguration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCertificateAuthorityRevocationConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("crl_configurations".to_string(), self.r#crl_configurations.to_pulumi_value().await);
            map.insert("ocsp_configurations".to_string(), self.r#ocsp_configurations.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCertificateAuthorityRevocationConfiguration {
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
                    r#crl_configurations: {
                        let field_value = match fields_map.get("crl_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'crl_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::acmpca::GetCertificateAuthorityRevocationConfigurationCrlConfiguration> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ocsp_configurations: {
                        let field_value = match fields_map.get("ocsp_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'ocsp_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::acmpca::GetCertificateAuthorityRevocationConfigurationOcspConfiguration> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
