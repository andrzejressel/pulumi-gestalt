#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GatewayRouteSpec {
    /// Specification of a gRPC gateway route.
    #[builder(into)]
    #[serde(rename = "grpcRoute")]
    pub r#grpc_route: Option<Box<super::super::types::appmesh::GatewayRouteSpecGrpcRoute>>,
    /// Specification of an HTTP/2 gateway route.
    #[builder(into)]
    #[serde(rename = "http2Route")]
    pub r#http_2_route: Option<Box<super::super::types::appmesh::GatewayRouteSpecHttp2Route>>,
    /// Specification of an HTTP gateway route.
    #[builder(into)]
    #[serde(rename = "httpRoute")]
    pub r#http_route: Option<Box<super::super::types::appmesh::GatewayRouteSpecHttpRoute>>,
    /// Priority for the gateway route, between `0` and `1000`.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GatewayRouteSpec {
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
                "grpc_route".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#grpc_route,
                )
                .await,
            );
            map.insert(
                "http_2_route".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_2_route,
                )
                .await,
            );
            map.insert(
                "http_route".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_route,
                )
                .await,
            );
            map.insert(
                "priority".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#priority,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GatewayRouteSpec {
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
