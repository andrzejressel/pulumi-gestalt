#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VmwareReplicatedVmNetworkInterface {
    /// Whether this `network_interface` is primary for the replicated VM.
    #[builder(into)]
    #[serde(rename = "isPrimary")]
    pub r#is_primary: bool,
    /// Mac address of the network interface of source VM.
    #[builder(into)]
    #[serde(rename = "sourceMacAddress")]
    pub r#source_mac_address: String,
    /// Static IP to assign when a failover is done.
    #[builder(into)]
    #[serde(rename = "targetStaticIp")]
    pub r#target_static_ip: Option<String>,
    /// Name of the subnet to use when a failover is done.
    #[builder(into)]
    #[serde(rename = "targetSubnetName")]
    pub r#target_subnet_name: Option<String>,
    /// Name of the subnet to use when a test failover is done.
    #[builder(into)]
    #[serde(rename = "testSubnetName")]
    pub r#test_subnet_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VmwareReplicatedVmNetworkInterface {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "is_primary",
                    &self.r#is_primary,
                ),
                to_pulumi_object_field(
                    "source_mac_address",
                    &self.r#source_mac_address,
                ),
                to_pulumi_object_field(
                    "target_static_ip",
                    &self.r#target_static_ip,
                ),
                to_pulumi_object_field(
                    "target_subnet_name",
                    &self.r#target_subnet_name,
                ),
                to_pulumi_object_field(
                    "test_subnet_name",
                    &self.r#test_subnet_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VmwareReplicatedVmNetworkInterface {
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
                    r#is_primary: {
                        let field_value = match fields_map.get("is_primary") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_primary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_mac_address: {
                        let field_value = match fields_map.get("source_mac_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_mac_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_static_ip: {
                        let field_value = match fields_map.get("target_static_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_static_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_subnet_name: {
                        let field_value = match fields_map.get("target_subnet_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_subnet_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#test_subnet_name: {
                        let field_value = match fields_map.get("test_subnet_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'test_subnet_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
