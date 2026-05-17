#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RouteSpec {
    /// GRPC routing information for the route.
    #[builder(into)]
    #[serde(rename = "grpcRoute")]
    pub r#grpc_route: Option<Box<super::super::types::appmesh::RouteSpecGrpcRoute>>,
    /// HTTP/2 routing information for the route.
    #[builder(into)]
    #[serde(rename = "http2Route")]
    pub r#http_2_route: Option<Box<super::super::types::appmesh::RouteSpecHttp2Route>>,
    /// HTTP routing information for the route.
    #[builder(into)]
    #[serde(rename = "httpRoute")]
    pub r#http_route: Option<Box<super::super::types::appmesh::RouteSpecHttpRoute>>,
    /// Priority for the route, between `0` and `1000`.
    /// Routes are matched based on the specified value, where `0` is the highest priority.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
    /// TCP routing information for the route.
    #[builder(into)]
    #[serde(rename = "tcpRoute")]
    pub r#tcp_route: Option<Box<super::super::types::appmesh::RouteSpecTcpRoute>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RouteSpec {
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
                    "grpc_route",
                    &self.r#grpc_route,
                ),
                to_pulumi_object_field(
                    "http_2_route",
                    &self.r#http_2_route,
                ),
                to_pulumi_object_field(
                    "http_route",
                    &self.r#http_route,
                ),
                to_pulumi_object_field(
                    "priority",
                    &self.r#priority,
                ),
                to_pulumi_object_field(
                    "tcp_route",
                    &self.r#tcp_route,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RouteSpec {
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
                    r#grpc_route: {
                        let field_value = match fields_map.get("grpc_route") {
                            Some(value) => value,
                            None => bail!("Missing field 'grpc_route' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_2_route: {
                        let field_value = match fields_map.get("http_2_route") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_2_route' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_route: {
                        let field_value = match fields_map.get("http_route") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_route' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#priority: {
                        let field_value = match fields_map.get("priority") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tcp_route: {
                        let field_value = match fields_map.get("tcp_route") {
                            Some(value) => value,
                            None => bail!("Missing field 'tcp_route' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
