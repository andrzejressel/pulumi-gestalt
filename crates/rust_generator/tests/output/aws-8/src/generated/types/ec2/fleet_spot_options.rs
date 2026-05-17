#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetSpotOptions {
    /// How to allocate the target capacity across the Spot pools. Valid values: `diversified`, `lowestPrice`, `capacity-optimized`, `capacity-optimized-prioritized` and `price-capacity-optimized`. Default: `lowestPrice`.
    #[builder(into)]
    #[serde(rename = "allocationStrategy")]
    pub r#allocation_strategy: Option<String>,
    /// Behavior when a Spot Instance is interrupted. Valid values: `hibernate`, `stop`, `terminate`. Default: `terminate`.
    #[builder(into)]
    #[serde(rename = "instanceInterruptionBehavior")]
    pub r#instance_interruption_behavior: Option<String>,
    /// Number of Spot pools across which to allocate your target Spot capacity. Valid only when Spot `allocation_strategy` is set to `lowestPrice`. Default: `1`.
    #[builder(into)]
    #[serde(rename = "instancePoolsToUseCount")]
    pub r#instance_pools_to_use_count: Option<i32>,
    /// Nested argument containing maintenance strategies for managing your Spot Instances that are at an elevated risk of being interrupted. Defined below.
    #[builder(into)]
    #[serde(rename = "maintenanceStrategies")]
    pub r#maintenance_strategies: Option<Box<super::super::types::ec2::FleetSpotOptionsMaintenanceStrategies>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FleetSpotOptions {
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
                "allocation_strategy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allocation_strategy,
                )
                .await,
            );
            map.insert(
                "instance_interruption_behavior".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_interruption_behavior,
                )
                .await,
            );
            map.insert(
                "instance_pools_to_use_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_pools_to_use_count,
                )
                .await,
            );
            map.insert(
                "maintenance_strategies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maintenance_strategies,
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
                        let field_value = match fields_map.get("allocation_strategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'allocation_strategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_interruption_behavior: {
                        let field_value = match fields_map.get("instance_interruption_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_interruption_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_pools_to_use_count: {
                        let field_value = match fields_map.get("instance_pools_to_use_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_pools_to_use_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maintenance_strategies: {
                        let field_value = match fields_map.get("maintenance_strategies") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenance_strategies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
