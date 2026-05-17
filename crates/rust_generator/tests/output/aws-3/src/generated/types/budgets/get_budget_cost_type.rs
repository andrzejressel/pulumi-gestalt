#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBudgetCostType {
    /// A boolean value whether to include credits in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeCredit")]
    pub r#include_credit: bool,
    /// Whether a budget includes discounts. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeDiscount")]
    pub r#include_discount: bool,
    /// A boolean value whether to include other subscription costs in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeOtherSubscription")]
    pub r#include_other_subscription: bool,
    /// A boolean value whether to include recurring costs in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeRecurring")]
    pub r#include_recurring: bool,
    /// A boolean value whether to include refunds in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeRefund")]
    pub r#include_refund: bool,
    /// A boolean value whether to include subscriptions in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeSubscription")]
    pub r#include_subscription: bool,
    /// A boolean value whether to include support costs in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeSupport")]
    pub r#include_support: bool,
    /// A boolean value whether to include tax in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeTax")]
    pub r#include_tax: bool,
    /// A boolean value whether to include upfront costs in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeUpfront")]
    pub r#include_upfront: bool,
    /// Whether a budget uses the amortized rate. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "useAmortized")]
    pub r#use_amortized: bool,
    /// A boolean value whether to use blended costs in the cost budget. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "useBlended")]
    pub r#use_blended: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetBudgetCostType {
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
                "include_credit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_credit,
                )
                .await,
            );
            map.insert(
                "include_discount".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_discount,
                )
                .await,
            );
            map.insert(
                "include_other_subscription".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_other_subscription,
                )
                .await,
            );
            map.insert(
                "include_recurring".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_recurring,
                )
                .await,
            );
            map.insert(
                "include_refund".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_refund,
                )
                .await,
            );
            map.insert(
                "include_subscription".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_subscription,
                )
                .await,
            );
            map.insert(
                "include_support".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_support,
                )
                .await,
            );
            map.insert(
                "include_tax".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_tax,
                )
                .await,
            );
            map.insert(
                "include_upfront".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_upfront,
                )
                .await,
            );
            map.insert(
                "use_amortized".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_amortized,
                )
                .await,
            );
            map.insert(
                "use_blended".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_blended,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetBudgetCostType {
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
                    r#include_credit: {
                        let field_value = match fields_map.get("include_credit") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_credit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_discount: {
                        let field_value = match fields_map.get("include_discount") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_discount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_other_subscription: {
                        let field_value = match fields_map.get("include_other_subscription") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_other_subscription' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_recurring: {
                        let field_value = match fields_map.get("include_recurring") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_recurring' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_refund: {
                        let field_value = match fields_map.get("include_refund") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_refund' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_subscription: {
                        let field_value = match fields_map.get("include_subscription") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_subscription' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_support: {
                        let field_value = match fields_map.get("include_support") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_support' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_tax: {
                        let field_value = match fields_map.get("include_tax") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_tax' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_upfront: {
                        let field_value = match fields_map.get("include_upfront") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_upfront' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_amortized: {
                        let field_value = match fields_map.get("use_amortized") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_amortized' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_blended: {
                        let field_value = match fields_map.get("use_blended") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_blended' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
