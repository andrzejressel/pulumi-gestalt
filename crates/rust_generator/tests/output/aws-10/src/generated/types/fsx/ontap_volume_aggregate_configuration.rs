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

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("aggregates".to_string(), self.r#aggregates.to_pulumi_value().await);
            map.insert("constituents_per_aggregate".to_string(), self.r#constituents_per_aggregate.to_pulumi_value().await);
            map.insert("total_constituents".to_string(), self.r#total_constituents.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OntapVolumeAggregateConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#aggregates: {
                        let field_value = match fields_map.get("aggregates") {
                            Some(value) => value,
                            None => bail!("Missing field 'aggregates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#constituents_per_aggregate: {
                        let field_value = match fields_map.get("constituents_per_aggregate") {
                            Some(value) => value,
                            None => bail!("Missing field 'constituents_per_aggregate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#total_constituents: {
                        let field_value = match fields_map.get("total_constituents") {
                            Some(value) => value,
                            None => bail!("Missing field 'total_constituents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
