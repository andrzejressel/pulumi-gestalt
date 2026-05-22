#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CaPoolIssuancePolicyBaselineValues {
    /// Specifies an X.509 extension, which may be used in different parts of X.509 objects like certificates, CSRs, and CRLs.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "additionalExtensions")]
    pub r#additional_extensions: Option<Vec<super::super::types::certificateauthority::CaPoolIssuancePolicyBaselineValuesAdditionalExtension>>,
    /// Describes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the
    /// "Authority Information Access" extension in the certificate.
    #[builder(into)]
    #[serde(rename = "aiaOcspServers")]
    pub r#aia_ocsp_servers: Option<Vec<String>>,
    /// Describes values that are relevant in a CA certificate.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "caOptions")]
    pub r#ca_options: Box<super::super::types::certificateauthority::CaPoolIssuancePolicyBaselineValuesCaOptions>,
    /// Indicates the intended use for keys that correspond to a certificate.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "keyUsage")]
    pub r#key_usage: Box<super::super::types::certificateauthority::CaPoolIssuancePolicyBaselineValuesKeyUsage>,
    /// Describes the X.509 name constraints extension.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "nameConstraints")]
    pub r#name_constraints: Option<Box<super::super::types::certificateauthority::CaPoolIssuancePolicyBaselineValuesNameConstraints>>,
    /// Describes the X.509 certificate policy object identifiers, per https://tools.ietf.org/html/rfc5280#section-4.2.1.4.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "policyIds")]
    pub r#policy_ids: Option<Vec<super::super::types::certificateauthority::CaPoolIssuancePolicyBaselineValuesPolicyId>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CaPoolIssuancePolicyBaselineValues {
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
                    "nameConstraints",
                    &self.r#name_constraints,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CaPoolIssuancePolicyBaselineValues {
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
                    r#name_constraints: {
                        let field_value = match fields_map.get("nameConstraints") {
                            Some(value) => value,
                            None => bail!("Missing field 'nameConstraints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
