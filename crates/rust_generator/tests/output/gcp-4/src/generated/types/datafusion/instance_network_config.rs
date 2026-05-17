#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceNetworkConfig {
    /// Optional. Type of connection for establishing private IP connectivity between the Data Fusion customer project VPC and
    /// the corresponding tenant project from a predefined list of available connection modes.
    /// If this field is unspecified for a private instance, VPC peering is used.
    /// Possible values are: `VPC_PEERING`, `PRIVATE_SERVICE_CONNECT_INTERFACES`.
    #[builder(into)]
    #[serde(rename = "connectionType")]
    pub r#connection_type: Option<String>,
    /// The IP range in CIDR notation to use for the managed Data Fusion instance
    /// nodes. This range must not overlap with any other ranges used in the Data Fusion instance network.
    #[builder(into)]
    #[serde(rename = "ipAllocation")]
    pub r#ip_allocation: Option<String>,
    /// Name of the network in the project with which the tenant project
    /// will be peered for executing pipelines. In case of shared VPC where the network resides in another host
    /// project the network should specified in the form of projects/{host-project-id}/global/networks/{network}
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// Optional. Configuration for Private Service Connect.
    /// This is required only when using connection type PRIVATE_SERVICE_CONNECT_INTERFACES.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "privateServiceConnectConfig")]
    pub r#private_service_connect_config: Option<Box<super::super::types::datafusion::InstanceNetworkConfigPrivateServiceConnectConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceNetworkConfig {
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
                "connection_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection_type,
                )
                .await,
            );
            map.insert(
                "ip_allocation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_allocation,
                )
                .await,
            );
            map.insert(
                "network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network,
                )
                .await,
            );
            map.insert(
                "private_service_connect_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_service_connect_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceNetworkConfig {
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
                    r#connection_type: {
                        let field_value = match fields_map.get("connection_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_allocation: {
                        let field_value = match fields_map.get("ip_allocation") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_allocation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network: {
                        let field_value = match fields_map.get("network") {
                            Some(value) => value,
                            None => bail!("Missing field 'network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_service_connect_config: {
                        let field_value = match fields_map.get("private_service_connect_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_service_connect_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
