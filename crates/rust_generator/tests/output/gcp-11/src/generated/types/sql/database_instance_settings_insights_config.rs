#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatabaseInstanceSettingsInsightsConfig {
    /// True if Query Insights feature is enabled.
    #[builder(into)]
    #[serde(rename = "queryInsightsEnabled")]
    pub r#query_insights_enabled: Option<bool>,
    /// Number of query execution plans captured by Insights per minute for all queries combined. Between 0 and 20. Default to 5.
    #[builder(into)]
    #[serde(rename = "queryPlansPerMinute")]
    pub r#query_plans_per_minute: Option<i32>,
    /// Maximum query length stored in bytes. Between 256 and 4500. Default to 1024. Higher query lengths are more useful for analytical queries, but they also require more memory. Changing the query length requires you to restart the instance. You can still add tags to queries that exceed the length limit.
    #[builder(into)]
    #[serde(rename = "queryStringLength")]
    pub r#query_string_length: Option<i32>,
    /// True if Query Insights will record application tags from query when enabled.
    #[builder(into)]
    #[serde(rename = "recordApplicationTags")]
    pub r#record_application_tags: Option<bool>,
    /// True if Query Insights will record client address when enabled.
    #[builder(into)]
    #[serde(rename = "recordClientAddress")]
    pub r#record_client_address: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatabaseInstanceSettingsInsightsConfig {
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
                    "query_insights_enabled",
                    &self.r#query_insights_enabled,
                ),
                to_pulumi_object_field(
                    "query_plans_per_minute",
                    &self.r#query_plans_per_minute,
                ),
                to_pulumi_object_field(
                    "query_string_length",
                    &self.r#query_string_length,
                ),
                to_pulumi_object_field(
                    "record_application_tags",
                    &self.r#record_application_tags,
                ),
                to_pulumi_object_field(
                    "record_client_address",
                    &self.r#record_client_address,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatabaseInstanceSettingsInsightsConfig {
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
                    r#query_insights_enabled: {
                        let field_value = match fields_map.get("query_insights_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_insights_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
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
