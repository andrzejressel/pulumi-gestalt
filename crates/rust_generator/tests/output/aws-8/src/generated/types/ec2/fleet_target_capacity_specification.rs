#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetTargetCapacitySpecification {
    /// Default target capacity type. Valid values: `on-demand`, `spot`.
    #[builder(into)]
    #[serde(rename = "defaultTargetCapacityType")]
    pub r#default_target_capacity_type: String,
    /// The number of On-Demand units to request.
    #[builder(into)]
    #[serde(rename = "onDemandTargetCapacity")]
    pub r#on_demand_target_capacity: Option<i32>,
    /// The number of Spot units to request.
    #[builder(into)]
    #[serde(rename = "spotTargetCapacity")]
    pub r#spot_target_capacity: Option<i32>,
    /// The unit for the target capacity.
    /// If you specify `target_capacity_unit_type`, `instance_requirements` must be specified.
    #[builder(into)]
    #[serde(rename = "targetCapacityUnitType")]
    pub r#target_capacity_unit_type: Option<String>,
    /// The number of units to request, filled using `default_target_capacity_type`.
    #[builder(into)]
    #[serde(rename = "totalTargetCapacity")]
    pub r#total_target_capacity: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FleetTargetCapacitySpecification {
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
                "default_target_capacity_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_target_capacity_type,
                )
                .await,
            );
            map.insert(
                "on_demand_target_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#on_demand_target_capacity,
                )
                .await,
            );
            map.insert(
                "spot_target_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spot_target_capacity,
                )
                .await,
            );
            map.insert(
                "target_capacity_unit_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_capacity_unit_type,
                )
                .await,
            );
            map.insert(
                "total_target_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#total_target_capacity,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FleetTargetCapacitySpecification {
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
                    r#default_target_capacity_type: {
                        let field_value = match fields_map.get("default_target_capacity_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_target_capacity_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_demand_target_capacity: {
                        let field_value = match fields_map.get("on_demand_target_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_demand_target_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spot_target_capacity: {
                        let field_value = match fields_map.get("spot_target_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'spot_target_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_capacity_unit_type: {
                        let field_value = match fields_map.get("target_capacity_unit_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_capacity_unit_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#total_target_capacity: {
                        let field_value = match fields_map.get("total_target_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'total_target_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
