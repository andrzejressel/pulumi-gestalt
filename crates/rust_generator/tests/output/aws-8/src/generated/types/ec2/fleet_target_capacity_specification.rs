#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetTargetCapacitySpecification {
    /// Default target capacity type. Valid values: `on-demand`, `spot`.
    #[builder(into)]
    pub r#default_target_capacity_type: String,
    /// The number of On-Demand units to request.
    #[builder(into)]
    pub r#on_demand_target_capacity: Option<i32>,
    /// The number of Spot units to request.
    #[builder(into)]
    pub r#spot_target_capacity: Option<i32>,
    /// The unit for the target capacity.
    /// If you specify `target_capacity_unit_type`, `instance_requirements` must be specified.
    #[builder(into)]
    pub r#target_capacity_unit_type: Option<String>,
    /// The number of units to request, filled using `default_target_capacity_type`.
    #[builder(into)]
    pub r#total_target_capacity: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FleetTargetCapacitySpecification {
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
                    "defaultTargetCapacityType",
                    &self.r#default_target_capacity_type,
                ),
                to_pulumi_object_field(
                    "onDemandTargetCapacity",
                    &self.r#on_demand_target_capacity,
                ),
                to_pulumi_object_field(
                    "spotTargetCapacity",
                    &self.r#spot_target_capacity,
                ),
                to_pulumi_object_field(
                    "targetCapacityUnitType",
                    &self.r#target_capacity_unit_type,
                ),
                to_pulumi_object_field(
                    "totalTargetCapacity",
                    &self.r#total_target_capacity,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("defaultTargetCapacityType") {
                            Some(value) => value,
                            None => bail!("Missing field 'defaultTargetCapacityType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_demand_target_capacity: {
                        let field_value = match fields_map.get("onDemandTargetCapacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'onDemandTargetCapacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spot_target_capacity: {
                        let field_value = match fields_map.get("spotTargetCapacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'spotTargetCapacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_capacity_unit_type: {
                        let field_value = match fields_map.get("targetCapacityUnitType") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetCapacityUnitType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#total_target_capacity: {
                        let field_value = match fields_map.get("totalTargetCapacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'totalTargetCapacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
