#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAlertRuleTemplateScheduledTemplate {
    /// The description of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The query of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: String,
    /// The ISO 8601 timespan duration between two consecutive queries.
    #[builder(into)]
    #[serde(rename = "queryFrequency")]
    pub r#query_frequency: String,
    /// The ISO 8601 timespan duration, which determine the time period of the data covered by the query.
    #[builder(into)]
    #[serde(rename = "queryPeriod")]
    pub r#query_period: String,
    /// The alert severity of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: String,
    /// A list of categories of attacks by which to classify the rule.
    #[builder(into)]
    #[serde(rename = "tactics")]
    pub r#tactics: Vec<String>,
    /// The alert trigger operator, combined with `trigger_threshold`, setting alert threshold of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "triggerOperator")]
    pub r#trigger_operator: String,
    /// The baseline number of query results generated, combined with `trigger_operator`, setting alert threshold of this Sentinel Scheduled Alert Rule Template.
    #[builder(into)]
    #[serde(rename = "triggerThreshold")]
    pub r#trigger_threshold: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAlertRuleTemplateScheduledTemplate {
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
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "query".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query,
                )
                .await,
            );
            map.insert(
                "query_frequency".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_frequency,
                )
                .await,
            );
            map.insert(
                "query_period".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_period,
                )
                .await,
            );
            map.insert(
                "severity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#severity,
                )
                .await,
            );
            map.insert(
                "tactics".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tactics,
                )
                .await,
            );
            map.insert(
                "trigger_operator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#trigger_operator,
                )
                .await,
            );
            map.insert(
                "trigger_threshold".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#trigger_threshold,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAlertRuleTemplateScheduledTemplate {
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query: {
                        let field_value = match fields_map.get("query") {
                            Some(value) => value,
                            None => bail!("Missing field 'query' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_frequency: {
                        let field_value = match fields_map.get("query_frequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_frequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_period: {
                        let field_value = match fields_map.get("query_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#severity: {
                        let field_value = match fields_map.get("severity") {
                            Some(value) => value,
                            None => bail!("Missing field 'severity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tactics: {
                        let field_value = match fields_map.get("tactics") {
                            Some(value) => value,
                            None => bail!("Missing field 'tactics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trigger_operator: {
                        let field_value = match fields_map.get("trigger_operator") {
                            Some(value) => value,
                            None => bail!("Missing field 'trigger_operator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trigger_threshold: {
                        let field_value = match fields_map.get("trigger_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'trigger_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
