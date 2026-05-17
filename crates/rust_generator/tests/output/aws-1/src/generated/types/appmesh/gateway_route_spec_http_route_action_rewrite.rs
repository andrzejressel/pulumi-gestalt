#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GatewayRouteSpecHttpRouteActionRewrite {
    /// Host name to rewrite.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Option<Box<super::super::types::appmesh::GatewayRouteSpecHttpRouteActionRewriteHostname>>,
    /// Exact path to rewrite.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<Box<super::super::types::appmesh::GatewayRouteSpecHttpRouteActionRewritePath>>,
    /// Specified beginning characters to rewrite.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<Box<super::super::types::appmesh::GatewayRouteSpecHttpRouteActionRewritePrefix>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GatewayRouteSpecHttpRouteActionRewrite {
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
                "hostname".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hostname,
                )
                .await,
            );
            map.insert(
                "path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path,
                )
                .await,
            );
            map.insert(
                "prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#prefix,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GatewayRouteSpecHttpRouteActionRewrite {
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
                    r#hostname: {
                        let field_value = match fields_map.get("hostname") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix: {
                        let field_value = match fields_map.get("prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
