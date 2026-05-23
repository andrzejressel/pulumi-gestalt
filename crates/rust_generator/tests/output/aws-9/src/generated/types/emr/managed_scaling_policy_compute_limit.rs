#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagedScalingPolicyComputeLimit {
    /// The upper boundary of EC2 units. It is measured through VCPU cores or instances for instance groups and measured through units for instance fleets. Managed scaling activities are not allowed beyond this boundary. The limit only applies to the core and task nodes. The master node cannot be scaled after initial configuration.
    #[builder(into)]
    pub r#maximum_capacity_units: i32,
    /// The upper boundary of EC2 units for core node type in a cluster. It is measured through VCPU cores or instances for instance groups and measured through units for instance fleets. The core units are not allowed to scale beyond this boundary. The parameter is used to split capacity allocation between core and task nodes.
    #[builder(into)]
    pub r#maximum_core_capacity_units: Option<i32>,
    /// The upper boundary of On-Demand EC2 units. It is measured through VCPU cores or instances for instance groups and measured through units for instance fleets. The On-Demand units are not allowed to scale beyond this boundary. The parameter is used to split capacity allocation between On-Demand and Spot instances.
    #[builder(into)]
    pub r#maximum_ondemand_capacity_units: Option<i32>,
    /// The lower boundary of EC2 units. It is measured through VCPU cores or instances for instance groups and measured through units for instance fleets. Managed scaling activities are not allowed beyond this boundary. The limit only applies to the core and task nodes. The master node cannot be scaled after initial configuration.
    #[builder(into)]
    pub r#minimum_capacity_units: i32,
    /// The unit type used for specifying a managed scaling policy. Valid Values: `InstanceFleetUnits` | `Instances` | `VCPU`
    #[builder(into)]
    pub r#unit_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ManagedScalingPolicyComputeLimit {
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
                    "maximumCapacityUnits",
                    &self.r#maximum_capacity_units,
                ),
                to_pulumi_object_field(
                    "maximumCoreCapacityUnits",
                    &self.r#maximum_core_capacity_units,
                ),
                to_pulumi_object_field(
                    "maximumOndemandCapacityUnits",
                    &self.r#maximum_ondemand_capacity_units,
                ),
                to_pulumi_object_field(
                    "minimumCapacityUnits",
                    &self.r#minimum_capacity_units,
                ),
                to_pulumi_object_field(
                    "unitType",
                    &self.r#unit_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("maximumCapacityUnits") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximumCapacityUnits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_core_capacity_units: {
                        let field_value = match fields_map.get("maximumCoreCapacityUnits") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximumCoreCapacityUnits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_ondemand_capacity_units: {
                        let field_value = match fields_map.get("maximumOndemandCapacityUnits") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximumOndemandCapacityUnits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_capacity_units: {
                        let field_value = match fields_map.get("minimumCapacityUnits") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimumCapacityUnits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unit_type: {
                        let field_value = match fields_map.get("unitType") {
                            Some(value) => value,
                            None => bail!("Missing field 'unitType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
