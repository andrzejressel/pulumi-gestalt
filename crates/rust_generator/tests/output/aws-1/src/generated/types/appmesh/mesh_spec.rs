#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MeshSpec {
    /// Egress filter rules for the service mesh.
    #[builder(into)]
    #[serde(rename = "egressFilter")]
    pub r#egress_filter: Option<Box<super::super::types::appmesh::MeshSpecEgressFilter>>,
    /// The service discovery information for the service mesh.
    #[builder(into)]
    #[serde(rename = "serviceDiscovery")]
    pub r#service_discovery: Option<Box<super::super::types::appmesh::MeshSpecServiceDiscovery>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MeshSpec {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("egress_filter".to_string(), self.r#egress_filter.to_pulumi_value().await);
            map.insert("service_discovery".to_string(), self.r#service_discovery.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MeshSpec {
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
                    r#egress_filter: {
                        let field_value = match fields_map.get("egress_filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'egress_filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::MeshSpecEgressFilter>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#service_discovery: {
                        let field_value = match fields_map.get("service_discovery") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_discovery' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::MeshSpecServiceDiscovery>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
