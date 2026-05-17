#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesCentralServer {
    /// The full name for the availability set. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "availabilitySetName")]
    pub r#availability_set_name: Option<String>,
    /// A `load_balancer` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "loadBalancer")]
    pub r#load_balancer: Option<Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesCentralServerLoadBalancer>>,
    /// One or more `virtual_machine` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "virtualMachines")]
    pub r#virtual_machines: Option<Vec<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesCentralServerVirtualMachine>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesCentralServer {
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
                "availability_set_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#availability_set_name,
                )
                .await,
            );
            map.insert(
                "load_balancer".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#load_balancer,
                )
                .await,
            );
            map.insert(
                "virtual_machines".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#virtual_machines,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesCentralServer {
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
                    r#availability_set_name: {
                        let field_value = match fields_map.get("availability_set_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'availability_set_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer: {
                        let field_value = match fields_map.get("load_balancer") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_machines: {
                        let field_value = match fields_map.get("virtual_machines") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_machines' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
