#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionUrlRewrite {
    /// Prior to forwarding the request to the selected origin, the request's host header is replaced with contents of hostRewrite.
    #[builder(into)]
    #[serde(rename = "hostRewrite")]
    pub r#host_rewrite: Option<String>,
    /// Prior to forwarding the request to the selected origin, the matching portion of the request's path is replaced by pathPrefixRewrite.
    #[builder(into)]
    #[serde(rename = "pathPrefixRewrite")]
    pub r#path_prefix_rewrite: Option<String>,
    /// Prior to forwarding the request to the selected origin, if the
    /// request matched a pathTemplateMatch, the matching portion of the
    /// request's path is replaced re-written using the pattern specified
    /// by pathTemplateRewrite.
    /// pathTemplateRewrite must be between 1 and 255 characters
    /// (inclusive), must start with a '/', and must only use variables
    /// captured by the route's pathTemplate matchers.
    /// pathTemplateRewrite may only be used when all of a route's
    /// MatchRules specify pathTemplate.
    /// Only one of pathPrefixRewrite and pathTemplateRewrite may be
    /// specified.
    #[builder(into)]
    #[serde(rename = "pathTemplateRewrite")]
    pub r#path_template_rewrite: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionUrlRewrite {
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
                "host_rewrite".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_rewrite,
                )
                .await,
            );
            map.insert(
                "path_prefix_rewrite".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path_prefix_rewrite,
                )
                .await,
            );
            map.insert(
                "path_template_rewrite".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path_template_rewrite,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionUrlRewrite {
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
                    r#host_rewrite: {
                        let field_value = match fields_map.get("host_rewrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_rewrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path_prefix_rewrite: {
                        let field_value = match fields_map.get("path_prefix_rewrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'path_prefix_rewrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path_template_rewrite: {
                        let field_value = match fields_map.get("path_template_rewrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'path_template_rewrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
