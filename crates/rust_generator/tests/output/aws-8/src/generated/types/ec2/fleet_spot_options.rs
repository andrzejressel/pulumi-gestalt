#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetSpotOptions {
    /// How to allocate the target capacity across the Spot pools. Valid values: `diversified`, `lowestPrice`, `capacity-optimized`, `capacity-optimized-prioritized` and `price-capacity-optimized`. Default: `lowestPrice`.
    #[builder(into)]
    pub r#allocation_strategy: Option<String>,
    /// Behavior when a Spot Instance is interrupted. Valid values: `hibernate`, `stop`, `terminate`. Default: `terminate`.
    #[builder(into)]
    pub r#instance_interruption_behavior: Option<String>,
    /// Number of Spot pools across which to allocate your target Spot capacity. Valid only when Spot `allocation_strategy` is set to `lowestPrice`. Default: `1`.
    #[builder(into)]
    pub r#instance_pools_to_use_count: Option<i32>,
    /// Nested argument containing maintenance strategies for managing your Spot Instances that are at an elevated risk of being interrupted. Defined below.
    #[builder(into)]
    pub r#maintenance_strategies: Option<Box<super::super::types::ec2::FleetSpotOptionsMaintenanceStrategies>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FleetSpotOptions {
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
                    "allocationStrategy",
                    &self.r#allocation_strategy,
                ),
                to_pulumi_object_field(
                    "instanceInterruptionBehavior",
                    &self.r#instance_interruption_behavior,
                ),
                to_pulumi_object_field(
                    "instancePoolsToUseCount",
                    &self.r#instance_pools_to_use_count,
                ),
                to_pulumi_object_field(
                    "maintenanceStrategies",
                    &self.r#maintenance_strategies,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FleetSpotOptions {
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
                    r#allocation_strategy: {
                        let field_value = match fields_map.get("allocationStrategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocationStrategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_interruption_behavior: {
                        let field_value = match fields_map.get("instanceInterruptionBehavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceInterruptionBehavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_pools_to_use_count: {
                        let field_value = match fields_map.get("instancePoolsToUseCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'instancePoolsToUseCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maintenance_strategies: {
                        let field_value = match fields_map.get("maintenanceStrategies") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenanceStrategies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
