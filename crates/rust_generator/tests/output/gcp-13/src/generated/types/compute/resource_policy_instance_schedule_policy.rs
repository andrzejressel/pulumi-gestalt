#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResourcePolicyInstanceSchedulePolicy {
    /// The expiration time of the schedule. The timestamp is an RFC3339 string.
    #[builder(into)]
    #[serde(rename = "expirationTime")]
    pub r#expiration_time: Option<String>,
    /// The start time of the schedule. The timestamp is an RFC3339 string.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
    /// Specifies the time zone to be used in interpreting the schedule. The value of this field must be a time zone name
    /// from the tz database: http://en.wikipedia.org/wiki/Tz_database.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: String,
    /// Specifies the schedule for starting instances.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vmStartSchedule")]
    pub r#vm_start_schedule: Option<Box<super::super::types::compute::ResourcePolicyInstanceSchedulePolicyVmStartSchedule>>,
    /// Specifies the schedule for stopping instances.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vmStopSchedule")]
    pub r#vm_stop_schedule: Option<Box<super::super::types::compute::ResourcePolicyInstanceSchedulePolicyVmStopSchedule>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResourcePolicyInstanceSchedulePolicy {
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
                "expiration_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expiration_time,
                )
                .await,
            );
            map.insert(
                "start_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start_time,
                )
                .await,
            );
            map.insert(
                "time_zone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_zone,
                )
                .await,
            );
            map.insert(
                "vm_start_schedule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vm_start_schedule,
                )
                .await,
            );
            map.insert(
                "vm_stop_schedule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vm_stop_schedule,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResourcePolicyInstanceSchedulePolicy {
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
                    r#expiration_time: {
                        let field_value = match fields_map.get("expiration_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'expiration_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_time: {
                        let field_value = match fields_map.get("start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_zone: {
                        let field_value = match fields_map.get("time_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_start_schedule: {
                        let field_value = match fields_map.get("vm_start_schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_start_schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_stop_schedule: {
                        let field_value = match fields_map.get("vm_stop_schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_stop_schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
