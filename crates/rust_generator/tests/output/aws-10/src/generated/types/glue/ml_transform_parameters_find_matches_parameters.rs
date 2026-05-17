#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MlTransformParametersFindMatchesParameters {
    /// The value that is selected when tuning your transform for a balance between accuracy and cost.
    #[builder(into)]
    #[serde(rename = "accuracyCostTradeOff")]
    pub r#accuracy_cost_trade_off: Option<f64>,
    /// The value to switch on or off to force the output to match the provided labels from users.
    #[builder(into)]
    #[serde(rename = "enforceProvidedLabels")]
    pub r#enforce_provided_labels: Option<bool>,
    /// The value selected when tuning your transform for a balance between precision and recall.
    #[builder(into)]
    #[serde(rename = "precisionRecallTradeOff")]
    pub r#precision_recall_trade_off: Option<f64>,
    /// The name of a column that uniquely identifies rows in the source table.
    #[builder(into)]
    #[serde(rename = "primaryKeyColumnName")]
    pub r#primary_key_column_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MlTransformParametersFindMatchesParameters {
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
                "accuracy_cost_trade_off".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#accuracy_cost_trade_off,
                )
                .await,
            );
            map.insert(
                "enforce_provided_labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enforce_provided_labels,
                )
                .await,
            );
            map.insert(
                "precision_recall_trade_off".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#precision_recall_trade_off,
                )
                .await,
            );
            map.insert(
                "primary_key_column_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#primary_key_column_name,
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
                        let field_value = match fields_map.get("accuracy_cost_trade_off") {
                            Some(value) => value,
                            None => bail!("Missing field 'accuracy_cost_trade_off' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enforce_provided_labels: {
                        let field_value = match fields_map.get("enforce_provided_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforce_provided_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#precision_recall_trade_off: {
                        let field_value = match fields_map.get("precision_recall_trade_off") {
                            Some(value) => value,
                            None => bail!("Missing field 'precision_recall_trade_off' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary_key_column_name: {
                        let field_value = match fields_map.get("primary_key_column_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary_key_column_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
