#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertifiateCertificatePolicy {
    /// A `issuer_parameters` block as defined below.
    #[builder(into)]
    #[serde(rename = "issuerParameters")]
    pub r#issuer_parameters: Box<super::super::types::keyvault::CertifiateCertificatePolicyIssuerParameters>,
    /// A `key_properties` block as defined below.
    #[builder(into)]
    #[serde(rename = "keyProperties")]
    pub r#key_properties: Box<super::super::types::keyvault::CertifiateCertificatePolicyKeyProperties>,
    /// A `lifetime_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "lifetimeActions")]
    pub r#lifetime_actions: Option<Vec<super::super::types::keyvault::CertifiateCertificatePolicyLifetimeAction>>,
    /// A `secret_properties` block as defined below.
    #[builder(into)]
    #[serde(rename = "secretProperties")]
    pub r#secret_properties: Box<super::super::types::keyvault::CertifiateCertificatePolicySecretProperties>,
    /// A `x509_certificate_properties` block as defined below. Required when `certificate` block is not specified.
    #[builder(into)]
    #[serde(rename = "x509CertificateProperties")]
    pub r#x_509_certificate_properties: Option<Box<super::super::types::keyvault::CertifiateCertificatePolicyX509CertificateProperties>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertifiateCertificatePolicy {
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
                    "issuerParameters",
                    &self.r#issuer_parameters,
                ),
                to_pulumi_object_field(
                    "keyProperties",
                    &self.r#key_properties,
                ),
                to_pulumi_object_field(
                    "lifetimeActions",
                    &self.r#lifetime_actions,
                ),
                to_pulumi_object_field(
                    "secretProperties",
                    &self.r#secret_properties,
                ),
                to_pulumi_object_field(
                    "x509CertificateProperties",
                    &self.r#x_509_certificate_properties,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertifiateCertificatePolicy {
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
                    r#issuer_parameters: {
                        let field_value = match fields_map.get("issuerParameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuerParameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_properties: {
                        let field_value = match fields_map.get("keyProperties") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyProperties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifetime_actions: {
                        let field_value = match fields_map.get("lifetimeActions") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifetimeActions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_properties: {
                        let field_value = match fields_map.get("secretProperties") {
                            Some(value) => value,
                            None => bail!("Missing field 'secretProperties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#x_509_certificate_properties: {
                        let field_value = match fields_map.get("x509CertificateProperties") {
                            Some(value) => value,
                            None => bail!("Missing field 'x509CertificateProperties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
