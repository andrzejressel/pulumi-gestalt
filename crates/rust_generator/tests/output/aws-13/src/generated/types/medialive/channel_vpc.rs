#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelVpc {
    #[builder(into)]
    pub r#availability_zones: Option<Vec<String>>,
    #[builder(into)]
    pub r#network_interface_ids: Option<Vec<String>>,
    /// List of public address allocation ids to associate with ENIs that will be created in Output VPC. Must specify one for SINGLE_PIPELINE, two for STANDARD channels.
    #[builder(into)]
    pub r#public_address_allocation_ids: Vec<String>,
    /// A list of up to 5 EC2 VPC security group IDs to attach to the Output VPC network interfaces. If none are specified then the VPC default security group will be used.
    #[builder(into)]
    pub r#security_group_ids: Option<Vec<String>>,
    /// A list of VPC subnet IDs from the same VPC. If STANDARD channel, subnet IDs must be mapped to two unique availability zones (AZ).
    #[builder(into)]
    pub r#subnet_ids: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelVpc {
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
                    "availabilityZones",
                    &self.r#availability_zones,
                ),
                to_pulumi_object_field(
                    "networkInterfaceIds",
                    &self.r#network_interface_ids,
                ),
                to_pulumi_object_field(
                    "publicAddressAllocationIds",
                    &self.r#public_address_allocation_ids,
                ),
                to_pulumi_object_field(
                    "securityGroupIds",
                    &self.r#security_group_ids,
                ),
                to_pulumi_object_field(
                    "subnetIds",
                    &self.r#subnet_ids,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelVpc {
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
                    r#availability_zones: {
                        let field_value = match fields_map.get("availabilityZones") {
                            Some(value) => value,
                            None => bail!("Missing field 'availabilityZones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_interface_ids: {
                        let field_value = match fields_map.get("networkInterfaceIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkInterfaceIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_address_allocation_ids: {
                        let field_value = match fields_map.get("publicAddressAllocationIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicAddressAllocationIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_group_ids: {
                        let field_value = match fields_map.get("securityGroupIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityGroupIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_ids: {
                        let field_value = match fields_map.get("subnetIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
