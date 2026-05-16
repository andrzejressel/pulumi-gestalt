#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualServiceSpecProvider {
    #[builder(into)]
    #[serde(rename = "virtualNodes")]
    pub r#virtual_nodes: Vec<super::super::types::appmesh::GetVirtualServiceSpecProviderVirtualNode>,
    #[builder(into)]
    #[serde(rename = "virtualRouters")]
    pub r#virtual_routers: Vec<super::super::types::appmesh::GetVirtualServiceSpecProviderVirtualRouter>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualServiceSpecProvider {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("virtual_nodes".to_string(), self.r#virtual_nodes.to_pulumi_value().await);
            map.insert("virtual_routers".to_string(), self.r#virtual_routers.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualServiceSpecProvider {
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
                    r#virtual_nodes: {
                        let field_value = match fields_map.get("virtual_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetVirtualServiceSpecProviderVirtualNode> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#virtual_routers: {
                        let field_value = match fields_map.get("virtual_routers") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_routers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetVirtualServiceSpecProviderVirtualRouter> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
