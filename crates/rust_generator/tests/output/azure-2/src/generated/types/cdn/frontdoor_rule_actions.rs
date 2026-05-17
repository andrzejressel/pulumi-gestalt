#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorRuleActions {
    /// A `request_header_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "requestHeaderActions")]
    pub r#request_header_actions: Option<Vec<super::super::types::cdn::FrontdoorRuleActionsRequestHeaderAction>>,
    /// A `response_header_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "responseHeaderActions")]
    pub r#response_header_actions: Option<Vec<super::super::types::cdn::FrontdoorRuleActionsResponseHeaderAction>>,
    /// A `route_configuration_override_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "routeConfigurationOverrideAction")]
    pub r#route_configuration_override_action: Option<Box<super::super::types::cdn::FrontdoorRuleActionsRouteConfigurationOverrideAction>>,
    /// A `url_redirect_action` block as defined below. You may **not** have a `url_redirect_action` **and** a `url_rewrite_action` defined in the same `actions` block.
    #[builder(into)]
    #[serde(rename = "urlRedirectAction")]
    pub r#url_redirect_action: Option<Box<super::super::types::cdn::FrontdoorRuleActionsUrlRedirectAction>>,
    /// A `url_rewrite_action` block as defined below. You may **not** have a `url_rewrite_action` **and** a `url_redirect_action` defined in the same `actions` block.
    #[builder(into)]
    #[serde(rename = "urlRewriteAction")]
    pub r#url_rewrite_action: Option<Box<super::super::types::cdn::FrontdoorRuleActionsUrlRewriteAction>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorRuleActions {
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
                "request_header_actions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_header_actions,
                )
                .await,
            );
            map.insert(
                "response_header_actions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#response_header_actions,
                )
                .await,
            );
            map.insert(
                "route_configuration_override_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#route_configuration_override_action,
                )
                .await,
            );
            map.insert(
                "url_redirect_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#url_redirect_action,
                )
                .await,
            );
            map.insert(
                "url_rewrite_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#url_rewrite_action,
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
                        let field_value = match fields_map.get("request_header_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_header_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_header_actions: {
                        let field_value = match fields_map.get("response_header_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_header_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_configuration_override_action: {
                        let field_value = match fields_map.get("route_configuration_override_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_configuration_override_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_redirect_action: {
                        let field_value = match fields_map.get("url_redirect_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'url_redirect_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url_rewrite_action: {
                        let field_value = match fields_map.get("url_rewrite_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'url_rewrite_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
