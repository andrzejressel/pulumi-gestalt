#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupMixedInstancesPolicyInstancesDistribution {
    /// Strategy to use when launching on-demand instances. Valid values: `prioritized`, `lowest-price`. Default: `prioritized`.
    #[builder(into)]
    #[serde(rename = "onDemandAllocationStrategy")]
    pub r#on_demand_allocation_strategy: Option<String>,
    /// Absolute minimum amount of desired capacity that must be fulfilled by on-demand instances. Default: `0`.
    #[builder(into)]
    #[serde(rename = "onDemandBaseCapacity")]
    pub r#on_demand_base_capacity: Option<i32>,
    /// Percentage split between on-demand and Spot instances above the base on-demand capacity. Default: `100`.
    #[builder(into)]
    #[serde(rename = "onDemandPercentageAboveBaseCapacity")]
    pub r#on_demand_percentage_above_base_capacity: Option<i32>,
    /// How to allocate capacity across the Spot pools. Valid values: `lowest-price`, `capacity-optimized`, `capacity-optimized-prioritized`, and `price-capacity-optimized`. Default: `lowest-price`.
    #[builder(into)]
    #[serde(rename = "spotAllocationStrategy")]
    pub r#spot_allocation_strategy: Option<String>,
    /// Number of Spot pools per availability zone to allocate capacity. EC2 Auto Scaling selects the cheapest Spot pools and evenly allocates Spot capacity across the number of Spot pools that you specify. Only available with `spot_allocation_strategy` set to `lowest-price`. Otherwise it must be set to `0`, if it has been defined before. Default: `2`.
    #[builder(into)]
    #[serde(rename = "spotInstancePools")]
    pub r#spot_instance_pools: Option<i32>,
    /// Maximum price per unit hour that the user is willing to pay for the Spot instances. Default: an empty string which means the on-demand price.
    #[builder(into)]
    #[serde(rename = "spotMaxPrice")]
    pub r#spot_max_price: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GroupMixedInstancesPolicyInstancesDistribution {
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
                "on_demand_allocation_strategy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#on_demand_allocation_strategy,
                )
                .await,
            );
            map.insert(
                "on_demand_base_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#on_demand_base_capacity,
                )
                .await,
            );
            map.insert(
                "on_demand_percentage_above_base_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#on_demand_percentage_above_base_capacity,
                )
                .await,
            );
            map.insert(
                "spot_allocation_strategy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spot_allocation_strategy,
                )
                .await,
            );
            map.insert(
                "spot_instance_pools".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spot_instance_pools,
                )
                .await,
            );
            map.insert(
                "spot_max_price".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spot_max_price,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GroupMixedInstancesPolicyInstancesDistribution {
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
                    r#on_demand_allocation_strategy: {
                        let field_value = match fields_map.get("on_demand_allocation_strategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_demand_allocation_strategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_demand_base_capacity: {
                        let field_value = match fields_map.get("on_demand_base_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_demand_base_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_demand_percentage_above_base_capacity: {
                        let field_value = match fields_map.get("on_demand_percentage_above_base_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_demand_percentage_above_base_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spot_allocation_strategy: {
                        let field_value = match fields_map.get("spot_allocation_strategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'spot_allocation_strategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spot_instance_pools: {
                        let field_value = match fields_map.get("spot_instance_pools") {
                            Some(value) => value,
                            None => bail!("Missing field 'spot_instance_pools' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spot_max_price: {
                        let field_value = match fields_map.get("spot_max_price") {
                            Some(value) => value,
                            None => bail!("Missing field 'spot_max_price' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
