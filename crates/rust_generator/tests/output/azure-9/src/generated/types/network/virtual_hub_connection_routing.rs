#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualHubConnectionRouting {
    /// The ID of the route table associated with this Virtual Hub connection.
    #[builder(into)]
    #[serde(rename = "associatedRouteTableId")]
    pub r#associated_route_table_id: Option<String>,
    /// The resource ID of the Route Map associated with this Routing Configuration for inbound learned routes.
    #[builder(into)]
    #[serde(rename = "inboundRouteMapId")]
    pub r#inbound_route_map_id: Option<String>,
    /// The resource ID of the Route Map associated with this Routing Configuration for outbound advertised routes.
    #[builder(into)]
    #[serde(rename = "outboundRouteMapId")]
    pub r#outbound_route_map_id: Option<String>,
    /// A `propagated_route_table` block as defined below.
    #[builder(into)]
    #[serde(rename = "propagatedRouteTable")]
    pub r#propagated_route_table: Option<Box<super::super::types::network::VirtualHubConnectionRoutingPropagatedRouteTable>>,
    /// The static VNet local route override criteria that is used to determine whether NVA in spoke VNet is bypassed for traffic with destination in spoke VNet. Possible values are `Contains` and `Equal`. Defaults to `Contains`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "staticVnetLocalRouteOverrideCriteria")]
    pub r#static_vnet_local_route_override_criteria: Option<String>,
    /// A `static_vnet_route` block as defined below.
    #[builder(into)]
    #[serde(rename = "staticVnetRoutes")]
    pub r#static_vnet_routes: Option<Vec<super::super::types::network::VirtualHubConnectionRoutingStaticVnetRoute>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualHubConnectionRouting {
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
                    "associated_route_table_id",
                    &self.r#associated_route_table_id,
                ),
                to_pulumi_object_field(
                    "inbound_route_map_id",
                    &self.r#inbound_route_map_id,
                ),
                to_pulumi_object_field(
                    "outbound_route_map_id",
                    &self.r#outbound_route_map_id,
                ),
                to_pulumi_object_field(
                    "propagated_route_table",
                    &self.r#propagated_route_table,
                ),
                to_pulumi_object_field(
                    "static_vnet_local_route_override_criteria",
                    &self.r#static_vnet_local_route_override_criteria,
                ),
                to_pulumi_object_field(
                    "static_vnet_routes",
                    &self.r#static_vnet_routes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualHubConnectionRouting {
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
                    r#associated_route_table_id: {
                        let field_value = match fields_map.get("associated_route_table_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'associated_route_table_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inbound_route_map_id: {
                        let field_value = match fields_map.get("inbound_route_map_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'inbound_route_map_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outbound_route_map_id: {
                        let field_value = match fields_map.get("outbound_route_map_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'outbound_route_map_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#propagated_route_table: {
                        let field_value = match fields_map.get("propagated_route_table") {
                            Some(value) => value,
                            None => bail!("Missing field 'propagated_route_table' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#static_vnet_local_route_override_criteria: {
                        let field_value = match fields_map.get("static_vnet_local_route_override_criteria") {
                            Some(value) => value,
                            None => bail!("Missing field 'static_vnet_local_route_override_criteria' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#static_vnet_routes: {
                        let field_value = match fields_map.get("static_vnet_routes") {
                            Some(value) => value,
                            None => bail!("Missing field 'static_vnet_routes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
