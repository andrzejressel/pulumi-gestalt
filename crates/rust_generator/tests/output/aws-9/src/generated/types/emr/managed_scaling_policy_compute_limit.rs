#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagedScalingPolicyComputeLimit {
    /// The upper boundary of EC2 units. It is measured through VCPU cores or instances for instance groups and measured through units for instance fleets. Managed scaling activities are not allowed beyond this boundary. The limit only applies to the core and task nodes. The master node cannot be scaled after initial configuration.
    #[builder(into)]
    #[serde(rename = "maximumCapacityUnits")]
    pub r#maximum_capacity_units: i32,
    /// The upper boundary of EC2 units for core node type in a cluster. It is measured through VCPU cores or instances for instance groups and measured through units for instance fleets. The core units are not allowed to scale beyond this boundary. The parameter is used to split capacity allocation between core and task nodes.
    #[builder(into)]
    #[serde(rename = "maximumCoreCapacityUnits")]
    pub r#maximum_core_capacity_units: Option<i32>,
    /// The upper boundary of On-Demand EC2 units. It is measured through VCPU cores or instances for instance groups and measured through units for instance fleets. The On-Demand units are not allowed to scale beyond this boundary. The parameter is used to split capacity allocation between On-Demand and Spot instances.
    #[builder(into)]
    #[serde(rename = "maximumOndemandCapacityUnits")]
    pub r#maximum_ondemand_capacity_units: Option<i32>,
    /// The lower boundary of EC2 units. It is measured through VCPU cores or instances for instance groups and measured through units for instance fleets. Managed scaling activities are not allowed beyond this boundary. The limit only applies to the core and task nodes. The master node cannot be scaled after initial configuration.
    #[builder(into)]
    #[serde(rename = "minimumCapacityUnits")]
    pub r#minimum_capacity_units: i32,
    /// The unit type used for specifying a managed scaling policy. Valid Values: `InstanceFleetUnits` | `Instances` | `VCPU`
    #[builder(into)]
    #[serde(rename = "unitType")]
    pub r#unit_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ManagedScalingPolicyComputeLimit {
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
                "maximum_capacity_units".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_capacity_units,
                )
                .await,
            );
            map.insert(
                "maximum_core_capacity_units".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_core_capacity_units,
                )
                .await,
            );
            map.insert(
                "maximum_ondemand_capacity_units".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_ondemand_capacity_units,
                )
                .await,
            );
            map.insert(
                "minimum_capacity_units".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#minimum_capacity_units,
                )
                .await,
            );
            map.insert(
                "unit_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unit_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ManagedScalingPolicyComputeLimit {
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
                    r#maximum_capacity_units: {
                        let field_value = match fields_map.get("maximum_capacity_units") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_capacity_units' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_core_capacity_units: {
                        let field_value = match fields_map.get("maximum_core_capacity_units") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_core_capacity_units' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_ondemand_capacity_units: {
                        let field_value = match fields_map.get("maximum_ondemand_capacity_units") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_ondemand_capacity_units' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_capacity_units: {
                        let field_value = match fields_map.get("minimum_capacity_units") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_capacity_units' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unit_type: {
                        let field_value = match fields_map.get("unit_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'unit_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
