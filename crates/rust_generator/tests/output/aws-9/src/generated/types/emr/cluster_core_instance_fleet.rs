#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterCoreInstanceFleet {
    /// ID of the cluster.
    #[builder(into)]
    pub r#id: Option<String>,
    /// Configuration block for instance fleet.
    #[builder(into)]
    pub r#instance_type_configs: Option<Vec<super::super::types::emr::ClusterCoreInstanceFleetInstanceTypeConfig>>,
    /// Configuration block for launch specification.
    #[builder(into)]
    pub r#launch_specifications: Option<Box<super::super::types::emr::ClusterCoreInstanceFleetLaunchSpecifications>>,
    /// Friendly name given to the instance fleet.
    #[builder(into)]
    pub r#name: Option<String>,
    #[builder(into)]
    pub r#provisioned_on_demand_capacity: Option<i32>,
    #[builder(into)]
    pub r#provisioned_spot_capacity: Option<i32>,
    /// The target capacity of On-Demand units for the instance fleet, which determines how many On-Demand instances to provision.
    #[builder(into)]
    pub r#target_on_demand_capacity: Option<i32>,
    /// Target capacity of Spot units for the instance fleet, which determines how many Spot instances to provision.
    #[builder(into)]
    pub r#target_spot_capacity: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterCoreInstanceFleet {
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
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "instanceTypeConfigs",
                    &self.r#instance_type_configs,
                ),
                to_pulumi_object_field(
                    "launchSpecifications",
                    &self.r#launch_specifications,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "provisionedOnDemandCapacity",
                    &self.r#provisioned_on_demand_capacity,
                ),
                to_pulumi_object_field(
                    "provisionedSpotCapacity",
                    &self.r#provisioned_spot_capacity,
                ),
                to_pulumi_object_field(
                    "targetOnDemandCapacity",
                    &self.r#target_on_demand_capacity,
                ),
                to_pulumi_object_field(
                    "targetSpotCapacity",
                    &self.r#target_spot_capacity,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterCoreInstanceFleet {
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
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_type_configs: {
                        let field_value = match fields_map.get("instanceTypeConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceTypeConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_specifications: {
                        let field_value = match fields_map.get("launchSpecifications") {
                            Some(value) => value,
                            None => bail!("Missing field 'launchSpecifications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioned_on_demand_capacity: {
                        let field_value = match fields_map.get("provisionedOnDemandCapacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisionedOnDemandCapacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioned_spot_capacity: {
                        let field_value = match fields_map.get("provisionedSpotCapacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisionedSpotCapacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_on_demand_capacity: {
                        let field_value = match fields_map.get("targetOnDemandCapacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetOnDemandCapacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_spot_capacity: {
                        let field_value = match fields_map.get("targetSpotCapacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetSpotCapacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
