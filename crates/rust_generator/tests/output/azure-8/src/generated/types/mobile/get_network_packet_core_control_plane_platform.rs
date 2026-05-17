#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkPacketCoreControlPlanePlatform {
    /// The ID of Azure Arc connected cluster where the packet core is deployed.
    #[builder(into)]
    #[serde(rename = "arcKubernetesClusterId")]
    pub r#arc_kubernetes_cluster_id: String,
    /// The ID of Azure Arc custom location where the packet core is deployed.
    #[builder(into)]
    #[serde(rename = "customLocationId")]
    pub r#custom_location_id: String,
    /// The ID of Azure Stack Edge device where the packet core is deployed.
    #[builder(into)]
    #[serde(rename = "edgeDeviceId")]
    pub r#edge_device_id: String,
    /// The ID of Azure Stack HCI cluster where the packet core is deployed.
    #[builder(into)]
    #[serde(rename = "stackHciClusterId")]
    pub r#stack_hci_cluster_id: String,
    /// The platform type where the packet core is deployed.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNetworkPacketCoreControlPlanePlatform {
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
                "arc_kubernetes_cluster_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#arc_kubernetes_cluster_id,
                )
                .await,
            );
            map.insert(
                "custom_location_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_location_id,
                )
                .await,
            );
            map.insert(
                "edge_device_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#edge_device_id,
                )
                .await,
            );
            map.insert(
                "stack_hci_cluster_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#stack_hci_cluster_id,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetNetworkPacketCoreControlPlanePlatform {
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
                    r#arc_kubernetes_cluster_id: {
                        let field_value = match fields_map.get("arc_kubernetes_cluster_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'arc_kubernetes_cluster_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_location_id: {
                        let field_value = match fields_map.get("custom_location_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_location_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#edge_device_id: {
                        let field_value = match fields_map.get("edge_device_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'edge_device_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stack_hci_cluster_id: {
                        let field_value = match fields_map.get("stack_hci_cluster_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'stack_hci_cluster_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
