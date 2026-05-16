#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContactsRotationRecurrenceShiftCoverageCoverageTime {
    /// (Required) The end time of the on-call shift. See Hand Off Time for more details.
    #[builder(into)]
    #[serde(rename = "end")]
    pub r#end: Option<Box<super::super::types::ssm::ContactsRotationRecurrenceShiftCoverageCoverageTimeEnd>>,
    /// (Required) The start time of the on-call shift. See Hand Off Time for more details.
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: Option<Box<super::super::types::ssm::ContactsRotationRecurrenceShiftCoverageCoverageTimeStart>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ContactsRotationRecurrenceShiftCoverageCoverageTime {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("end".to_string(), self.r#end.to_pulumi_value().await);
            map.insert("start".to_string(), self.r#start.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ContactsRotationRecurrenceShiftCoverageCoverageTime {
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
                    r#end: {
                        let field_value = match fields_map.get("end") {
                            Some(value) => value,
                            None => bail!("Missing field 'end' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::ssm::ContactsRotationRecurrenceShiftCoverageCoverageTimeEnd>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#start: {
                        let field_value = match fields_map.get("start") {
                            Some(value) => value,
                            None => bail!("Missing field 'start' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::ssm::ContactsRotationRecurrenceShiftCoverageCoverageTimeStart>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
