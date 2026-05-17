#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateSelfManaged {
    /// (Optional, Deprecated)
    /// The certificate chain in PEM-encoded form.
    /// Leaf certificate comes first, followed by intermediate ones if any.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    /// 
    /// > **Warning:** `certificate_pem` is deprecated and will be removed in a future major release. Use `pem_certificate` instead.
    #[builder(into)]
    #[serde(rename = "certificatePem")]
    pub r#certificate_pem: Option<String>,
    /// The certificate chain in PEM-encoded form.
    /// Leaf certificate comes first, followed by intermediate ones if any.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "pemCertificate")]
    pub r#pem_certificate: Option<String>,
    /// The private key of the leaf certificate in PEM-encoded form.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "pemPrivateKey")]
    pub r#pem_private_key: Option<String>,
    /// (Optional, Deprecated)
    /// The private key of the leaf certificate in PEM-encoded form.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    /// 
    /// > **Warning:** `private_key_pem` is deprecated and will be removed in a future major release. Use `pem_private_key` instead.
    #[builder(into)]
    #[serde(rename = "privateKeyPem")]
    pub r#private_key_pem: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateSelfManaged {
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
                    "certificate_pem",
                    &self.r#certificate_pem,
                ),
                to_pulumi_object_field(
                    "pem_certificate",
                    &self.r#pem_certificate,
                ),
                to_pulumi_object_field(
                    "pem_private_key",
                    &self.r#pem_private_key,
                ),
                to_pulumi_object_field(
                    "private_key_pem",
                    &self.r#private_key_pem,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateSelfManaged {
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
                    r#certificate_pem: {
                        let field_value = match fields_map.get("certificate_pem") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_pem' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pem_certificate: {
                        let field_value = match fields_map.get("pem_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'pem_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pem_private_key: {
                        let field_value = match fields_map.get("pem_private_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'pem_private_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_key_pem: {
                        let field_value = match fields_map.get("private_key_pem") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_key_pem' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
