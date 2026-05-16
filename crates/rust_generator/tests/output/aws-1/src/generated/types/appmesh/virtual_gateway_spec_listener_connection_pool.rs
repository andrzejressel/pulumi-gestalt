#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualGatewaySpecListenerConnectionPool {
    /// Connection pool information for gRPC listeners.
    #[builder(into)]
    #[serde(rename = "grpc")]
    pub r#grpc: Option<Box<super::super::types::appmesh::VirtualGatewaySpecListenerConnectionPoolGrpc>>,
    /// Connection pool information for HTTP listeners.
    #[builder(into)]
    #[serde(rename = "http")]
    pub r#http: Option<Box<super::super::types::appmesh::VirtualGatewaySpecListenerConnectionPoolHttp>>,
    /// Connection pool information for HTTP2 listeners.
    #[builder(into)]
    #[serde(rename = "http2")]
    pub r#http_2: Option<Box<super::super::types::appmesh::VirtualGatewaySpecListenerConnectionPoolHttp2>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualGatewaySpecListenerConnectionPool {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("grpc".to_string(), self.r#grpc.to_pulumi_value().await);
            map.insert("http".to_string(), self.r#http.to_pulumi_value().await);
            map.insert("http_2".to_string(), self.r#http_2.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualGatewaySpecListenerConnectionPool {
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
                    r#grpc: {
                        let field_value = match fields_map.get("grpc") {
                            Some(value) => value,
                            None => bail!("Missing field 'grpc' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualGatewaySpecListenerConnectionPoolGrpc>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#http: {
                        let field_value = match fields_map.get("http") {
                            Some(value) => value,
                            None => bail!("Missing field 'http' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualGatewaySpecListenerConnectionPoolHttp>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#http_2: {
                        let field_value = match fields_map.get("http_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualGatewaySpecListenerConnectionPoolHttp2>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
