#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGatewayRouteSpecHttp2RouteAction {
    #[builder(into)]
    #[serde(rename = "rewrites")]
    pub r#rewrites: Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteActionRewrite>,
    #[builder(into)]
    #[serde(rename = "targets")]
    pub r#targets: Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteActionTarget>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetGatewayRouteSpecHttp2RouteAction {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("rewrites".to_string(), self.r#rewrites.to_pulumi_value().await);
            map.insert("targets".to_string(), self.r#targets.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetGatewayRouteSpecHttp2RouteAction {
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
                    r#rewrites: {
                        let field_value = match fields_map.get("rewrites") {
                            Some(value) => value,
                            None => bail!("Missing field 'rewrites' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteActionRewrite> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#targets: {
                        let field_value = match fields_map.get("targets") {
                            Some(value) => value,
                            None => bail!("Missing field 'targets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteActionTarget> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
