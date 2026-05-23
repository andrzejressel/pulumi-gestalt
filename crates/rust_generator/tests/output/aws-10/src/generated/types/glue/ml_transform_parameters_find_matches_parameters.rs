#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MlTransformParametersFindMatchesParameters {
    /// The value that is selected when tuning your transform for a balance between accuracy and cost.
    #[builder(into)]
    pub r#accuracy_cost_trade_off: Option<f64>,
    /// The value to switch on or off to force the output to match the provided labels from users.
    #[builder(into)]
    pub r#enforce_provided_labels: Option<bool>,
    /// The value selected when tuning your transform for a balance between precision and recall.
    #[builder(into)]
    pub r#precision_recall_trade_off: Option<f64>,
    /// The name of a column that uniquely identifies rows in the source table.
    #[builder(into)]
    pub r#primary_key_column_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MlTransformParametersFindMatchesParameters {
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
                    "accuracyCostTradeOff",
                    &self.r#accuracy_cost_trade_off,
                ),
                to_pulumi_object_field(
                    "enforceProvidedLabels",
                    &self.r#enforce_provided_labels,
                ),
                to_pulumi_object_field(
                    "precisionRecallTradeOff",
                    &self.r#precision_recall_trade_off,
                ),
                to_pulumi_object_field(
                    "primaryKeyColumnName",
                    &self.r#primary_key_column_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MlTransformParametersFindMatchesParameters {
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
                    r#accuracy_cost_trade_off: {
                        let field_value = match fields_map.get("accuracyCostTradeOff") {
                            Some(value) => value,
                            None => bail!("Missing field 'accuracyCostTradeOff' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enforce_provided_labels: {
                        let field_value = match fields_map.get("enforceProvidedLabels") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforceProvidedLabels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#precision_recall_trade_off: {
                        let field_value = match fields_map.get("precisionRecallTradeOff") {
                            Some(value) => value,
                            None => bail!("Missing field 'precisionRecallTradeOff' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary_key_column_name: {
                        let field_value = match fields_map.get("primaryKeyColumnName") {
                            Some(value) => value,
                            None => bail!("Missing field 'primaryKeyColumnName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
