#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomHostnameSsl {
    /// A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`.
    #[builder(into)]
    #[serde(rename = "bundleMethod")]
    pub r#bundle_method: Option<String>,
    #[builder(into)]
    #[serde(rename = "certificateAuthority")]
    pub r#certificate_authority: Option<String>,
    /// If a custom uploaded certificate is used.
    #[builder(into)]
    #[serde(rename = "customCertificate")]
    pub r#custom_certificate: Option<String>,
    /// The key for a custom uploaded certificate.
    #[builder(into)]
    #[serde(rename = "customKey")]
    pub r#custom_key: Option<String>,
    /// Domain control validation (DCV) method used for this hostname. Available values: `http`, `txt`, `email`.
    #[builder(into)]
    #[serde(rename = "method")]
    pub r#method: Option<String>,
    /// SSL/TLS settings for the certificate.
    #[builder(into)]
    #[serde(rename = "settings")]
    pub r#settings: Option<Vec<super::types::CustomHostnameSslSetting>>,
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// Level of validation to be used for this hostname. Available values: `dv`. Defaults to `dv`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    #[builder(into)]
    #[serde(rename = "validationErrors")]
    pub r#validation_errors: Option<Vec<super::types::CustomHostnameSslValidationError>>,
    #[builder(into)]
    #[serde(rename = "validationRecords")]
    pub r#validation_records: Option<Vec<super::types::CustomHostnameSslValidationRecord>>,
    /// Indicates whether the certificate covers a wildcard.
    #[builder(into)]
    #[serde(rename = "wildcard")]
    pub r#wildcard: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CustomHostnameSsl {
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
                    "bundleMethod",
                    &self.r#bundle_method,
                ),
                to_pulumi_object_field(
                    "certificateAuthority",
                    &self.r#certificate_authority,
                ),
                to_pulumi_object_field(
                    "customCertificate",
                    &self.r#custom_certificate,
                ),
                to_pulumi_object_field(
                    "customKey",
                    &self.r#custom_key,
                ),
                to_pulumi_object_field(
                    "method",
                    &self.r#method,
                ),
                to_pulumi_object_field(
                    "settings",
                    &self.r#settings,
                ),
                to_pulumi_object_field(
                    "status",
                    &self.r#status,
                ),
                to_pulumi_object_field(
                    "type",
                    &self.r#type_,
                ),
                to_pulumi_object_field(
                    "validationErrors",
                    &self.r#validation_errors,
                ),
                to_pulumi_object_field(
                    "validationRecords",
                    &self.r#validation_records,
                ),
                to_pulumi_object_field(
                    "wildcard",
                    &self.r#wildcard,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CustomHostnameSsl {
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
                    r#bundle_method: {
                        let field_value = match fields_map.get("bundleMethod") {
                            Some(value) => value,
                            None => bail!("Missing field 'bundleMethod' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certificate_authority: {
                        let field_value = match fields_map.get("certificateAuthority") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificateAuthority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_certificate: {
                        let field_value = match fields_map.get("customCertificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'customCertificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_key: {
                        let field_value = match fields_map.get("customKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'customKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#method: {
                        let field_value = match fields_map.get("method") {
                            Some(value) => value,
                            None => bail!("Missing field 'method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#settings: {
                        let field_value = match fields_map.get("settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type") {
                            Some(value) => value,
                            None => bail!("Missing field 'type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#validation_errors: {
                        let field_value = match fields_map.get("validationErrors") {
                            Some(value) => value,
                            None => bail!("Missing field 'validationErrors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#validation_records: {
                        let field_value = match fields_map.get("validationRecords") {
                            Some(value) => value,
                            None => bail!("Missing field 'validationRecords' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#wildcard: {
                        let field_value = match fields_map.get("wildcard") {
                            Some(value) => value,
                            None => bail!("Missing field 'wildcard' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
