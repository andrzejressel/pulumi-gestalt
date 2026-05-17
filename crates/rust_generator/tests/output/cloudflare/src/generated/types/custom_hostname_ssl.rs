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
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "bundle_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bundle_method,
                )
                .await,
            );
            map.insert(
                "certificate_authority".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#certificate_authority,
                )
                .await,
            );
            map.insert(
                "custom_certificate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_certificate,
                )
                .await,
            );
            map.insert(
                "custom_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_key,
                )
                .await,
            );
            map.insert(
                "method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#method,
                )
                .await,
            );
            map.insert(
                "settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#settings,
                )
                .await,
            );
            map.insert(
                "status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#status,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
                )
                .await,
            );
            map.insert(
                "validation_errors".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#validation_errors,
                )
                .await,
            );
            map.insert(
                "validation_records".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#validation_records,
                )
                .await,
            );
            map.insert(
                "wildcard".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#wildcard,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("bundle_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'bundle_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certificate_authority: {
                        let field_value = match fields_map.get("certificate_authority") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate_authority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_certificate: {
                        let field_value = match fields_map.get("custom_certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_key: {
                        let field_value = match fields_map.get("custom_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#validation_errors: {
                        let field_value = match fields_map.get("validation_errors") {
                            Some(value) => value,
                            None => bail!("Missing field 'validation_errors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#validation_records: {
                        let field_value = match fields_map.get("validation_records") {
                            Some(value) => value,
                            None => bail!("Missing field 'validation_records' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
