#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateTemplatePredefinedValues {
    /// Optional. Describes custom X.509 extensions.
    /// Structure is documented below.
    #[builder(into)]
    pub r#additional_extensions: Option<Vec<super::super::types::certificateauthority::CertificateTemplatePredefinedValuesAdditionalExtension>>,
    /// Optional. Describes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the "Authority Information Access" extension in the certificate.
    #[builder(into)]
    pub r#aia_ocsp_servers: Option<Vec<String>>,
    /// Optional. Describes options in this X509Parameters that are relevant in a CA certificate.
    /// Structure is documented below.
    #[builder(into)]
    pub r#ca_options: Option<Box<super::super::types::certificateauthority::CertificateTemplatePredefinedValuesCaOptions>>,
    /// Optional. Indicates the intended use for keys that correspond to a certificate.
    /// Structure is documented below.
    #[builder(into)]
    pub r#key_usage: Option<Box<super::super::types::certificateauthority::CertificateTemplatePredefinedValuesKeyUsage>>,
    /// Optional. Describes the X.509 certificate policy object identifiers, per https://tools.ietf.org/html/rfc5280#section-4.2.1.4.
    /// Structure is documented below.
    #[builder(into)]
    pub r#policy_ids: Option<Vec<super::super::types::certificateauthority::CertificateTemplatePredefinedValuesPolicyId>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateTemplatePredefinedValues {
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
                    "additionalExtensions",
                    &self.r#additional_extensions,
                ),
                to_pulumi_object_field(
                    "aiaOcspServers",
                    &self.r#aia_ocsp_servers,
                ),
                to_pulumi_object_field(
                    "caOptions",
                    &self.r#ca_options,
                ),
                to_pulumi_object_field(
                    "keyUsage",
                    &self.r#key_usage,
                ),
                to_pulumi_object_field(
                    "policyIds",
                    &self.r#policy_ids,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateTemplatePredefinedValues {
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
                    r#additional_extensions: {
                        let field_value = match fields_map.get("additionalExtensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'additionalExtensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aia_ocsp_servers: {
                        let field_value = match fields_map.get("aiaOcspServers") {
                            Some(value) => value,
                            None => bail!("Missing field 'aiaOcspServers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ca_options: {
                        let field_value = match fields_map.get("caOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'caOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_usage: {
                        let field_value = match fields_map.get("keyUsage") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyUsage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_ids: {
                        let field_value = match fields_map.get("policyIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'policyIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
