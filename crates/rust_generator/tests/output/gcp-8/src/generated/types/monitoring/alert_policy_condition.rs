#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertPolicyCondition {
    /// A condition that checks that a time series
    /// continues to receive new data points.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "conditionAbsent")]
    pub r#condition_absent: Option<Box<super::super::types::monitoring::AlertPolicyConditionConditionAbsent>>,
    /// A condition that checks for log messages matching given constraints.
    /// If set, no other conditions can be present.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "conditionMatchedLog")]
    pub r#condition_matched_log: Option<Box<super::super::types::monitoring::AlertPolicyConditionConditionMatchedLog>>,
    /// A Monitoring Query Language query that outputs a boolean stream
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "conditionMonitoringQueryLanguage")]
    pub r#condition_monitoring_query_language: Option<Box<super::super::types::monitoring::AlertPolicyConditionConditionMonitoringQueryLanguage>>,
    /// A condition type that allows alert policies to be defined using
    /// Prometheus Query Language (PromQL).
    /// The PrometheusQueryLanguageCondition message contains information
    /// from a Prometheus alerting rule and its associated rule group.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "conditionPrometheusQueryLanguage")]
    pub r#condition_prometheus_query_language: Option<Box<super::super::types::monitoring::AlertPolicyConditionConditionPrometheusQueryLanguage>>,
    /// A condition that compares a time series against a
    /// threshold.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "conditionThreshold")]
    pub r#condition_threshold: Option<Box<super::super::types::monitoring::AlertPolicyConditionConditionThreshold>>,
    /// A short name or phrase used to identify the
    /// condition in dashboards, notifications, and
    /// incidents. To avoid confusion, don't use the same
    /// display name for multiple conditions in the same
    /// policy.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: String,
    /// (Output)
    /// The unique resource name for this condition.
    /// Its syntax is:
    /// projects/[PROJECT_ID]/alertPolicies/[POLICY_ID]/conditions/[CONDITION_ID]
    /// [CONDITION_ID] is assigned by Stackdriver Monitoring when
    /// the condition is created as part of a new or updated alerting
    /// policy.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AlertPolicyCondition {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "condition_absent",
                    &self.r#condition_absent,
                ),
                to_pulumi_object_field(
                    "condition_matched_log",
                    &self.r#condition_matched_log,
                ),
                to_pulumi_object_field(
                    "condition_monitoring_query_language",
                    &self.r#condition_monitoring_query_language,
                ),
                to_pulumi_object_field(
                    "condition_prometheus_query_language",
                    &self.r#condition_prometheus_query_language,
                ),
                to_pulumi_object_field(
                    "condition_threshold",
                    &self.r#condition_threshold,
                ),
                to_pulumi_object_field(
                    "display_name",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AlertPolicyCondition {
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
                    r#condition_absent: {
                        let field_value = match fields_map.get("condition_absent") {
                            Some(value) => value,
                            None => bail!("Missing field 'condition_absent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#condition_matched_log: {
                        let field_value = match fields_map.get("condition_matched_log") {
                            Some(value) => value,
                            None => bail!("Missing field 'condition_matched_log' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#condition_monitoring_query_language: {
                        let field_value = match fields_map.get("condition_monitoring_query_language") {
                            Some(value) => value,
                            None => bail!("Missing field 'condition_monitoring_query_language' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#condition_prometheus_query_language: {
                        let field_value = match fields_map.get("condition_prometheus_query_language") {
                            Some(value) => value,
                            None => bail!("Missing field 'condition_prometheus_query_language' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#condition_threshold: {
                        let field_value = match fields_map.get("condition_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'condition_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("display_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
