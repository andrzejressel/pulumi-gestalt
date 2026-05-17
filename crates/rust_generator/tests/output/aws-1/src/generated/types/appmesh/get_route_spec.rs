#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRouteSpec {
    #[builder(into)]
    #[serde(rename = "grpcRoutes")]
    pub r#grpc_routes: Vec<super::super::types::appmesh::GetRouteSpecGrpcRoute>,
    #[builder(into)]
    #[serde(rename = "http2Routes")]
    pub r#http_2_routes: Vec<super::super::types::appmesh::GetRouteSpecHttp2Route>,
    #[builder(into)]
    #[serde(rename = "httpRoutes")]
    pub r#http_routes: Vec<super::super::types::appmesh::GetRouteSpecHttpRoute>,
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    #[builder(into)]
    #[serde(rename = "tcpRoutes")]
    pub r#tcp_routes: Vec<super::super::types::appmesh::GetRouteSpecTcpRoute>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRouteSpec {
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
                "grpc_routes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#grpc_routes,
                )
                .await,
            );
            map.insert(
                "http_2_routes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_2_routes,
                )
                .await,
            );
            map.insert(
                "http_routes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_routes,
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
            map.insert(
                "tcp_routes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tcp_routes,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRouteSpec {
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
                    r#grpc_routes: {
                        let field_value = match fields_map.get("grpc_routes") {
                            Some(value) => value,
                            None => bail!("Missing field 'grpc_routes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_2_routes: {
                        let field_value = match fields_map.get("http_2_routes") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_2_routes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_routes: {
                        let field_value = match fields_map.get("http_routes") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_routes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#tcp_routes: {
                        let field_value = match fields_map.get("tcp_routes") {
                            Some(value) => value,
                            None => bail!("Missing field 'tcp_routes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
