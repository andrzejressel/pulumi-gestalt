#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BudgetAmountSpecifiedAmount {
    /// The 3-letter currency code defined in ISO 4217.
    #[builder(into)]
    #[serde(rename = "currencyCode")]
    pub r#currency_code: Option<String>,
    /// Number of nano (10^-9) units of the amount.
    /// The value must be between -999,999,999 and +999,999,999
    /// inclusive. If units is positive, nanos must be positive or
    /// zero. If units is zero, nanos can be positive, zero, or
    /// negative. If units is negative, nanos must be negative or
    /// zero. For example $-1.75 is represented as units=-1 and
    /// nanos=-750,000,000.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "nanos")]
    pub r#nanos: Option<i32>,
    /// The whole units of the amount. For example if currencyCode
    /// is "USD", then 1 unit is one US dollar.
    #[builder(into)]
    #[serde(rename = "units")]
    pub r#units: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BudgetAmountSpecifiedAmount {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "currency_code",
                    &self.r#currency_code,
                ),
                to_pulumi_object_field(
                    "nanos",
                    &self.r#nanos,
                ),
                to_pulumi_object_field(
                    "units",
                    &self.r#units,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BudgetAmountSpecifiedAmount {
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
                    r#currency_code: {
                        let field_value = match fields_map.get("currency_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'currency_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nanos: {
                        let field_value = match fields_map.get("nanos") {
                            Some(value) => value,
                            None => bail!("Missing field 'nanos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#units: {
                        let field_value = match fields_map.get("units") {
                            Some(value) => value,
                            None => bail!("Missing field 'units' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
