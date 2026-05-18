#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobEventTriggerConfigScale {
    /// Maximum number of job executions that are created for a trigger.
    #[builder(into)]
    #[serde(rename = "maxExecutions")]
    pub r#max_executions: Option<i32>,
    /// Minimum number of job executions that are created for a trigger.
    #[builder(into)]
    #[serde(rename = "minExecutions")]
    pub r#min_executions: Option<i32>,
    /// Interval to check each event source in seconds.
    #[builder(into)]
    #[serde(rename = "pollingIntervalInSeconds")]
    pub r#polling_interval_in_seconds: Option<i32>,
    /// A `rules` block as defined below.
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Option<Vec<super::super::types::containerapp::JobEventTriggerConfigScaleRule>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobEventTriggerConfigScale {
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
                    "max_executions",
                    &self.r#max_executions,
                ),
                to_pulumi_object_field(
                    "min_executions",
                    &self.r#min_executions,
                ),
                to_pulumi_object_field(
                    "polling_interval_in_seconds",
                    &self.r#polling_interval_in_seconds,
                ),
                to_pulumi_object_field(
                    "rules",
                    &self.r#rules,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobEventTriggerConfigScale {
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
                    r#max_executions: {
                        let field_value = match fields_map.get("max_executions") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_executions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_executions: {
                        let field_value = match fields_map.get("min_executions") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_executions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#polling_interval_in_seconds: {
                        let field_value = match fields_map.get("polling_interval_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'polling_interval_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rules: {
                        let field_value = match fields_map.get("rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
