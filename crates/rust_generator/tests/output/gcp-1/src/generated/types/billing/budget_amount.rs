#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BudgetAmount {
    /// Configures a budget amount that is automatically set to 100% of
    /// last period's spend.
    /// Boolean. Set value to true to use. Do not set to false, instead
    /// use the `specified_amount` block.
    #[builder(into)]
    #[serde(rename = "lastPeriodAmount")]
    pub r#last_period_amount: Option<bool>,
    /// A specified amount to use as the budget. currencyCode is
    /// optional. If specified, it must match the currency of the
    /// billing account. The currencyCode is provided on output.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "specifiedAmount")]
    pub r#specified_amount: Option<Box<super::super::types::billing::BudgetAmountSpecifiedAmount>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BudgetAmount {
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
                "last_period_amount".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_period_amount,
                )
                .await,
            );
            map.insert(
                "specified_amount".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#specified_amount,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BudgetAmount {
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
                    r#last_period_amount: {
                        let field_value = match fields_map.get("last_period_amount") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_period_amount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#specified_amount: {
                        let field_value = match fields_map.get("specified_amount") {
                            Some(value) => value,
                            None => bail!("Missing field 'specified_amount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
