#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceQueryInsightsConfig {
    /// Number of query execution plans captured by Insights per minute for all queries combined. The default value is 5. Any integer between 0 and 20 is considered valid.
    #[builder(into)]
    #[serde(rename = "queryPlansPerMinute")]
    pub r#query_plans_per_minute: Option<i32>,
    /// Query string length. The default value is 1024. Any integer between 256 and 4500 is considered valid.
    #[builder(into)]
    #[serde(rename = "queryStringLength")]
    pub r#query_string_length: Option<i32>,
    /// Record application tags for an instance. This flag is turned "on" by default.
    #[builder(into)]
    #[serde(rename = "recordApplicationTags")]
    pub r#record_application_tags: Option<bool>,
    /// Record client address for an instance. Client address is PII information. This flag is turned "on" by default.
    #[builder(into)]
    #[serde(rename = "recordClientAddress")]
    pub r#record_client_address: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceQueryInsightsConfig {
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
                "query_plans_per_minute".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_plans_per_minute,
                )
                .await,
            );
            map.insert(
                "query_string_length".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_string_length,
                )
                .await,
            );
            map.insert(
                "record_application_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#record_application_tags,
                )
                .await,
            );
            map.insert(
                "record_client_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#record_client_address,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceQueryInsightsConfig {
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
                    r#query_plans_per_minute: {
                        let field_value = match fields_map.get("query_plans_per_minute") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_plans_per_minute' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_string_length: {
                        let field_value = match fields_map.get("query_string_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_string_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_application_tags: {
                        let field_value = match fields_map.get("record_application_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_application_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_client_address: {
                        let field_value = match fields_map.get("record_client_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_client_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
