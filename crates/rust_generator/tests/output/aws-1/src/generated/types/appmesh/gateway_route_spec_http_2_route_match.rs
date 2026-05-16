#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GatewayRouteSpecHttp2RouteMatch {
    /// Client request headers to match on.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<super::super::types::appmesh::GatewayRouteSpecHttp2RouteMatchHeader>>,
    /// Host name to match on.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Option<Box<super::super::types::appmesh::GatewayRouteSpecHttp2RouteMatchHostname>>,
    /// Client request path to match on.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<Box<super::super::types::appmesh::GatewayRouteSpecHttp2RouteMatchPath>>,
    /// The port number to match from the request.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// Path to match requests with. This parameter must always start with `/`, which by itself matches all requests to the virtual service name.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// Client request query parameters to match on.
    #[builder(into)]
    #[serde(rename = "queryParameters")]
    pub r#query_parameters: Option<Vec<super::super::types::appmesh::GatewayRouteSpecHttp2RouteMatchQueryParameter>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GatewayRouteSpecHttp2RouteMatch {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("headers".to_string(), self.r#headers.to_pulumi_value().await);
            map.insert("hostname".to_string(), self.r#hostname.to_pulumi_value().await);
            map.insert("path".to_string(), self.r#path.to_pulumi_value().await);
            map.insert("port".to_string(), self.r#port.to_pulumi_value().await);
            map.insert("prefix".to_string(), self.r#prefix.to_pulumi_value().await);
            map.insert("query_parameters".to_string(), self.r#query_parameters.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GatewayRouteSpecHttp2RouteMatch {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#headers: {
                        let field_value = match fields_map.get("headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::appmesh::GatewayRouteSpecHttp2RouteMatchHeader>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hostname: {
                        let field_value = match fields_map.get("hostname") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::GatewayRouteSpecHttp2RouteMatchHostname>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::GatewayRouteSpecHttp2RouteMatchPath>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#prefix: {
                        let field_value = match fields_map.get("prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#query_parameters: {
                        let field_value = match fields_map.get("query_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::appmesh::GatewayRouteSpecHttp2RouteMatchQueryParameter>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
