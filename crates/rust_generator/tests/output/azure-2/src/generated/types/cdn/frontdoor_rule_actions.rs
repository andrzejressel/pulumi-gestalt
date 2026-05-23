#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorRuleActions {
    /// A `request_header_action` block as defined below.
    #[builder(into)]
    pub r#request_header_actions: Option<Vec<super::super::types::cdn::FrontdoorRuleActionsRequestHeaderAction>>,
    /// A `response_header_action` block as defined below.
    #[builder(into)]
    pub r#response_header_actions: Option<Vec<super::super::types::cdn::FrontdoorRuleActionsResponseHeaderAction>>,
    /// A `route_configuration_override_action` block as defined below.
    #[builder(into)]
    pub r#route_configuration_override_action: Option<Box<super::super::types::cdn::FrontdoorRuleActionsRouteConfigurationOverrideAction>>,
    /// A `url_redirect_action` block as defined below. You may **not** have a `url_redirect_action` **and** a `url_rewrite_action` defined in the same `actions` block.
    #[builder(into)]
    pub r#url_redirect_action: Option<Box<super::super::types::cdn::FrontdoorRuleActionsUrlRedirectAction>>,
    /// A `url_rewrite_action` block as defined below. You may **not** have a `url_rewrite_action` **and** a `url_redirect_action` defined in the same `actions` block.
    #[builder(into)]
    pub r#url_rewrite_action: Option<Box<super::super::types::cdn::FrontdoorRuleActionsUrlRewriteAction>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorRuleActions {
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
                    "requestHeaderActions",
                    &self.r#request_header_actions,
                ),
                to_pulumi_object_field(
                    "responseHeaderActions",
                    &self.r#response_header_actions,
                ),
                to_pulumi_object_field(
                    "routeConfigurationOverrideAction",
                    &self.r#route_configuration_override_action,
                ),
                to_pulumi_object_field(
                    "urlRedirectAction",
                    &self.r#url_redirect_action,
                ),
                to_pulumi_object_field(
                    "urlRewriteAction",
                    &self.r#url_rewrite_action,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FrontdoorRuleActions {
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
                    r#request_header_actions: {
                        let field_value = match fields_map.get("requestHeaderActions") {
                            Some(value) => value,
                            None => bail!("Missing field 'requestHeaderActions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_header_actions: {
                        let field_value = match fields_map.get("responseHeaderActions") {
                            Some(value) => value,
                            None => bail!("Missing field 'responseHeaderActions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_configuration_override_action: {
                        let field_value = match fields_map.get("routeConfigurationOverrideAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'routeConfigurationOverrideAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_redirect_action: {
                        let field_value = match fields_map.get("urlRedirectAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'urlRedirectAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_rewrite_action: {
                        let field_value = match fields_map.get("urlRewriteAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'urlRewriteAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
