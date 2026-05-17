#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetHoursOfOperationConfig {
    /// Day that the hours of operation applies to.
    #[builder(into)]
    #[serde(rename = "day")]
    pub r#day: String,
    /// End time block specifies the time that your contact center closes. The `end_time` is documented below.
    #[builder(into)]
    #[serde(rename = "endTimes")]
    pub r#end_times: Vec<super::super::types::connect::GetHoursOfOperationConfigEndTime>,
    /// Start time block specifies the time that your contact center opens. The `start_time` is documented below.
    #[builder(into)]
    #[serde(rename = "startTimes")]
    pub r#start_times: Vec<super::super::types::connect::GetHoursOfOperationConfigStartTime>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetHoursOfOperationConfig {
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
                "day".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#day,
                )
                .await,
            );
            map.insert(
                "end_times".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#end_times,
                )
                .await,
            );
            map.insert(
                "start_times".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start_times,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetHoursOfOperationConfig {
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
                    r#day: {
                        let field_value = match fields_map.get("day") {
                            Some(value) => value,
                            None => bail!("Missing field 'day' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#end_times: {
                        let field_value = match fields_map.get("end_times") {
                            Some(value) => value,
                            None => bail!("Missing field 'end_times' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_times: {
                        let field_value = match fields_map.get("start_times") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_times' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
