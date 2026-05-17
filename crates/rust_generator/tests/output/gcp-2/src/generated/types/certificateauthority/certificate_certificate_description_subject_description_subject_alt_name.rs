#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateCertificateDescriptionSubjectDescriptionSubjectAltName {
    /// (Output)
    /// Contains additional subject alternative name values.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customSans")]
    pub r#custom_sans: Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionSubjectDescriptionSubjectAltNameCustomSan>>,
    /// Contains only valid, fully-qualified host names.
    #[builder(into)]
    #[serde(rename = "dnsNames")]
    pub r#dns_names: Option<Vec<String>>,
    /// Contains only valid RFC 2822 E-mail addresses.
    #[builder(into)]
    #[serde(rename = "emailAddresses")]
    pub r#email_addresses: Option<Vec<String>>,
    /// Contains only valid 32-bit IPv4 addresses or RFC 4291 IPv6 addresses.
    #[builder(into)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Option<Vec<String>>,
    /// Contains only valid RFC 3986 URIs.
    #[builder(into)]
    #[serde(rename = "uris")]
    pub r#uris: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateCertificateDescriptionSubjectDescriptionSubjectAltName {
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
                    "custom_sans",
                    &self.r#custom_sans,
                ),
                to_pulumi_object_field(
                    "dns_names",
                    &self.r#dns_names,
                ),
                to_pulumi_object_field(
                    "email_addresses",
                    &self.r#email_addresses,
                ),
                to_pulumi_object_field(
                    "ip_addresses",
                    &self.r#ip_addresses,
                ),
                to_pulumi_object_field(
                    "uris",
                    &self.r#uris,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateCertificateDescriptionSubjectDescriptionSubjectAltName {
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
                    r#custom_sans: {
                        let field_value = match fields_map.get("custom_sans") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_sans' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_names: {
                        let field_value = match fields_map.get("dns_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_addresses: {
                        let field_value = match fields_map.get("email_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_addresses: {
                        let field_value = match fields_map.get("ip_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uris: {
                        let field_value = match fields_map.get("uris") {
                            Some(value) => value,
                            None => bail!("Missing field 'uris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
