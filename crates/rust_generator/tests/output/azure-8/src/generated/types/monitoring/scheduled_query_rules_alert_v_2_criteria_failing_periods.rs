#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScheduledQueryRulesAlertV2CriteriaFailingPeriods {
    /// Specifies the number of violations to trigger an alert. Should be smaller or equal to `number_of_evaluation_periods`. Possible value is integer between 1 and 6.
    #[builder(into)]
    #[serde(rename = "minimumFailingPeriodsToTriggerAlert")]
    pub r#minimum_failing_periods_to_trigger_alert: i32,
    /// Specifies the number of aggregated look-back points. The look-back time window is calculated based on the aggregation granularity `window_duration` and the selected number of aggregated points. Possible value is integer between 1 and 6.
    /// 
    /// > **Note** The query look back which is `window_duration`*`number_of_evaluation_periods` cannot exceed 48 hours.
    /// 
    /// > **Note** `number_of_evaluation_periods` must be `1` for queries that do not project timestamp column
    #[builder(into)]
    #[serde(rename = "numberOfEvaluationPeriods")]
    pub r#number_of_evaluation_periods: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScheduledQueryRulesAlertV2CriteriaFailingPeriods {
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
                "minimum_failing_periods_to_trigger_alert".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#minimum_failing_periods_to_trigger_alert,
                )
                .await,
            );
            map.insert(
                "number_of_evaluation_periods".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#number_of_evaluation_periods,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScheduledQueryRulesAlertV2CriteriaFailingPeriods {
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
                    r#minimum_failing_periods_to_trigger_alert: {
                        let field_value = match fields_map.get("minimum_failing_periods_to_trigger_alert") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_failing_periods_to_trigger_alert' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_of_evaluation_periods: {
                        let field_value = match fields_map.get("number_of_evaluation_periods") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_of_evaluation_periods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
