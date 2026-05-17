#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAuthorityConfigX509Config {
    /// Specifies an X.509 extension, which may be used in different parts of X.509 objects like certificates, CSRs, and CRLs.
    #[builder(into)]
    #[serde(rename = "additionalExtensions")]
    pub r#additional_extensions: Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigAdditionalExtension>,
    /// Describes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the
    /// "Authority Information Access" extension in the certificate.
    #[builder(into)]
    #[serde(rename = "aiaOcspServers")]
    pub r#aia_ocsp_servers: Vec<String>,
    /// Describes values that are relevant in a CA certificate.
    #[builder(into)]
    #[serde(rename = "caOptions")]
    pub r#ca_options: Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigCaOption>,
    /// Indicates the intended use for keys that correspond to a certificate.
    #[builder(into)]
    #[serde(rename = "keyUsages")]
    pub r#key_usages: Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigKeyUsage>,
    /// Describes the X.509 name constraints extension.
    #[builder(into)]
    #[serde(rename = "nameConstraints")]
    pub r#name_constraints: Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigNameConstraint>,
    /// Describes the X.509 certificate policy object identifiers, per https://tools.ietf.org/html/rfc5280#section-4.2.1.4.
    #[builder(into)]
    #[serde(rename = "policyIds")]
    pub r#policy_ids: Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigPolicyId>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAuthorityConfigX509Config {
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
                    "additional_extensions",
                    &self.r#additional_extensions,
                ),
                to_pulumi_object_field(
                    "aia_ocsp_servers",
                    &self.r#aia_ocsp_servers,
                ),
                to_pulumi_object_field(
                    "ca_options",
                    &self.r#ca_options,
                ),
                to_pulumi_object_field(
                    "key_usages",
                    &self.r#key_usages,
                ),
                to_pulumi_object_field(
                    "name_constraints",
                    &self.r#name_constraints,
                ),
                to_pulumi_object_field(
                    "policy_ids",
                    &self.r#policy_ids,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAuthorityConfigX509Config {
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
                    r#key_usages: {
                        let field_value = match fields_map.get("key_usages") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_usages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name_constraints: {
                        let field_value = match fields_map.get("name_constraints") {
                            Some(value) => value,
                            None => bail!("Missing field 'name_constraints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
