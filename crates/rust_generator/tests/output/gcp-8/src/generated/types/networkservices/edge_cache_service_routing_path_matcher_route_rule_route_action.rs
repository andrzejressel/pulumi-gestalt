#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleRouteAction {
    /// The policy to use for defining caching and signed request behaviour for requests that match this route.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cdnPolicy")]
    pub r#cdn_policy: Option<Box<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicy>>,
    /// CORSPolicy defines Cross-Origin-Resource-Sharing configuration, including which CORS response headers will be set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "corsPolicy")]
    pub r#cors_policy: Option<Box<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCorsPolicy>>,
    /// The URL rewrite configuration for requests that match this route.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "urlRewrite")]
    pub r#url_rewrite: Option<Box<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionUrlRewrite>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleRouteAction {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "cdn_policy",
                    &self.r#cdn_policy,
                ),
                to_pulumi_object_field(
                    "cors_policy",
                    &self.r#cors_policy,
                ),
                to_pulumi_object_field(
                    "url_rewrite",
                    &self.r#url_rewrite,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleRouteAction {
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
                    r#cdn_policy: {
                        let field_value = match fields_map.get("cdn_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'cdn_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cors_policy: {
                        let field_value = match fields_map.get("cors_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'cors_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_rewrite: {
                        let field_value = match fields_map.get("url_rewrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'url_rewrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
