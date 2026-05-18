#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomHostnameSslSetting {
    /// List of SSL/TLS ciphers to associate with this certificate.
    #[builder(into)]
    #[serde(rename = "ciphers")]
    pub r#ciphers: Option<Vec<String>>,
    /// Whether early hints should be supported. Available values: `on`, `off`.
    #[builder(into)]
    #[serde(rename = "earlyHints")]
    pub r#early_hints: Option<String>,
    /// Whether HTTP2 should be supported. Available values: `on`, `off`.
    #[builder(into)]
    #[serde(rename = "http2")]
    pub r#http_2: Option<String>,
    /// Lowest version of TLS this certificate should support. Available values: `1.0`, `1.1`, `1.2`, `1.3`.
    #[builder(into)]
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Option<String>,
    /// Whether TLSv1.3 should be supported. Available values: `on`, `off`.
    #[builder(into)]
    #[serde(rename = "tls13")]
    pub r#tls_13: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CustomHostnameSslSetting {
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
                    "ciphers",
                    &self.r#ciphers,
                ),
                to_pulumi_object_field(
                    "early_hints",
                    &self.r#early_hints,
                ),
                to_pulumi_object_field(
                    "http_2",
                    &self.r#http_2,
                ),
                to_pulumi_object_field(
                    "min_tls_version",
                    &self.r#min_tls_version,
                ),
                to_pulumi_object_field(
                    "tls_13",
                    &self.r#tls_13,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CustomHostnameSslSetting {
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
                    r#ciphers: {
                        let field_value = match fields_map.get("ciphers") {
                            Some(value) => value,
                            None => bail!("Missing field 'ciphers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#early_hints: {
                        let field_value = match fields_map.get("early_hints") {
                            Some(value) => value,
                            None => bail!("Missing field 'early_hints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_2: {
                        let field_value = match fields_map.get("http_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_tls_version: {
                        let field_value = match fields_map.get("min_tls_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_tls_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_13: {
                        let field_value = match fields_map.get("tls_13") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_13' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
