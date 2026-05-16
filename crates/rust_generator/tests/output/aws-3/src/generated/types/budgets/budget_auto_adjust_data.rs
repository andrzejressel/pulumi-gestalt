#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BudgetAutoAdjustData {
    /// (Required) - The string that defines whether your budget auto-adjusts based on historical or forecasted data. Valid values: `FORECAST`,`HISTORICAL`
    #[builder(into)]
    #[serde(rename = "autoAdjustType")]
    pub r#auto_adjust_type: String,
    /// (Optional) - Configuration block of Historical Options. Required for `auto_adjust_type` of `HISTORICAL` Configuration block that defines the historical data that your auto-adjusting budget is based on.
    #[builder(into)]
    #[serde(rename = "historicalOptions")]
    pub r#historical_options: Option<Box<super::super::types::budgets::BudgetAutoAdjustDataHistoricalOptions>>,
    /// (Optional) - The last time that your budget was auto-adjusted.
    #[builder(into)]
    #[serde(rename = "lastAutoAdjustTime")]
    pub r#last_auto_adjust_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BudgetAutoAdjustData {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("auto_adjust_type".to_string(), self.r#auto_adjust_type.to_pulumi_value().await);
            map.insert("historical_options".to_string(), self.r#historical_options.to_pulumi_value().await);
            map.insert("last_auto_adjust_time".to_string(), self.r#last_auto_adjust_time.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BudgetAutoAdjustData {
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
                    r#auto_adjust_type: {
                        let field_value = match fields_map.get("auto_adjust_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_adjust_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#historical_options: {
                        let field_value = match fields_map.get("historical_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'historical_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::budgets::BudgetAutoAdjustDataHistoricalOptions>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#last_auto_adjust_time: {
                        let field_value = match fields_map.get("last_auto_adjust_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_auto_adjust_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
