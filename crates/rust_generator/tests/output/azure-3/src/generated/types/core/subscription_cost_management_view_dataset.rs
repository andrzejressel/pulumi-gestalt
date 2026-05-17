#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubscriptionCostManagementViewDataset {
    /// One or more `aggregation` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "aggregations")]
    pub r#aggregations: Vec<super::super::types::core::SubscriptionCostManagementViewDatasetAggregation>,
    /// The granularity of rows in the report. Possible values are `Daily` and `Monthly`.
    #[builder(into)]
    #[serde(rename = "granularity")]
    pub r#granularity: String,
    /// One or more `grouping` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "groupings")]
    pub r#groupings: Option<Vec<super::super::types::core::SubscriptionCostManagementViewDatasetGrouping>>,
    /// One or more `sorting` blocks as defined below, containing the order by expression to be used in the report
    #[builder(into)]
    #[serde(rename = "sortings")]
    pub r#sortings: Option<Vec<super::super::types::core::SubscriptionCostManagementViewDatasetSorting>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SubscriptionCostManagementViewDataset {
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
                "aggregations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aggregations,
                )
                .await,
            );
            map.insert(
                "granularity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#granularity,
                )
                .await,
            );
            map.insert(
                "groupings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#groupings,
                )
                .await,
            );
            map.insert(
                "sortings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sortings,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SubscriptionCostManagementViewDataset {
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
                    r#aggregations: {
                        let field_value = match fields_map.get("aggregations") {
                            Some(value) => value,
                            None => bail!("Missing field 'aggregations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#granularity: {
                        let field_value = match fields_map.get("granularity") {
                            Some(value) => value,
                            None => bail!("Missing field 'granularity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#groupings: {
                        let field_value = match fields_map.get("groupings") {
                            Some(value) => value,
                            None => bail!("Missing field 'groupings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sortings: {
                        let field_value = match fields_map.get("sortings") {
                            Some(value) => value,
                            None => bail!("Missing field 'sortings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
