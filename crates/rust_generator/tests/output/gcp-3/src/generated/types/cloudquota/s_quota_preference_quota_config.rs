#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SQuotaPreferenceQuotaConfig {
    /// The annotations map for clients to store small amounts of arbitrary data. Do not put PII or other sensitive information here. See https://google.aip.dev/128#annotations.
    /// An object containing a list of "key: value" pairs. Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
    #[builder(into)]
    #[serde(rename = "annotations")]
    pub r#annotations: Option<std::collections::HashMap<String, String>>,
    /// (Output)
    /// Granted quota value.
    #[builder(into)]
    #[serde(rename = "grantedValue")]
    pub r#granted_value: Option<String>,
    /// The preferred value. Must be greater than or equal to -1. If set to -1, it means the value is "unlimited".
    #[builder(into)]
    #[serde(rename = "preferredValue")]
    pub r#preferred_value: String,
    /// (Output)
    /// The origin of the quota preference request.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "requestOrigin")]
    pub r#request_origin: Option<String>,
    /// (Output)
    /// Optional details about the state of this quota preference.
    #[builder(into)]
    #[serde(rename = "stateDetail")]
    pub r#state_detail: Option<String>,
    /// (Output)
    /// The trace id that the Google Cloud uses to provision the requested quota. This trace id may be used by the client to contact Cloud support to track the state of a quota preference request. The trace id is only produced for increase requests and is unique for each request. The quota decrease requests do not have a trace id.
    #[builder(into)]
    #[serde(rename = "traceId")]
    pub r#trace_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SQuotaPreferenceQuotaConfig {
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
                "annotations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#annotations,
                )
                .await,
            );
            map.insert(
                "granted_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#granted_value,
                )
                .await,
            );
            map.insert(
                "preferred_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preferred_value,
                )
                .await,
            );
            map.insert(
                "request_origin".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_origin,
                )
                .await,
            );
            map.insert(
                "state_detail".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#state_detail,
                )
                .await,
            );
            map.insert(
                "trace_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#trace_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SQuotaPreferenceQuotaConfig {
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
                    r#annotations: {
                        let field_value = match fields_map.get("annotations") {
                            Some(value) => value,
                            None => bail!("Missing field 'annotations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#granted_value: {
                        let field_value = match fields_map.get("granted_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'granted_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preferred_value: {
                        let field_value = match fields_map.get("preferred_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'preferred_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_origin: {
                        let field_value = match fields_map.get("request_origin") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_origin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state_detail: {
                        let field_value = match fields_map.get("state_detail") {
                            Some(value) => value,
                            None => bail!("Missing field 'state_detail' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trace_id: {
                        let field_value = match fields_map.get("trace_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'trace_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
