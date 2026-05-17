#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InsightsReportConfigFrequencyOptions {
    /// The date to stop generating inventory reports. For example, {"day": 15, "month": 9, "year": 2022}.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "endDate")]
    pub r#end_date: Box<super::super::types::storage::InsightsReportConfigFrequencyOptionsEndDate>,
    /// The frequency in which inventory reports are generated. Values are DAILY or WEEKLY.
    /// Possible values are: `DAILY`, `WEEKLY`.
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: String,
    /// The date to start generating inventory reports. For example, {"day": 15, "month": 8, "year": 2022}.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "startDate")]
    pub r#start_date: Box<super::super::types::storage::InsightsReportConfigFrequencyOptionsStartDate>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InsightsReportConfigFrequencyOptions {
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
                "end_date".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#end_date,
                )
                .await,
            );
            map.insert(
                "frequency".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#frequency,
                )
                .await,
            );
            map.insert(
                "start_date".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start_date,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InsightsReportConfigFrequencyOptions {
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
                    r#end_date: {
                        let field_value = match fields_map.get("end_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'end_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frequency: {
                        let field_value = match fields_map.get("frequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'frequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_date: {
                        let field_value = match fields_map.get("start_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
