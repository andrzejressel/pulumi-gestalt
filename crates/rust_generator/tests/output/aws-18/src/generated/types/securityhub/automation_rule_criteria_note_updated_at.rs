#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutomationRuleCriteriaNoteUpdatedAt {
    /// A configuration block of the date range for the date filter. See date_range below for more details.
    #[builder(into)]
    #[serde(rename = "dateRange")]
    pub r#date_range: Option<Box<super::super::types::securityhub::AutomationRuleCriteriaNoteUpdatedAtDateRange>>,
    /// An end date for the date filter. Required with `start` if `date_range` is not specified.
    #[builder(into)]
    #[serde(rename = "end")]
    pub r#end: Option<String>,
    /// A start date for the date filter. Required with `end` if `date_range` is not specified.
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AutomationRuleCriteriaNoteUpdatedAt {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("date_range".to_string(), self.r#date_range.to_pulumi_value().await);
            map.insert("end".to_string(), self.r#end.to_pulumi_value().await);
            map.insert("start".to_string(), self.r#start.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AutomationRuleCriteriaNoteUpdatedAt {
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
                    r#date_range: {
                        let field_value = match fields_map.get("date_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'date_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::securityhub::AutomationRuleCriteriaNoteUpdatedAtDateRange>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#end: {
                        let field_value = match fields_map.get("end") {
                            Some(value) => value,
                            None => bail!("Missing field 'end' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#start: {
                        let field_value = match fields_map.get("start") {
                            Some(value) => value,
                            None => bail!("Missing field 'start' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
