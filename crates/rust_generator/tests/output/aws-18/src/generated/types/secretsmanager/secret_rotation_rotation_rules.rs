#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecretRotationRotationRules {
    /// Specifies the number of days between automatic scheduled rotations of the secret. Either `automatically_after_days` or `schedule_expression` must be specified.
    #[builder(into)]
    #[serde(rename = "automaticallyAfterDays")]
    pub r#automatically_after_days: Option<i32>,
    /// The length of the rotation window in hours. For example, `3h` for a three hour window.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Option<String>,
    /// A `cron()` or `rate()` expression that defines the schedule for rotating your secret. Either `automatically_after_days` or `schedule_expression` must be specified.
    #[builder(into)]
    #[serde(rename = "scheduleExpression")]
    pub r#schedule_expression: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SecretRotationRotationRules {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("automatically_after_days".to_string(), self.r#automatically_after_days.to_pulumi_value().await);
            map.insert("duration".to_string(), self.r#duration.to_pulumi_value().await);
            map.insert("schedule_expression".to_string(), self.r#schedule_expression.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SecretRotationRotationRules {
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
                    r#automatically_after_days: {
                        let field_value = match fields_map.get("automatically_after_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'automatically_after_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#duration: {
                        let field_value = match fields_map.get("duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#schedule_expression: {
                        let field_value = match fields_map.get("schedule_expression") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule_expression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
