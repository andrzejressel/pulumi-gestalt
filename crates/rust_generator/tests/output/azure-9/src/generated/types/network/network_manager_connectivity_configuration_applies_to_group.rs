#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkManagerConnectivityConfigurationAppliesToGroup {
    /// Indicates whether to global mesh is supported for this group. Possible values are `true` and `false`.
    /// 
    /// > **NOTE:** A group can be global only if the `group_connectivity` is `DirectlyConnected`.
    #[builder(into)]
    pub r#global_mesh_enabled: Option<bool>,
    /// Specifies the group connectivity type. Possible values are `None` and `DirectlyConnected`.
    #[builder(into)]
    pub r#group_connectivity: String,
    /// Specifies the resource ID of Network Group which the configuration applies to.
    #[builder(into)]
    pub r#network_group_id: String,
    /// Indicates whether the hub gateway is used. Possible values are `true` and `false`.
    #[builder(into)]
    pub r#use_hub_gateway: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkManagerConnectivityConfigurationAppliesToGroup {
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
                    "globalMeshEnabled",
                    &self.r#global_mesh_enabled,
                ),
                to_pulumi_object_field(
                    "groupConnectivity",
                    &self.r#group_connectivity,
                ),
                to_pulumi_object_field(
                    "networkGroupId",
                    &self.r#network_group_id,
                ),
                to_pulumi_object_field(
                    "useHubGateway",
                    &self.r#use_hub_gateway,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkManagerConnectivityConfigurationAppliesToGroup {
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
                        let field_value = match fields_map.get("globalMeshEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'globalMeshEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group_connectivity: {
                        let field_value = match fields_map.get("groupConnectivity") {
                            Some(value) => value,
                            None => bail!("Missing field 'groupConnectivity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_group_id: {
                        let field_value = match fields_map.get("networkGroupId") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkGroupId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_hub_gateway: {
                        let field_value = match fields_map.get("useHubGateway") {
                            Some(value) => value,
                            None => bail!("Missing field 'useHubGateway' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
