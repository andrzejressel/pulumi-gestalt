#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDatabaseInstancesInstanceSettingInsightsConfig {
    /// True if Query Insights feature is enabled.
    #[builder(into)]
    pub r#query_insights_enabled: bool,
    /// Number of query execution plans captured by Insights per minute for all queries combined. Between 0 and 20. Default to 5.
    #[builder(into)]
    pub r#query_plans_per_minute: i32,
    /// Maximum query length stored in bytes. Between 256 and 4500. Default to 1024.
    #[builder(into)]
    pub r#query_string_length: i32,
    /// True if Query Insights will record application tags from query when enabled.
    #[builder(into)]
    pub r#record_application_tags: bool,
    /// True if Query Insights will record client address when enabled.
    #[builder(into)]
    pub r#record_client_address: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDatabaseInstancesInstanceSettingInsightsConfig {
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
                    "queryInsightsEnabled",
                    &self.r#query_insights_enabled,
                ),
                to_pulumi_object_field(
                    "queryPlansPerMinute",
                    &self.r#query_plans_per_minute,
                ),
                to_pulumi_object_field(
                    "queryStringLength",
                    &self.r#query_string_length,
                ),
                to_pulumi_object_field(
                    "recordApplicationTags",
                    &self.r#record_application_tags,
                ),
                to_pulumi_object_field(
                    "recordClientAddress",
                    &self.r#record_client_address,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDatabaseInstancesInstanceSettingInsightsConfig {
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
                        let field_value = match fields_map.get("queryInsightsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'queryInsightsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#query_string_length: {
                        let field_value = match fields_map.get("queryStringLength") {
                            Some(value) => value,
                            None => bail!("Missing field 'queryStringLength' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#record_client_address: {
                        let field_value = match fields_map.get("recordClientAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'recordClientAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
