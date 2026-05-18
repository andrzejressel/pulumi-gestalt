#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkInsightsAnalysisExplanationTransitGatewayRouteTableRoute {
    #[builder(into)]
    #[serde(rename = "attachmentId")]
    pub r#attachment_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "destinationCidr")]
    pub r#destination_cidr: Option<String>,
    #[builder(into)]
    #[serde(rename = "prefixListId")]
    pub r#prefix_list_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Option<String>,
    #[builder(into)]
    #[serde(rename = "routeOrigin")]
    pub r#route_origin: Option<String>,
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkInsightsAnalysisExplanationTransitGatewayRouteTableRoute {
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
                    "attachment_id",
                    &self.r#attachment_id,
                ),
                to_pulumi_object_field(
                    "destination_cidr",
                    &self.r#destination_cidr,
                ),
                to_pulumi_object_field(
                    "prefix_list_id",
                    &self.r#prefix_list_id,
                ),
                to_pulumi_object_field(
                    "resource_id",
                    &self.r#resource_id,
                ),
                to_pulumi_object_field(
                    "resource_type",
                    &self.r#resource_type,
                ),
                to_pulumi_object_field(
                    "route_origin",
                    &self.r#route_origin,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkInsightsAnalysisExplanationTransitGatewayRouteTableRoute {
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
                    r#attachment_id: {
                        let field_value = match fields_map.get("attachment_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'attachment_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_cidr: {
                        let field_value = match fields_map.get("destination_cidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_cidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_list_id: {
                        let field_value = match fields_map.get("prefix_list_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix_list_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_id: {
                        let field_value = match fields_map.get("resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_type: {
                        let field_value = match fields_map.get("resource_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_origin: {
                        let field_value = match fields_map.get("route_origin") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_origin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
