#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceObservabilityConfig {
    /// Observability feature status for an instance.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Query string length. The default value is 10240. Any integer between 1024 and 100000 is considered valid.
    #[builder(into)]
    #[serde(rename = "maxQueryStringLength")]
    pub r#max_query_string_length: Option<i32>,
    /// Preserve comments in the query string.
    #[builder(into)]
    #[serde(rename = "preserveComments")]
    pub r#preserve_comments: Option<bool>,
    /// Number of query execution plans captured by Insights per minute for all queries combined. The default value is 5. Any integer between 0 and 200 is considered valid.
    #[builder(into)]
    #[serde(rename = "queryPlansPerMinute")]
    pub r#query_plans_per_minute: Option<i32>,
    /// Record application tags for an instance. This flag is turned "on" by default.
    #[builder(into)]
    #[serde(rename = "recordApplicationTags")]
    pub r#record_application_tags: Option<bool>,
    /// Track actively running queries. If not set, default value is "off".
    #[builder(into)]
    #[serde(rename = "trackActiveQueries")]
    pub r#track_active_queries: Option<bool>,
    /// Record wait event types during query execution for an instance.
    #[builder(into)]
    #[serde(rename = "trackWaitEventTypes")]
    pub r#track_wait_event_types: Option<bool>,
    /// Record wait events during query execution for an instance.
    #[builder(into)]
    #[serde(rename = "trackWaitEvents")]
    pub r#track_wait_events: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceObservabilityConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "max_query_string_length",
                    &self.r#max_query_string_length,
                ),
                to_pulumi_object_field(
                    "preserve_comments",
                    &self.r#preserve_comments,
                ),
                to_pulumi_object_field(
                    "query_plans_per_minute",
                    &self.r#query_plans_per_minute,
                ),
                to_pulumi_object_field(
                    "record_application_tags",
                    &self.r#record_application_tags,
                ),
                to_pulumi_object_field(
                    "track_active_queries",
                    &self.r#track_active_queries,
                ),
                to_pulumi_object_field(
                    "track_wait_event_types",
                    &self.r#track_wait_event_types,
                ),
                to_pulumi_object_field(
                    "track_wait_events",
                    &self.r#track_wait_events,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceObservabilityConfig {
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_query_string_length: {
                        let field_value = match fields_map.get("max_query_string_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_query_string_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preserve_comments: {
                        let field_value = match fields_map.get("preserve_comments") {
                            Some(value) => value,
                            None => bail!("Missing field 'preserve_comments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#record_application_tags: {
                        let field_value = match fields_map.get("record_application_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_application_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#track_active_queries: {
                        let field_value = match fields_map.get("track_active_queries") {
                            Some(value) => value,
                            None => bail!("Missing field 'track_active_queries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#track_wait_event_types: {
                        let field_value = match fields_map.get("track_wait_event_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'track_wait_event_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#track_wait_events: {
                        let field_value = match fields_map.get("track_wait_events") {
                            Some(value) => value,
                            None => bail!("Missing field 'track_wait_events' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
