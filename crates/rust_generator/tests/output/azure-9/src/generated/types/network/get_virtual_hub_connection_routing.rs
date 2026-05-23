#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualHubConnectionRouting {
    /// The ID of the route table associated with this Virtual Hub connection.
    #[builder(into)]
    #[serde(rename = "associatedRouteTableId")]
    pub r#associated_route_table_id: String,
    /// The ID of the Route Map associated with this Routing Configuration for inbound learned routes.
    #[builder(into)]
    #[serde(rename = "inboundRouteMapId")]
    pub r#inbound_route_map_id: String,
    /// The ID of the Route Map associated with this Routing Configuration for outbound advertised routes.
    #[builder(into)]
    #[serde(rename = "outboundRouteMapId")]
    pub r#outbound_route_map_id: String,
    /// A `propagated_route_table` block as defined below.
    #[builder(into)]
    #[serde(rename = "propagatedRouteTables")]
    pub r#propagated_route_tables: Vec<super::super::types::network::GetVirtualHubConnectionRoutingPropagatedRouteTable>,
    /// The static VNet local route override criteria that is used to determine whether NVA in spoke VNet is bypassed for traffic with destination in spoke VNet.
    #[builder(into)]
    #[serde(rename = "staticVnetLocalRouteOverrideCriteria")]
    pub r#static_vnet_local_route_override_criteria: String,
    /// A `static_vnet_route` block as defined below.
    #[builder(into)]
    #[serde(rename = "staticVnetRoutes")]
    pub r#static_vnet_routes: Vec<super::super::types::network::GetVirtualHubConnectionRoutingStaticVnetRoute>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualHubConnectionRouting {
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
                    "associatedRouteTableId",
                    &self.r#associated_route_table_id,
                ),
                to_pulumi_object_field(
                    "inboundRouteMapId",
                    &self.r#inbound_route_map_id,
                ),
                to_pulumi_object_field(
                    "outboundRouteMapId",
                    &self.r#outbound_route_map_id,
                ),
                to_pulumi_object_field(
                    "propagatedRouteTables",
                    &self.r#propagated_route_tables,
                ),
                to_pulumi_object_field(
                    "staticVnetLocalRouteOverrideCriteria",
                    &self.r#static_vnet_local_route_override_criteria,
                ),
                to_pulumi_object_field(
                    "staticVnetRoutes",
                    &self.r#static_vnet_routes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualHubConnectionRouting {
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
                        let field_value = match fields_map.get("associatedRouteTableId") {
                            Some(value) => value,
                            None => bail!("Missing field 'associatedRouteTableId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inbound_route_map_id: {
                        let field_value = match fields_map.get("inboundRouteMapId") {
                            Some(value) => value,
                            None => bail!("Missing field 'inboundRouteMapId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outbound_route_map_id: {
                        let field_value = match fields_map.get("outboundRouteMapId") {
                            Some(value) => value,
                            None => bail!("Missing field 'outboundRouteMapId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#propagated_route_tables: {
                        let field_value = match fields_map.get("propagatedRouteTables") {
                            Some(value) => value,
                            None => bail!("Missing field 'propagatedRouteTables' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#static_vnet_local_route_override_criteria: {
                        let field_value = match fields_map.get("staticVnetLocalRouteOverrideCriteria") {
                            Some(value) => value,
                            None => bail!("Missing field 'staticVnetLocalRouteOverrideCriteria' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#static_vnet_routes: {
                        let field_value = match fields_map.get("staticVnetRoutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'staticVnetRoutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
