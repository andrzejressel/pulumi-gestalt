#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OntapVolumeAggregateConfiguration {
    /// Used to specify the names of the aggregates on which the volume will be created. Each aggregate needs to be in the format aggrX where X is the number of the aggregate.
    #[builder(into)]
    #[serde(rename = "aggregates")]
    pub r#aggregates: Option<Vec<String>>,
    /// Used to explicitly set the number of constituents within the FlexGroup per storage aggregate. the default value is `8`.
    #[builder(into)]
    #[serde(rename = "constituentsPerAggregate")]
    pub r#constituents_per_aggregate: Option<i32>,
    /// The total amount of constituents for a `FLEXGROUP` volume. This would equal constituents_per_aggregate x aggregates.
    #[builder(into)]
    #[serde(rename = "totalConstituents")]
    pub r#total_constituents: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OntapVolumeAggregateConfiguration {
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
                "aggregates".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aggregates,
                )
                .await,
            );
            map.insert(
                "constituents_per_aggregate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#constituents_per_aggregate,
                )
                .await,
            );
            map.insert(
                "total_constituents".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#total_constituents,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OntapVolumeAggregateConfiguration {
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
                    r#aggregates: {
                        let field_value = match fields_map.get("aggregates") {
                            Some(value) => value,
                            None => bail!("Missing field 'aggregates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#constituents_per_aggregate: {
                        let field_value = match fields_map.get("constituents_per_aggregate") {
                            Some(value) => value,
                            None => bail!("Missing field 'constituents_per_aggregate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#total_constituents: {
                        let field_value = match fields_map.get("total_constituents") {
                            Some(value) => value,
                            None => bail!("Missing field 'total_constituents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
