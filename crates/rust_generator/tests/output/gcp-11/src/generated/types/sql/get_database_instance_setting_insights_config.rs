#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDatabaseInstanceSettingInsightsConfig {
    /// True if Query Insights feature is enabled.
    #[builder(into)]
    #[serde(rename = "queryInsightsEnabled")]
    pub r#query_insights_enabled: bool,
    /// Number of query execution plans captured by Insights per minute for all queries combined. Between 0 and 20. Default to 5.
    #[builder(into)]
    #[serde(rename = "queryPlansPerMinute")]
    pub r#query_plans_per_minute: i32,
    /// Maximum query length stored in bytes. Between 256 and 4500. Default to 1024.
    #[builder(into)]
    #[serde(rename = "queryStringLength")]
    pub r#query_string_length: i32,
    /// True if Query Insights will record application tags from query when enabled.
    #[builder(into)]
    #[serde(rename = "recordApplicationTags")]
    pub r#record_application_tags: bool,
    /// True if Query Insights will record client address when enabled.
    #[builder(into)]
    #[serde(rename = "recordClientAddress")]
    pub r#record_client_address: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDatabaseInstanceSettingInsightsConfig {
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
                "query_insights_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_insights_enabled,
                )
                .await,
            );
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
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDatabaseInstanceSettingInsightsConfig {
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
