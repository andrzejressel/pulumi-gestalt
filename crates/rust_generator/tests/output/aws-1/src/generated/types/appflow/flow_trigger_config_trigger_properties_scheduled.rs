#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowTriggerConfigTriggerPropertiesScheduled {
    /// Whether a scheduled flow has an incremental data transfer or a complete data transfer for each flow run. Valid values are `Incremental` and `Complete`.
    #[builder(into)]
    #[serde(rename = "dataPullMode")]
    pub r#data_pull_mode: Option<String>,
    /// Date range for the records to import from the connector in the first flow run. Must be a valid RFC3339 timestamp.
    #[builder(into)]
    #[serde(rename = "firstExecutionFrom")]
    pub r#first_execution_from: Option<String>,
    /// Scheduled end time for a schedule-triggered flow. Must be a valid RFC3339 timestamp.
    #[builder(into)]
    #[serde(rename = "scheduleEndTime")]
    pub r#schedule_end_time: Option<String>,
    /// Scheduling expression that determines the rate at which the schedule will run, for example `rate(5minutes)`.
    #[builder(into)]
    #[serde(rename = "scheduleExpression")]
    pub r#schedule_expression: String,
    /// Optional offset that is added to the time interval for a schedule-triggered flow. Maximum value of 36000.
    #[builder(into)]
    #[serde(rename = "scheduleOffset")]
    pub r#schedule_offset: Option<i32>,
    /// Scheduled start time for a schedule-triggered flow. Must be a valid RFC3339 timestamp.
    #[builder(into)]
    #[serde(rename = "scheduleStartTime")]
    pub r#schedule_start_time: Option<String>,
    /// Time zone used when referring to the date and time of a scheduled-triggered flow, such as `America/New_York`.
    /// 
    /// ```yaml
    /// resources:
    ///   example:
    ///     type: aws:appflow:Flow
    ///     properties:
    ///       triggerConfig:
    ///         scheduled:
    ///           - scheduleExpression: rate(1minutes)
    /// ```
    #[builder(into)]
    #[serde(rename = "timezone")]
    pub r#timezone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlowTriggerConfigTriggerPropertiesScheduled {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("data_pull_mode".to_string(), self.r#data_pull_mode.to_pulumi_value().await);
            map.insert("first_execution_from".to_string(), self.r#first_execution_from.to_pulumi_value().await);
            map.insert("schedule_end_time".to_string(), self.r#schedule_end_time.to_pulumi_value().await);
            map.insert("schedule_expression".to_string(), self.r#schedule_expression.to_pulumi_value().await);
            map.insert("schedule_offset".to_string(), self.r#schedule_offset.to_pulumi_value().await);
            map.insert("schedule_start_time".to_string(), self.r#schedule_start_time.to_pulumi_value().await);
            map.insert("timezone".to_string(), self.r#timezone.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowTriggerConfigTriggerPropertiesScheduled {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#data_pull_mode: {
                        let field_value = match fields_map.get("data_pull_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_pull_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#first_execution_from: {
                        let field_value = match fields_map.get("first_execution_from") {
                            Some(value) => value,
                            None => bail!("Missing field 'first_execution_from' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#schedule_end_time: {
                        let field_value = match fields_map.get("schedule_end_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule_end_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#schedule_expression: {
                        let field_value = match fields_map.get("schedule_expression") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule_expression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#schedule_offset: {
                        let field_value = match fields_map.get("schedule_offset") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule_offset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#schedule_start_time: {
                        let field_value = match fields_map.get("schedule_start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule_start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timezone: {
                        let field_value = match fields_map.get("timezone") {
                            Some(value) => value,
                            None => bail!("Missing field 'timezone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
