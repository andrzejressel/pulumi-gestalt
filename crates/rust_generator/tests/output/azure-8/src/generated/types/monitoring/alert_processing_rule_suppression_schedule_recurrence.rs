#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertProcessingRuleSuppressionScheduleRecurrence {
    /// One or more `daily` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "dailies")]
    pub r#dailies: Option<Vec<super::super::types::monitoring::AlertProcessingRuleSuppressionScheduleRecurrenceDaily>>,
    /// One or more `monthly` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "monthlies")]
    pub r#monthlies: Option<Vec<super::super::types::monitoring::AlertProcessingRuleSuppressionScheduleRecurrenceMonthly>>,
    /// One or more `weekly` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "weeklies")]
    pub r#weeklies: Option<Vec<super::super::types::monitoring::AlertProcessingRuleSuppressionScheduleRecurrenceWeekly>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AlertProcessingRuleSuppressionScheduleRecurrence {
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
                "dailies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dailies,
                )
                .await,
            );
            map.insert(
                "monthlies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#monthlies,
                )
                .await,
            );
            map.insert(
                "weeklies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#weeklies,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AlertProcessingRuleSuppressionScheduleRecurrence {
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
                    r#dailies: {
                        let field_value = match fields_map.get("dailies") {
                            Some(value) => value,
                            None => bail!("Missing field 'dailies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monthlies: {
                        let field_value = match fields_map.get("monthlies") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthlies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weeklies: {
                        let field_value = match fields_map.get("weeklies") {
                            Some(value) => value,
                            None => bail!("Missing field 'weeklies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
