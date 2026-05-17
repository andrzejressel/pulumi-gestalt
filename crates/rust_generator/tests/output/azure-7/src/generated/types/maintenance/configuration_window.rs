#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigurationWindow {
    /// The duration of the maintenance window in HH:mm format.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Option<String>,
    /// Effective expiration date of the maintenance window in YYYY-MM-DD hh:mm format.
    #[builder(into)]
    #[serde(rename = "expirationDateTime")]
    pub r#expiration_date_time: Option<String>,
    /// The rate at which a maintenance window is expected to recur. The rate can be expressed as daily, weekly, or monthly schedules.
    #[builder(into)]
    #[serde(rename = "recurEvery")]
    pub r#recur_every: Option<String>,
    /// Effective start date of the maintenance window in YYYY-MM-DD hh:mm format.
    #[builder(into)]
    #[serde(rename = "startDateTime")]
    pub r#start_date_time: String,
    /// The time zone for the maintenance window. A list of timezones can be obtained by executing [System.TimeZoneInfo]::GetSystemTimeZones() in PowerShell.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConfigurationWindow {
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
                "duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#duration,
                )
                .await,
            );
            map.insert(
                "expiration_date_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expiration_date_time,
                )
                .await,
            );
            map.insert(
                "recur_every".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#recur_every,
                )
                .await,
            );
            map.insert(
                "start_date_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start_date_time,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConfigurationWindow {
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
                    r#duration: {
                        let field_value = match fields_map.get("duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expiration_date_time: {
                        let field_value = match fields_map.get("expiration_date_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'expiration_date_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recur_every: {
                        let field_value = match fields_map.get("recur_every") {
                            Some(value) => value,
                            None => bail!("Missing field 'recur_every' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_date_time: {
                        let field_value = match fields_map.get("start_date_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_date_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
