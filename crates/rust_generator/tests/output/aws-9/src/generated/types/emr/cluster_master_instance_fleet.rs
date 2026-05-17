#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterMasterInstanceFleet {
    /// ID of the cluster.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Configuration block for instance fleet.
    #[builder(into)]
    #[serde(rename = "instanceTypeConfigs")]
    pub r#instance_type_configs: Option<Vec<super::super::types::emr::ClusterMasterInstanceFleetInstanceTypeConfig>>,
    /// Configuration block for launch specification.
    #[builder(into)]
    #[serde(rename = "launchSpecifications")]
    pub r#launch_specifications: Option<Box<super::super::types::emr::ClusterMasterInstanceFleetLaunchSpecifications>>,
    /// Friendly name given to the instance fleet.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[builder(into)]
    #[serde(rename = "provisionedOnDemandCapacity")]
    pub r#provisioned_on_demand_capacity: Option<i32>,
    #[builder(into)]
    #[serde(rename = "provisionedSpotCapacity")]
    pub r#provisioned_spot_capacity: Option<i32>,
    /// Target capacity of On-Demand units for the instance fleet, which determines how many On-Demand instances to provision.
    #[builder(into)]
    #[serde(rename = "targetOnDemandCapacity")]
    pub r#target_on_demand_capacity: Option<i32>,
    /// Target capacity of Spot units for the instance fleet, which determines how many Spot instances to provision.
    #[builder(into)]
    #[serde(rename = "targetSpotCapacity")]
    pub r#target_spot_capacity: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterMasterInstanceFleet {
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
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "instance_type_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_type_configs,
                )
                .await,
            );
            map.insert(
                "launch_specifications".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#launch_specifications,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "provisioned_on_demand_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#provisioned_on_demand_capacity,
                )
                .await,
            );
            map.insert(
                "provisioned_spot_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#provisioned_spot_capacity,
                )
                .await,
            );
            map.insert(
                "target_on_demand_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_on_demand_capacity,
                )
                .await,
            );
            map.insert(
                "target_spot_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_spot_capacity,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterMasterInstanceFleet {
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
                        let field_value = match fields_map.get("instance_type_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_type_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#launch_specifications: {
                        let field_value = match fields_map.get("launch_specifications") {
                            Some(value) => value,
                            None => bail!("Missing field 'launch_specifications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("provisioned_on_demand_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioned_on_demand_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioned_spot_capacity: {
                        let field_value = match fields_map.get("provisioned_spot_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioned_spot_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_on_demand_capacity: {
                        let field_value = match fields_map.get("target_on_demand_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_on_demand_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_spot_capacity: {
                        let field_value = match fields_map.get("target_spot_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_spot_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
