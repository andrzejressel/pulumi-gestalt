#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetOnDemandOptions {
    /// The order of the launch template overrides to use in fulfilling On-Demand capacity. Valid values: `lowestPrice`, `prioritized`. Default: `lowestPrice`.
    #[builder(into)]
    #[serde(rename = "allocationStrategy")]
    pub r#allocation_strategy: Option<String>,
    /// The strategy for using unused Capacity Reservations for fulfilling On-Demand capacity. Supported only for fleets of type `instant`.
    #[builder(into)]
    #[serde(rename = "capacityReservationOptions")]
    pub r#capacity_reservation_options: Option<Box<super::super::types::ec2::FleetOnDemandOptionsCapacityReservationOptions>>,
    /// The maximum amount per hour for On-Demand Instances that you're willing to pay.
    #[builder(into)]
    #[serde(rename = "maxTotalPrice")]
    pub r#max_total_price: Option<String>,
    /// The minimum target capacity for On-Demand Instances in the fleet. If the minimum target capacity is not reached, the fleet launches no instances. Supported only for fleets of type `instant`.
    /// If you specify `min_target_capacity`, at least one of the following must be specified: `single_availability_zone` or `single_instance_type`.
    #[builder(into)]
    #[serde(rename = "minTargetCapacity")]
    pub r#min_target_capacity: Option<i32>,
    /// Indicates that the fleet launches all On-Demand Instances into a single Availability Zone. Supported only for fleets of type `instant`.
    #[builder(into)]
    #[serde(rename = "singleAvailabilityZone")]
    pub r#single_availability_zone: Option<bool>,
    /// Indicates that the fleet uses a single instance type to launch all On-Demand Instances in the fleet. Supported only for fleets of type `instant`.
    #[builder(into)]
    #[serde(rename = "singleInstanceType")]
    pub r#single_instance_type: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FleetOnDemandOptions {
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
                "allocation_strategy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allocation_strategy,
                )
                .await,
            );
            map.insert(
                "capacity_reservation_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#capacity_reservation_options,
                )
                .await,
            );
            map.insert(
                "max_total_price".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_total_price,
                )
                .await,
            );
            map.insert(
                "min_target_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_target_capacity,
                )
                .await,
            );
            map.insert(
                "single_availability_zone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#single_availability_zone,
                )
                .await,
            );
            map.insert(
                "single_instance_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#single_instance_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FleetOnDemandOptions {
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
                    r#capacity_reservation_options: {
                        let field_value = match fields_map.get("capacity_reservation_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacity_reservation_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_total_price: {
                        let field_value = match fields_map.get("max_total_price") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_total_price' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_target_capacity: {
                        let field_value = match fields_map.get("min_target_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_target_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#single_availability_zone: {
                        let field_value = match fields_map.get("single_availability_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'single_availability_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#single_instance_type: {
                        let field_value = match fields_map.get("single_instance_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'single_instance_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
