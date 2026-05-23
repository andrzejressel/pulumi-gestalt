#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NextGenerationFirewallVirtualNetworkPanoramaPanorama {
    /// The Device Group Name to which this Firewall Resource is registered.
    #[builder(into)]
    pub r#device_group_name: Option<String>,
    /// The Host Name of this Firewall Resource.
    #[builder(into)]
    pub r#host_name: Option<String>,
    /// The name which should be used for this Palo Alto Next Generation Firewall Virtual Network Panorama. Changing this forces a new Palo Alto Next Generation Firewall Virtual Network Panorama to be created.
    #[builder(into)]
    pub r#name: Option<String>,
    /// The name of the First Panorana server.
    #[builder(into)]
    pub r#panorama_server_1: Option<String>,
    /// The name of the Second Panorana server.
    #[builder(into)]
    pub r#panorama_server_2: Option<String>,
    /// The name of the Panorama Template applied to this Firewall Resource.
    #[builder(into)]
    pub r#template_name: Option<String>,
    /// The SSH Key to connect to the Firewall Resource.
    #[builder(into)]
    pub r#virtual_machine_ssh_key: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NextGenerationFirewallVirtualNetworkPanoramaPanorama {
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
                    "deviceGroupName",
                    &self.r#device_group_name,
                ),
                to_pulumi_object_field(
                    "hostName",
                    &self.r#host_name,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "panoramaServer1",
                    &self.r#panorama_server_1,
                ),
                to_pulumi_object_field(
                    "panoramaServer2",
                    &self.r#panorama_server_2,
                ),
                to_pulumi_object_field(
                    "templateName",
                    &self.r#template_name,
                ),
                to_pulumi_object_field(
                    "virtualMachineSshKey",
                    &self.r#virtual_machine_ssh_key,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NextGenerationFirewallVirtualNetworkPanoramaPanorama {
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
                    r#device_group_name: {
                        let field_value = match fields_map.get("deviceGroupName") {
                            Some(value) => value,
                            None => bail!("Missing field 'deviceGroupName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_name: {
                        let field_value = match fields_map.get("hostName") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#panorama_server_1: {
                        let field_value = match fields_map.get("panoramaServer1") {
                            Some(value) => value,
                            None => bail!("Missing field 'panoramaServer1' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#panorama_server_2: {
                        let field_value = match fields_map.get("panoramaServer2") {
                            Some(value) => value,
                            None => bail!("Missing field 'panoramaServer2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#template_name: {
                        let field_value = match fields_map.get("templateName") {
                            Some(value) => value,
                            None => bail!("Missing field 'templateName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_machine_ssh_key: {
                        let field_value = match fields_map.get("virtualMachineSshKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtualMachineSshKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
