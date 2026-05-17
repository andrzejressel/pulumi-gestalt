#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AzureClusterNetworking {
    /// The IP address range of the pods in this cluster, in CIDR notation (e.g. `10.96.0.0/14`). All pods in the cluster get assigned a unique RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creation.
    #[builder(into)]
    #[serde(rename = "podAddressCidrBlocks")]
    pub r#pod_address_cidr_blocks: Vec<String>,
    /// The IP address range for services in this cluster, in CIDR notation (e.g. `10.96.0.0/14`). All services in the cluster get assigned a unique RFC1918 IPv4 address from these ranges. Only a single range is supported. This field cannot be changed after creating a cluster.
    #[builder(into)]
    #[serde(rename = "serviceAddressCidrBlocks")]
    pub r#service_address_cidr_blocks: Vec<String>,
    /// The Azure Resource Manager (ARM) ID of the VNet associated with your cluster. All components in the cluster (i.e. control plane and node pools) run on a single VNet. Example: `/subscriptions/*/resourceGroups/*/providers/Microsoft.Network/virtualNetworks/*` This field cannot be changed after creation.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "virtualNetworkId")]
    pub r#virtual_network_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AzureClusterNetworking {
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
                "pod_address_cidr_blocks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pod_address_cidr_blocks,
                )
                .await,
            );
            map.insert(
                "service_address_cidr_blocks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_address_cidr_blocks,
                )
                .await,
            );
            map.insert(
                "virtual_network_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#virtual_network_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AzureClusterNetworking {
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
                    r#pod_address_cidr_blocks: {
                        let field_value = match fields_map.get("pod_address_cidr_blocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'pod_address_cidr_blocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_address_cidr_blocks: {
                        let field_value = match fields_map.get("service_address_cidr_blocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_address_cidr_blocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_network_id: {
                        let field_value = match fields_map.get("virtual_network_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_network_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
