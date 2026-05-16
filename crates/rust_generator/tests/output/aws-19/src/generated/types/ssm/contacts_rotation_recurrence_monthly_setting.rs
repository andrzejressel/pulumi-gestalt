#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContactsRotationRecurrenceMonthlySetting {
    /// (Required) The day of the month when monthly recurring on-call rotations begin.
    #[builder(into)]
    #[serde(rename = "dayOfMonth")]
    pub r#day_of_month: i32,
    /// (Required) The hand off time. See Hand Off Time for more details.
    #[builder(into)]
    #[serde(rename = "handOffTime")]
    pub r#hand_off_time: Option<Box<super::super::types::ssm::ContactsRotationRecurrenceMonthlySettingHandOffTime>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ContactsRotationRecurrenceMonthlySetting {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("day_of_month".to_string(), self.r#day_of_month.to_pulumi_value().await);
            map.insert("hand_off_time".to_string(), self.r#hand_off_time.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ContactsRotationRecurrenceMonthlySetting {
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
                    r#day_of_month: {
                        let field_value = match fields_map.get("day_of_month") {
                            Some(value) => value,
                            None => bail!("Missing field 'day_of_month' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hand_off_time: {
                        let field_value = match fields_map.get("hand_off_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'hand_off_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::ssm::ContactsRotationRecurrenceMonthlySettingHandOffTime>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
