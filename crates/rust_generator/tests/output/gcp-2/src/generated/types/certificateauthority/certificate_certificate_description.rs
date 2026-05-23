#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateCertificateDescription {
    /// (Output)
    /// Describes lists of issuer CA certificate URLs that appear in the "Authority Information Access" extension in the certificate.
    #[builder(into)]
    pub r#aia_issuing_certificate_urls: Option<Vec<String>>,
    /// (Output)
    /// Identifies the subjectKeyId of the parent certificate, per https://tools.ietf.org/html/rfc5280#section-4.2.1.1
    /// Structure is documented below.
    #[builder(into)]
    pub r#authority_key_ids: Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionAuthorityKeyId>>,
    /// (Output)
    /// The hash of the x.509 certificate.
    /// Structure is documented below.
    #[builder(into)]
    pub r#cert_fingerprints: Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionCertFingerprint>>,
    /// (Output)
    /// Describes a list of locations to obtain CRL information, i.e. the DistributionPoint.fullName described by https://tools.ietf.org/html/rfc5280#section-4.2.1.13
    #[builder(into)]
    pub r#crl_distribution_points: Option<Vec<String>>,
    /// (Output)
    /// A PublicKey describes a public key.
    /// Structure is documented below.
    #[builder(into)]
    pub r#public_keys: Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionPublicKey>>,
    /// (Output)
    /// Describes some of the values in a certificate that are related to the subject and lifetime.
    /// Structure is documented below.
    #[builder(into)]
    pub r#subject_descriptions: Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionSubjectDescription>>,
    /// (Output)
    /// Provides a means of identifiying certificates that contain a particular public key, per https://tools.ietf.org/html/rfc5280#section-4.2.1.2.
    /// Structure is documented below.
    #[builder(into)]
    pub r#subject_key_ids: Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionSubjectKeyId>>,
    /// (Output)
    /// A structured description of the issued X.509 certificate.
    /// Structure is documented below.
    #[builder(into)]
    pub r#x_509_descriptions: Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionX509Description>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateCertificateDescription {
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
                    "aiaIssuingCertificateUrls",
                    &self.r#aia_issuing_certificate_urls,
                ),
                to_pulumi_object_field(
                    "authorityKeyIds",
                    &self.r#authority_key_ids,
                ),
                to_pulumi_object_field(
                    "certFingerprints",
                    &self.r#cert_fingerprints,
                ),
                to_pulumi_object_field(
                    "crlDistributionPoints",
                    &self.r#crl_distribution_points,
                ),
                to_pulumi_object_field(
                    "publicKeys",
                    &self.r#public_keys,
                ),
                to_pulumi_object_field(
                    "subjectDescriptions",
                    &self.r#subject_descriptions,
                ),
                to_pulumi_object_field(
                    "subjectKeyIds",
                    &self.r#subject_key_ids,
                ),
                to_pulumi_object_field(
                    "x509Descriptions",
                    &self.r#x_509_descriptions,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateCertificateDescription {
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
                    r#aia_issuing_certificate_urls: {
                        let field_value = match fields_map.get("aiaIssuingCertificateUrls") {
                            Some(value) => value,
                            None => bail!("Missing field 'aiaIssuingCertificateUrls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authority_key_ids: {
                        let field_value = match fields_map.get("authorityKeyIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorityKeyIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cert_fingerprints: {
                        let field_value = match fields_map.get("certFingerprints") {
                            Some(value) => value,
                            None => bail!("Missing field 'certFingerprints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#crl_distribution_points: {
                        let field_value = match fields_map.get("crlDistributionPoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'crlDistributionPoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_keys: {
                        let field_value = match fields_map.get("publicKeys") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicKeys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subject_descriptions: {
                        let field_value = match fields_map.get("subjectDescriptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'subjectDescriptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subject_key_ids: {
                        let field_value = match fields_map.get("subjectKeyIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'subjectKeyIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#x_509_descriptions: {
                        let field_value = match fields_map.get("x509Descriptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'x509Descriptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
