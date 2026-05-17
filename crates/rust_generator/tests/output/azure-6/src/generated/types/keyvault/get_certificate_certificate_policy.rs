#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCertificateCertificatePolicy {
    /// A `issuer_parameters` block as defined below.
    #[builder(into)]
    #[serde(rename = "issuerParameters")]
    pub r#issuer_parameters: Vec<super::super::types::keyvault::GetCertificateCertificatePolicyIssuerParameter>,
    /// A `key_properties` block as defined below.
    #[builder(into)]
    #[serde(rename = "keyProperties")]
    pub r#key_properties: Vec<super::super::types::keyvault::GetCertificateCertificatePolicyKeyProperty>,
    /// A `lifetime_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "lifetimeActions")]
    pub r#lifetime_actions: Vec<super::super::types::keyvault::GetCertificateCertificatePolicyLifetimeAction>,
    /// A `secret_properties` block as defined below.
    #[builder(into)]
    #[serde(rename = "secretProperties")]
    pub r#secret_properties: Vec<super::super::types::keyvault::GetCertificateCertificatePolicySecretProperty>,
    /// An `x509_certificate_properties` block as defined below.
    #[builder(into)]
    #[serde(rename = "x509CertificateProperties")]
    pub r#x_509_certificate_properties: Vec<super::super::types::keyvault::GetCertificateCertificatePolicyX509CertificateProperty>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCertificateCertificatePolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "issuer_parameters",
                    &self.r#issuer_parameters,
                ),
                to_pulumi_object_field(
                    "key_properties",
                    &self.r#key_properties,
                ),
                to_pulumi_object_field(
                    "lifetime_actions",
                    &self.r#lifetime_actions,
                ),
                to_pulumi_object_field(
                    "secret_properties",
                    &self.r#secret_properties,
                ),
                to_pulumi_object_field(
                    "x_509_certificate_properties",
                    &self.r#x_509_certificate_properties,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCertificateCertificatePolicy {
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
                        let field_value = match fields_map.get("issuer_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'issuer_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_properties: {
                        let field_value = match fields_map.get("key_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifetime_actions: {
                        let field_value = match fields_map.get("lifetime_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifetime_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_properties: {
                        let field_value = match fields_map.get("secret_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#x_509_certificate_properties: {
                        let field_value = match fields_map.get("x_509_certificate_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'x_509_certificate_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
