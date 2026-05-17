#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SloWindowsBasedSliGoodTotalRatioThreshold {
    /// Basic SLI to evaluate to judge window quality.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "basicSliPerformance")]
    pub r#basic_sli_performance: Option<Box<super::super::types::monitoring::SloWindowsBasedSliGoodTotalRatioThresholdBasicSliPerformance>>,
    /// Request-based SLI to evaluate to judge window quality.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "performance")]
    pub r#performance: Option<Box<super::super::types::monitoring::SloWindowsBasedSliGoodTotalRatioThresholdPerformance>>,
    /// If window performance >= threshold, the window is counted
    /// as good.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SloWindowsBasedSliGoodTotalRatioThreshold {
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
                "basic_sli_performance".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#basic_sli_performance,
                )
                .await,
            );
            map.insert(
                "performance".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#performance,
                )
                .await,
            );
            map.insert(
                "threshold".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#threshold,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SloWindowsBasedSliGoodTotalRatioThreshold {
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
                    r#basic_sli_performance: {
                        let field_value = match fields_map.get("basic_sli_performance") {
                            Some(value) => value,
                            None => bail!("Missing field 'basic_sli_performance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#performance: {
                        let field_value = match fields_map.get("performance") {
                            Some(value) => value,
                            None => bail!("Missing field 'performance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threshold: {
                        let field_value = match fields_map.get("threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
