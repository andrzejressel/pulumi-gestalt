#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGroupMixedInstancesPolicyInstancesDistribution {
    /// Strategy used when launching on-demand instances.
    #[builder(into)]
    #[serde(rename = "onDemandAllocationStrategy")]
    pub r#on_demand_allocation_strategy: String,
    /// Absolute minimum amount of desired capacity that must be fulfilled by on-demand instances.
    #[builder(into)]
    #[serde(rename = "onDemandBaseCapacity")]
    pub r#on_demand_base_capacity: i32,
    #[builder(into)]
    #[serde(rename = "onDemandPercentageAboveBaseCapacity")]
    pub r#on_demand_percentage_above_base_capacity: i32,
    /// Strategy used when launching Spot instances.
    #[builder(into)]
    #[serde(rename = "spotAllocationStrategy")]
    pub r#spot_allocation_strategy: String,
    /// Number of Spot pools per availability zone to allocate capacity.
    #[builder(into)]
    #[serde(rename = "spotInstancePools")]
    pub r#spot_instance_pools: i32,
    /// Maximum price per unit hour that the user is willing to pay for the Spot instances.
    #[builder(into)]
    #[serde(rename = "spotMaxPrice")]
    pub r#spot_max_price: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetGroupMixedInstancesPolicyInstancesDistribution {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "on_demand_allocation_strategy",
                    &self.r#on_demand_allocation_strategy,
                ),
                to_pulumi_object_field(
                    "on_demand_base_capacity",
                    &self.r#on_demand_base_capacity,
                ),
                to_pulumi_object_field(
                    "on_demand_percentage_above_base_capacity",
                    &self.r#on_demand_percentage_above_base_capacity,
                ),
                to_pulumi_object_field(
                    "spot_allocation_strategy",
                    &self.r#spot_allocation_strategy,
                ),
                to_pulumi_object_field(
                    "spot_instance_pools",
                    &self.r#spot_instance_pools,
                ),
                to_pulumi_object_field(
                    "spot_max_price",
                    &self.r#spot_max_price,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetGroupMixedInstancesPolicyInstancesDistribution {
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
