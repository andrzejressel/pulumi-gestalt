#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRouteTableRoutesRoute {
    /// The CIDR used for route destination matches.
    #[builder(into)]
    #[serde(rename = "destinationCidrBlock")]
    pub r#destination_cidr_block: String,
    /// The ID of the prefix list used for destination matches.
    #[builder(into)]
    #[serde(rename = "prefixListId")]
    pub r#prefix_list_id: String,
    /// The current state of the route, can be `active`, `deleted`, `pending`, `blackhole`, `deleting`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
    /// The id of the transit gateway route table announcement, most of the time it is an empty string.
    #[builder(into)]
    #[serde(rename = "transitGatewayRouteTableAnnouncementId")]
    pub r#transit_gateway_route_table_announcement_id: String,
    /// The type of the route, can be `propagated` or `static`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRouteTableRoutesRoute {
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
                    "destinationCidrBlock",
                    &self.r#destination_cidr_block,
                ),
                to_pulumi_object_field(
                    "prefixListId",
                    &self.r#prefix_list_id,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
                to_pulumi_object_field(
                    "transitGatewayRouteTableAnnouncementId",
                    &self.r#transit_gateway_route_table_announcement_id,
                ),
                to_pulumi_object_field(
                    "type",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRouteTableRoutesRoute {
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
                    r#destination_cidr_block: {
                        let field_value = match fields_map.get("destinationCidrBlock") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationCidrBlock' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_list_id: {
                        let field_value = match fields_map.get("prefixListId") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefixListId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transit_gateway_route_table_announcement_id: {
                        let field_value = match fields_map.get("transitGatewayRouteTableAnnouncementId") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitGatewayRouteTableAnnouncementId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type") {
                            Some(value) => value,
                            None => bail!("Missing field 'type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
