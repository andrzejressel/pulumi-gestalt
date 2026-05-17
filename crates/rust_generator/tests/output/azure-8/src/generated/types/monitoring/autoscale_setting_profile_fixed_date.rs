#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutoscaleSettingProfileFixedDate {
    /// Specifies the end date for the profile, formatted as an RFC3339 date string.
    #[builder(into)]
    #[serde(rename = "end")]
    pub r#end: String,
    /// Specifies the start date for the profile, formatted as an RFC3339 date string.
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: String,
    /// The Time Zone of the `start` and `end` times. A list of [possible values can be found here](https://learn.microsoft.com/en-us/rest/api/monitor/autoscale-settings/create-or-update?view=rest-monitor-2022-10-01&tabs=HTTP#recurrentschedule). Defaults to `UTC`.
    #[builder(into)]
    #[serde(rename = "timezone")]
    pub r#timezone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AutoscaleSettingProfileFixedDate {
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
                "end".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#end,
                )
                .await,
            );
            map.insert(
                "start".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start,
                )
                .await,
            );
            map.insert(
                "timezone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timezone,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AutoscaleSettingProfileFixedDate {
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
                    r#end: {
                        let field_value = match fields_map.get("end") {
                            Some(value) => value,
                            None => bail!("Missing field 'end' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start: {
                        let field_value = match fields_map.get("start") {
                            Some(value) => value,
                            None => bail!("Missing field 'start' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timezone: {
                        let field_value = match fields_map.get("timezone") {
                            Some(value) => value,
                            None => bail!("Missing field 'timezone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
