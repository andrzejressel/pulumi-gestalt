#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNodeSpecListener {
    /// Connection pool information for the listener.
    #[builder(into)]
    #[serde(rename = "connectionPool")]
    pub r#connection_pool: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerConnectionPool>>,
    /// Health check information for the listener.
    #[builder(into)]
    #[serde(rename = "healthCheck")]
    pub r#health_check: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerHealthCheck>>,
    /// Outlier detection information for the listener.
    #[builder(into)]
    #[serde(rename = "outlierDetection")]
    pub r#outlier_detection: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerOutlierDetection>>,
    /// Port mapping information for the listener.
    #[builder(into)]
    #[serde(rename = "portMapping")]
    pub r#port_mapping: Box<super::super::types::appmesh::VirtualNodeSpecListenerPortMapping>,
    /// Timeouts for different protocols.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeout>>,
    /// Transport Layer Security (TLS) properties for the listener
    #[builder(into)]
    #[serde(rename = "tls")]
    pub r#tls: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTls>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualNodeSpecListener {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("connection_pool".to_string(), self.r#connection_pool.to_pulumi_value().await);
            map.insert("health_check".to_string(), self.r#health_check.to_pulumi_value().await);
            map.insert("outlier_detection".to_string(), self.r#outlier_detection.to_pulumi_value().await);
            map.insert("port_mapping".to_string(), self.r#port_mapping.to_pulumi_value().await);
            map.insert("timeout".to_string(), self.r#timeout.to_pulumi_value().await);
            map.insert("tls".to_string(), self.r#tls.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualNodeSpecListener {
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
                    r#connection_pool: {
                        let field_value = match fields_map.get("connection_pool") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_pool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerConnectionPool>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#health_check: {
                        let field_value = match fields_map.get("health_check") {
                            Some(value) => value,
                            None => bail!("Missing field 'health_check' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerHealthCheck>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#outlier_detection: {
                        let field_value = match fields_map.get("outlier_detection") {
                            Some(value) => value,
                            None => bail!("Missing field 'outlier_detection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerOutlierDetection>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#port_mapping: {
                        let field_value = match fields_map.get("port_mapping") {
                            Some(value) => value,
                            None => bail!("Missing field 'port_mapping' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::appmesh::VirtualNodeSpecListenerPortMapping> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timeout: {
                        let field_value = match fields_map.get("timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeout>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tls: {
                        let field_value = match fields_map.get("tls") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTls>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
