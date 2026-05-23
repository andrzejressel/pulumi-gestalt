#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceObservabilityConfig {
    /// Observability feature status for an instance.
    #[builder(into)]
    pub r#enabled: Option<bool>,
    /// Query string length. The default value is 10240. Any integer between 1024 and 100000 is considered valid.
    #[builder(into)]
    pub r#max_query_string_length: Option<i32>,
    /// Preserve comments in the query string.
    #[builder(into)]
    pub r#preserve_comments: Option<bool>,
    /// Number of query execution plans captured by Insights per minute for all queries combined. The default value is 5. Any integer between 0 and 200 is considered valid.
    #[builder(into)]
    pub r#query_plans_per_minute: Option<i32>,
    /// Record application tags for an instance. This flag is turned "on" by default.
    #[builder(into)]
    pub r#record_application_tags: Option<bool>,
    /// Track actively running queries. If not set, default value is "off".
    #[builder(into)]
    pub r#track_active_queries: Option<bool>,
    /// Record wait event types during query execution for an instance.
    #[builder(into)]
    pub r#track_wait_event_types: Option<bool>,
    /// Record wait events during query execution for an instance.
    #[builder(into)]
    pub r#track_wait_events: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceObservabilityConfig {
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
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "maxQueryStringLength",
                    &self.r#max_query_string_length,
                ),
                to_pulumi_object_field(
                    "preserveComments",
                    &self.r#preserve_comments,
                ),
                to_pulumi_object_field(
                    "queryPlansPerMinute",
                    &self.r#query_plans_per_minute,
                ),
                to_pulumi_object_field(
                    "recordApplicationTags",
                    &self.r#record_application_tags,
                ),
                to_pulumi_object_field(
                    "trackActiveQueries",
                    &self.r#track_active_queries,
                ),
                to_pulumi_object_field(
                    "trackWaitEventTypes",
                    &self.r#track_wait_event_types,
                ),
                to_pulumi_object_field(
                    "trackWaitEvents",
                    &self.r#track_wait_events,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("maxQueryStringLength") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxQueryStringLength' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preserve_comments: {
                        let field_value = match fields_map.get("preserveComments") {
                            Some(value) => value,
                            None => bail!("Missing field 'preserveComments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_plans_per_minute: {
                        let field_value = match fields_map.get("queryPlansPerMinute") {
                            Some(value) => value,
                            None => bail!("Missing field 'queryPlansPerMinute' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_application_tags: {
                        let field_value = match fields_map.get("recordApplicationTags") {
                            Some(value) => value,
                            None => bail!("Missing field 'recordApplicationTags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#track_active_queries: {
                        let field_value = match fields_map.get("trackActiveQueries") {
                            Some(value) => value,
                            None => bail!("Missing field 'trackActiveQueries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#track_wait_event_types: {
                        let field_value = match fields_map.get("trackWaitEventTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'trackWaitEventTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#track_wait_events: {
                        let field_value = match fields_map.get("trackWaitEvents") {
                            Some(value) => value,
                            None => bail!("Missing field 'trackWaitEvents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
