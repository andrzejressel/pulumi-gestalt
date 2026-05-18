#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExpressRouteConnectionRouting {
    /// The ID of the Virtual Hub Route Table associated with this Express Route Connection.
    #[builder(into)]
    #[serde(rename = "associatedRouteTableId")]
    pub r#associated_route_table_id: Option<String>,
    /// The ID of the Route Map associated with this Express Route Connection for inbound routes.
    #[builder(into)]
    #[serde(rename = "inboundRouteMapId")]
    pub r#inbound_route_map_id: Option<String>,
    /// The ID of the Route Map associated with this Express Route Connection for outbound routes.
    #[builder(into)]
    #[serde(rename = "outboundRouteMapId")]
    pub r#outbound_route_map_id: Option<String>,
    /// A `propagated_route_table` block as defined below.
    #[builder(into)]
    #[serde(rename = "propagatedRouteTable")]
    pub r#propagated_route_table: Option<Box<super::super::types::network::ExpressRouteConnectionRoutingPropagatedRouteTable>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ExpressRouteConnectionRouting {
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
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ExpressRouteConnectionRouting {
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
