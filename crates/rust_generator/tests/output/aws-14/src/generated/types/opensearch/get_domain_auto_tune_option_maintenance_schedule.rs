#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDomainAutoTuneOptionMaintenanceSchedule {
    /// Cron expression for an Auto-Tune maintenance schedule.
    #[builder(into)]
    #[serde(rename = "cronExpressionForRecurrence")]
    pub r#cron_expression_for_recurrence: String,
    /// Configuration block for the duration of the Auto-Tune maintenance window.
    #[builder(into)]
    #[serde(rename = "durations")]
    pub r#durations: Vec<super::super::types::opensearch::GetDomainAutoTuneOptionMaintenanceScheduleDuration>,
    /// Date and time at which the Auto-Tune maintenance schedule starts in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
    #[builder(into)]
    #[serde(rename = "startAt")]
    pub r#start_at: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDomainAutoTuneOptionMaintenanceSchedule {
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
                "cron_expression_for_recurrence".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cron_expression_for_recurrence,
                )
                .await,
            );
            map.insert(
                "durations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#durations,
                )
                .await,
            );
            map.insert(
                "start_at".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start_at,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDomainAutoTuneOptionMaintenanceSchedule {
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
                    r#cron_expression_for_recurrence: {
                        let field_value = match fields_map.get("cron_expression_for_recurrence") {
                            Some(value) => value,
                            None => bail!("Missing field 'cron_expression_for_recurrence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#durations: {
                        let field_value = match fields_map.get("durations") {
                            Some(value) => value,
                            None => bail!("Missing field 'durations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_at: {
                        let field_value = match fields_map.get("start_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
