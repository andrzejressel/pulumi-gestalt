#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkManagerConnectivityConfigurationAppliesToGroup {
    /// Whether global mesh is supported.
    #[builder(into)]
    #[serde(rename = "globalMeshEnabled")]
    pub r#global_mesh_enabled: bool,
    /// The group connectivity type.
    #[builder(into)]
    #[serde(rename = "groupConnectivity")]
    pub r#group_connectivity: String,
    /// The ID of the Network Manager Network Group.
    #[builder(into)]
    #[serde(rename = "networkGroupId")]
    pub r#network_group_id: String,
    /// Whether hub gateway is used.
    #[builder(into)]
    #[serde(rename = "useHubGateway")]
    pub r#use_hub_gateway: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNetworkManagerConnectivityConfigurationAppliesToGroup {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "global_mesh_enabled",
                    &self.r#global_mesh_enabled,
                ),
                to_pulumi_object_field(
                    "group_connectivity",
                    &self.r#group_connectivity,
                ),
                to_pulumi_object_field(
                    "network_group_id",
                    &self.r#network_group_id,
                ),
                to_pulumi_object_field(
                    "use_hub_gateway",
                    &self.r#use_hub_gateway,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetNetworkManagerConnectivityConfigurationAppliesToGroup {
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
                    r#global_mesh_enabled: {
                        let field_value = match fields_map.get("global_mesh_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'global_mesh_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group_connectivity: {
                        let field_value = match fields_map.get("group_connectivity") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_connectivity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_group_id: {
                        let field_value = match fields_map.get("network_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_hub_gateway: {
                        let field_value = match fields_map.get("use_hub_gateway") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_hub_gateway' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
