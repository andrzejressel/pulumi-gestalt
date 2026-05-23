#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicyAddSignatures {
    /// The actions to take to add signatures to responses.
    /// Each value may be one of: `GENERATE_COOKIE`, `GENERATE_TOKEN_HLS_COOKIELESS`, `PROPAGATE_TOKEN_HLS_COOKIELESS`.
    #[builder(into)]
    pub r#actions: String,
    /// The parameters to copy from the verified token to the generated token.
    /// Only the following parameters may be copied:
    /// * `PathGlobs`
    #[builder(into)]
    pub r#copied_parameters: Option<Vec<String>>,
    /// The keyset to use for signature generation.
    /// The following are both valid paths to an EdgeCacheKeyset resource:
    /// * `projects/project/locations/global/edgeCacheKeysets/yourKeyset`
    /// * `yourKeyset`
    /// This must be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified.  This field may not be specified otherwise.
    #[builder(into)]
    pub r#keyset: Option<String>,
    /// The query parameter in which to put the generated token.
    /// If not specified, defaults to `edge-cache-token`.
    /// If specified, the name must be 1-64 characters long and match the regular expression `a-zA-Z*` which means the first character must be a letter, and all following characters must be a dash, underscore, letter or digit.
    /// This field may only be set when the GENERATE_TOKEN_HLS_COOKIELESS or PROPAGATE_TOKEN_HLS_COOKIELESS actions are specified.
    #[builder(into)]
    pub r#token_query_parameter: Option<String>,
    /// The duration the token is valid starting from the moment the token is first generated.
    /// Defaults to `86400s` (1 day).
    /// The TTL must be >= 0 and <= 604,800 seconds (1 week).
    /// This field may only be specified when the GENERATE_COOKIE or GENERATE_TOKEN_HLS_COOKIELESS actions are specified.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    pub r#token_ttl: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicyAddSignatures {
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
                    "actions",
                    &self.r#actions,
                ),
                to_pulumi_object_field(
                    "copiedParameters",
                    &self.r#copied_parameters,
                ),
                to_pulumi_object_field(
                    "keyset",
                    &self.r#keyset,
                ),
                to_pulumi_object_field(
                    "tokenQueryParameter",
                    &self.r#token_query_parameter,
                ),
                to_pulumi_object_field(
                    "tokenTtl",
                    &self.r#token_ttl,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicyAddSignatures {
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
                    r#actions: {
                        let field_value = match fields_map.get("actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#copied_parameters: {
                        let field_value = match fields_map.get("copiedParameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'copiedParameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#keyset: {
                        let field_value = match fields_map.get("keyset") {
                            Some(value) => value,
                            None => bail!("Missing field 'keyset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_query_parameter: {
                        let field_value = match fields_map.get("tokenQueryParameter") {
                            Some(value) => value,
                            None => bail!("Missing field 'tokenQueryParameter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_ttl: {
                        let field_value = match fields_map.get("tokenTtl") {
                            Some(value) => value,
                            None => bail!("Missing field 'tokenTtl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
