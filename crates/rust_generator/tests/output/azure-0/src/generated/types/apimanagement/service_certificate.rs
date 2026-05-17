#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceCertificate {
    /// The password for the certificate.
    #[builder(into)]
    #[serde(rename = "certificatePassword")]
    pub r#certificate_password: Option<String>,
    /// The Base64 Encoded PFX or Base64 Encoded X.509 Certificate.
    #[builder(into)]
    #[serde(rename = "encodedCertificate")]
    pub r#encoded_certificate: String,
    /// The expiration date of the certificate in RFC3339 format: `2000-01-02T03:04:05Z`.
    #[builder(into)]
    #[serde(rename = "expiry")]
    pub r#expiry: Option<String>,
    /// The name of the Certificate Store where this certificate should be stored. Possible values are `CertificateAuthority` and `Root`.
    #[builder(into)]
    #[serde(rename = "storeName")]
    pub r#store_name: String,
    /// The subject of the certificate.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Option<String>,
    /// The thumbprint of the certificate.
    #[builder(into)]
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceCertificate {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "certificate_password",
                    &self.r#certificate_password,
                ),
                to_pulumi_object_field(
                    "encoded_certificate",
                    &self.r#encoded_certificate,
                ),
                to_pulumi_object_field(
                    "expiry",
                    &self.r#expiry,
                ),
                to_pulumi_object_field(
                    "store_name",
                    &self.r#store_name,
                ),
                to_pulumi_object_field(
                    "subject",
                    &self.r#subject,
                ),
                to_pulumi_object_field(
                    "thumbprint",
                    &self.r#thumbprint,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceCertificate {
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
                    r#certificate_password: {
                        let field_value = match fields_map.get("certificate_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encoded_certificate: {
                        let field_value = match fields_map.get("encoded_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'encoded_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expiry: {
                        let field_value = match fields_map.get("expiry") {
                            Some(value) => value,
                            None => bail!("Missing field 'expiry' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#store_name: {
                        let field_value = match fields_map.get("store_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'store_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#thumbprint: {
                        let field_value = match fields_map.get("thumbprint") {
                            Some(value) => value,
                            None => bail!("Missing field 'thumbprint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
