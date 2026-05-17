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
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "max_executions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_executions,
                )
                .await,
            );
            map.insert(
                "min_executions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_executions,
                )
                .await,
            );
            map.insert(
                "polling_interval_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#polling_interval_in_seconds,
                )
                .await,
            );
            map.insert(
                "rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rules,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
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
