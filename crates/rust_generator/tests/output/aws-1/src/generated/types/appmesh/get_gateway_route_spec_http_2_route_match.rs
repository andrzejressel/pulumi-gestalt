#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGatewayRouteSpecHttp2RouteMatch {
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteMatchHeader>,
    #[builder(into)]
    #[serde(rename = "hostnames")]
    pub r#hostnames: Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteMatchHostname>,
    #[builder(into)]
    #[serde(rename = "paths")]
    pub r#paths: Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteMatchPath>,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: String,
    #[builder(into)]
    #[serde(rename = "queryParameters")]
    pub r#query_parameters: Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteMatchQueryParameter>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetGatewayRouteSpecHttp2RouteMatch {
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
            map.insert("hostnames".to_string(), self.r#hostnames.to_pulumi_value().await);
            map.insert("paths".to_string(), self.r#paths.to_pulumi_value().await);
            map.insert("port".to_string(), self.r#port.to_pulumi_value().await);
            map.insert("prefix".to_string(), self.r#prefix.to_pulumi_value().await);
            map.insert("query_parameters".to_string(), self.r#query_parameters.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetGatewayRouteSpecHttp2RouteMatch {
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
                        <Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteMatchHeader> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#hostnames: {
                        let field_value = match fields_map.get("hostnames") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostnames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteMatchHostname> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#paths: {
                        let field_value = match fields_map.get("paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteMatchPath> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#prefix: {
                        let field_value = match fields_map.get("prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#query_parameters: {
                        let field_value = match fields_map.get("query_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteMatchQueryParameter> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
