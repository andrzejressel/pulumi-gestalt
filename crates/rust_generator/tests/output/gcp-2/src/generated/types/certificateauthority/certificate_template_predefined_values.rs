#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateTemplatePredefinedValues {
    /// Optional. Describes custom X.509 extensions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "additionalExtensions")]
    pub r#additional_extensions: Option<Vec<super::super::types::certificateauthority::CertificateTemplatePredefinedValuesAdditionalExtension>>,
    /// Optional. Describes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the "Authority Information Access" extension in the certificate.
    #[builder(into)]
    #[serde(rename = "aiaOcspServers")]
    pub r#aia_ocsp_servers: Option<Vec<String>>,
    /// Optional. Describes options in this X509Parameters that are relevant in a CA certificate.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "caOptions")]
    pub r#ca_options: Option<Box<super::super::types::certificateauthority::CertificateTemplatePredefinedValuesCaOptions>>,
    /// Optional. Indicates the intended use for keys that correspond to a certificate.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "keyUsage")]
    pub r#key_usage: Option<Box<super::super::types::certificateauthority::CertificateTemplatePredefinedValuesKeyUsage>>,
    /// Optional. Describes the X.509 certificate policy object identifiers, per https://tools.ietf.org/html/rfc5280#section-4.2.1.4.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "policyIds")]
    pub r#policy_ids: Option<Vec<super::super::types::certificateauthority::CertificateTemplatePredefinedValuesPolicyId>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateTemplatePredefinedValues {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "additional_extensions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#additional_extensions,
                )
                .await,
            );
            map.insert(
                "aia_ocsp_servers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aia_ocsp_servers,
                )
                .await,
            );
            map.insert(
                "ca_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ca_options,
                )
                .await,
            );
            map.insert(
                "key_usage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_usage,
                )
                .await,
            );
            map.insert(
                "policy_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#policy_ids,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
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
                        let field_value = match fields_map.get("additional_extensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_extensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aia_ocsp_servers: {
                        let field_value = match fields_map.get("aia_ocsp_servers") {
                            Some(value) => value,
                            None => bail!("Missing field 'aia_ocsp_servers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ca_options: {
                        let field_value = match fields_map.get("ca_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'ca_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_usage: {
                        let field_value = match fields_map.get("key_usage") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_usage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_ids: {
                        let field_value = match fields_map.get("policy_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
