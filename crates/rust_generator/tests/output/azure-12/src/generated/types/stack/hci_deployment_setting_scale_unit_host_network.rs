#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HciDeploymentSettingScaleUnitHostNetwork {
    /// One or more `intent` blocks as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "intents")]
    pub r#intents: Vec<super::super::types::stack::HciDeploymentSettingScaleUnitHostNetworkIntent>,
    /// Whether allows users to specify IPs and Mask for Storage NICs when Network ATC is not assigning the IPs for storage automatically. Optional parameter required only for [3 nodes switchless deployments](https://learn.microsoft.com/azure-stack/hci/concepts/physical-network-requirements?tabs=overview%2C23H2reqs#using-switchless). Possible values are `true` and `false`. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "storageAutoIpEnabled")]
    pub r#storage_auto_ip_enabled: Option<bool>,
    /// Defines how the storage adapters between nodes are connected either switch or switch less. Possible values are `true` and `false`. Defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "storageConnectivitySwitchlessEnabled")]
    pub r#storage_connectivity_switchless_enabled: Option<bool>,
    /// One or more `storage_network` blocks as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "storageNetworks")]
    pub r#storage_networks: Vec<super::super::types::stack::HciDeploymentSettingScaleUnitHostNetworkStorageNetwork>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HciDeploymentSettingScaleUnitHostNetwork {
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
                    "intents",
                    &self.r#intents,
                ),
                to_pulumi_object_field(
                    "storage_auto_ip_enabled",
                    &self.r#storage_auto_ip_enabled,
                ),
                to_pulumi_object_field(
                    "storage_connectivity_switchless_enabled",
                    &self.r#storage_connectivity_switchless_enabled,
                ),
                to_pulumi_object_field(
                    "storage_networks",
                    &self.r#storage_networks,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HciDeploymentSettingScaleUnitHostNetwork {
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
                    r#intents: {
                        let field_value = match fields_map.get("intents") {
                            Some(value) => value,
                            None => bail!("Missing field 'intents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_auto_ip_enabled: {
                        let field_value = match fields_map.get("storage_auto_ip_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_auto_ip_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_connectivity_switchless_enabled: {
                        let field_value = match fields_map.get("storage_connectivity_switchless_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_connectivity_switchless_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_networks: {
                        let field_value = match fields_map.get("storage_networks") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_networks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
