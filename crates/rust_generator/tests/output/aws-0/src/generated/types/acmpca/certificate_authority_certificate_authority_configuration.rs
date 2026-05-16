#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateAuthorityCertificateAuthorityConfiguration {
    /// Type of the public key algorithm and size, in bits, of the key pair that your key pair creates when it issues a certificate. Valid values can be found in the [ACM PCA Documentation](https://docs.aws.amazon.com/privateca/latest/APIReference/API_CertificateAuthorityConfiguration.html).
    #[builder(into)]
    #[serde(rename = "keyAlgorithm")]
    pub r#key_algorithm: String,
    /// Name of the algorithm your private CA uses to sign certificate requests. Valid values can be found in the [ACM PCA Documentation](https://docs.aws.amazon.com/privateca/latest/APIReference/API_CertificateAuthorityConfiguration.html).
    #[builder(into)]
    #[serde(rename = "signingAlgorithm")]
    pub r#signing_algorithm: String,
    /// Nested argument that contains X.500 distinguished name information. At least one nested attribute must be specified.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Box<super::super::types::acmpca::CertificateAuthorityCertificateAuthorityConfigurationSubject>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateAuthorityCertificateAuthorityConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("key_algorithm".to_string(), self.r#key_algorithm.to_pulumi_value().await);
            map.insert("signing_algorithm".to_string(), self.r#signing_algorithm.to_pulumi_value().await);
            map.insert("subject".to_string(), self.r#subject.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateAuthorityCertificateAuthorityConfiguration {
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
                    r#key_algorithm: {
                        let field_value = match fields_map.get("key_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#signing_algorithm: {
                        let field_value = match fields_map.get("signing_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'signing_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#subject: {
                        let field_value = match fields_map.get("subject") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::acmpca::CertificateAuthorityCertificateAuthorityConfigurationSubject> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
