#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig {
    /// Determines whether any cookies in viewer requests are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values are `none`, `whitelist`, `allExcept`, `all`.
    #[builder(into)]
    #[serde(rename = "cookieBehavior")]
    pub r#cookie_behavior: String,
    /// Object that contains a list of cookie names. See Items for more information.
    #[builder(into)]
    #[serde(rename = "cookies")]
    pub r#cookies: Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookie>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("cookie_behavior".to_string(), self.r#cookie_behavior.to_pulumi_value().await);
            map.insert("cookies".to_string(), self.r#cookies.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig {
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
                    r#cookie_behavior: {
                        let field_value = match fields_map.get("cookie_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookie_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cookies: {
                        let field_value = match fields_map.get("cookies") {
                            Some(value) => value,
                            None => bail!("Missing field 'cookies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookie> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
