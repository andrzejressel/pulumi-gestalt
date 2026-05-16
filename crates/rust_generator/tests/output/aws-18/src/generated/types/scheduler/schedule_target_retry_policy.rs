#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScheduleTargetRetryPolicy {
    /// Maximum amount of time, in seconds, to continue to make retry attempts. Ranges from `60` to `86400` (default).
    #[builder(into)]
    #[serde(rename = "maximumEventAgeInSeconds")]
    pub r#maximum_event_age_in_seconds: Option<i32>,
    /// Maximum number of retry attempts to make before the request fails. Ranges from `0` to `185` (default).
    #[builder(into)]
    #[serde(rename = "maximumRetryAttempts")]
    pub r#maximum_retry_attempts: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScheduleTargetRetryPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("maximum_event_age_in_seconds".to_string(), self.r#maximum_event_age_in_seconds.to_pulumi_value().await);
            map.insert("maximum_retry_attempts".to_string(), self.r#maximum_retry_attempts.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScheduleTargetRetryPolicy {
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
                    r#maximum_event_age_in_seconds: {
                        let field_value = match fields_map.get("maximum_event_age_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_event_age_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#maximum_retry_attempts: {
                        let field_value = match fields_map.get("maximum_retry_attempts") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_retry_attempts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
