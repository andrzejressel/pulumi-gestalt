#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DiagnosticFrontendRequest {
    /// Number of payload bytes to log (up to 8192).
    #[builder(into)]
    #[serde(rename = "bodyBytes")]
    pub r#body_bytes: Option<i32>,
    /// A `data_masking` block as defined below.
    #[builder(into)]
    #[serde(rename = "dataMasking")]
    pub r#data_masking: Option<Box<super::super::types::apimanagement::DiagnosticFrontendRequestDataMasking>>,
    /// Specifies a list of headers to log.
    #[builder(into)]
    #[serde(rename = "headersToLogs")]
    pub r#headers_to_logs: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DiagnosticFrontendRequest {
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
                "body_bytes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#body_bytes,
                )
                .await,
            );
            map.insert(
                "data_masking".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_masking,
                )
                .await,
            );
            map.insert(
                "headers_to_logs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#headers_to_logs,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DiagnosticFrontendRequest {
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
                    r#body_bytes: {
                        let field_value = match fields_map.get("body_bytes") {
                            Some(value) => value,
                            None => bail!("Missing field 'body_bytes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_masking: {
                        let field_value = match fields_map.get("data_masking") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_masking' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#headers_to_logs: {
                        let field_value = match fields_map.get("headers_to_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers_to_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
