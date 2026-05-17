#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BudgetThresholdRule {
    /// The type of basis used to determine if spend has passed
    /// the threshold.
    /// Default value is `CURRENT_SPEND`.
    /// Possible values are: `CURRENT_SPEND`, `FORECASTED_SPEND`.
    #[builder(into)]
    #[serde(rename = "spendBasis")]
    pub r#spend_basis: Option<String>,
    /// Send an alert when this threshold is exceeded. This is a
    /// 1.0-based percentage, so 0.5 = 50%. Must be >= 0.
    #[builder(into)]
    #[serde(rename = "thresholdPercent")]
    pub r#threshold_percent: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BudgetThresholdRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "spend_basis".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spend_basis,
                )
                .await,
            );
            map.insert(
                "threshold_percent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#threshold_percent,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BudgetThresholdRule {
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
                    r#spend_basis: {
                        let field_value = match fields_map.get("spend_basis") {
                            Some(value) => value,
                            None => bail!("Missing field 'spend_basis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threshold_percent: {
                        let field_value = match fields_map.get("threshold_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'threshold_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
