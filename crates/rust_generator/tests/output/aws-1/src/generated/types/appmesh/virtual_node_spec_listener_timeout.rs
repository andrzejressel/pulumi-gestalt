#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNodeSpecListenerTimeout {
    /// Timeouts for gRPC listeners.
    #[builder(into)]
    #[serde(rename = "grpc")]
    pub r#grpc: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutGrpc>>,
    /// Timeouts for HTTP listeners.
    #[builder(into)]
    #[serde(rename = "http")]
    pub r#http: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutHttp>>,
    /// Timeouts for HTTP2 listeners.
    #[builder(into)]
    #[serde(rename = "http2")]
    pub r#http_2: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutHttp2>>,
    /// Timeouts for TCP listeners.
    #[builder(into)]
    #[serde(rename = "tcp")]
    pub r#tcp: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutTcp>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualNodeSpecListenerTimeout {
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
            map.insert("tcp".to_string(), self.r#tcp.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualNodeSpecListenerTimeout {
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
                        <Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutGrpc>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#http: {
                        let field_value = match fields_map.get("http") {
                            Some(value) => value,
                            None => bail!("Missing field 'http' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutHttp>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#http_2: {
                        let field_value = match fields_map.get("http_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutHttp2>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tcp: {
                        let field_value = match fields_map.get("tcp") {
                            Some(value) => value,
                            None => bail!("Missing field 'tcp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutTcp>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
