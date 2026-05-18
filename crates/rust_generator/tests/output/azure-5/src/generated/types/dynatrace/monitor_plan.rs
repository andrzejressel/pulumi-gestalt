#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MonitorPlan {
    /// Different billing cycles. Possible values are `MONTHLY` and `WEEKLY`.
    #[builder(into)]
    #[serde(rename = "billingCycle")]
    pub r#billing_cycle: Option<String>,
    /// Date when plan was applied.
    #[builder(into)]
    #[serde(rename = "effectiveDate")]
    pub r#effective_date: Option<String>,
    /// Plan id as published by Dynatrace.
    #[builder(into)]
    #[serde(rename = "plan")]
    pub r#plan: String,
    /// Different usage type. Possible values are `PAYG` and `COMMITTED`.
    #[builder(into)]
    #[serde(rename = "usageType")]
    pub r#usage_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MonitorPlan {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "billing_cycle",
                    &self.r#billing_cycle,
                ),
                to_pulumi_object_field(
                    "effective_date",
                    &self.r#effective_date,
                ),
                to_pulumi_object_field(
                    "plan",
                    &self.r#plan,
                ),
                to_pulumi_object_field(
                    "usage_type",
                    &self.r#usage_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MonitorPlan {
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
                    r#billing_cycle: {
                        let field_value = match fields_map.get("billing_cycle") {
                            Some(value) => value,
                            None => bail!("Missing field 'billing_cycle' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#effective_date: {
                        let field_value = match fields_map.get("effective_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'effective_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#plan: {
                        let field_value = match fields_map.get("plan") {
                            Some(value) => value,
                            None => bail!("Missing field 'plan' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#usage_type: {
                        let field_value = match fields_map.get("usage_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'usage_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
