#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudGatewayRouteConfigRoute {
    /// Specifies the classification tags which will be applied to methods in the generated OpenAPI documentation.
    #[builder(into)]
    #[serde(rename = "classificationTags")]
    pub r#classification_tags: Option<Vec<String>>,
    /// Specifies the description which will be applied to methods in the generated OpenAPI documentation.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Specifies a list of filters which are used to modify the request before sending it to the target endpoint, or the received response.
    #[builder(into)]
    #[serde(rename = "filters")]
    pub r#filters: Option<Vec<String>>,
    /// Specifies the route processing order.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: i32,
    /// Specifies a list of conditions to evaluate a route for each request. Each predicate may be evaluated against request headers and parameter values. All of the predicates associated with a route must evaluate to true for the route to be matched to the request.
    #[builder(into)]
    #[serde(rename = "predicates")]
    pub r#predicates: Option<Vec<String>>,
    /// Should the sso validation be enabled?
    #[builder(into)]
    #[serde(rename = "ssoValidationEnabled")]
    pub r#sso_validation_enabled: Option<bool>,
    /// Specifies the title which will be applied to methods in the generated OpenAPI documentation.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Should pass currently-authenticated user's identity token to application service?
    #[builder(into)]
    #[serde(rename = "tokenRelay")]
    pub r#token_relay: Option<bool>,
    /// Specifies the full uri which will override `appName`.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpringCloudGatewayRouteConfigRoute {
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
                "classification_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#classification_tags,
                )
                .await,
            );
            map.insert(
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "filters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filters,
                )
                .await,
            );
            map.insert(
                "order".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#order,
                )
                .await,
            );
            map.insert(
                "predicates".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#predicates,
                )
                .await,
            );
            map.insert(
                "sso_validation_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sso_validation_enabled,
                )
                .await,
            );
            map.insert(
                "title".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#title,
                )
                .await,
            );
            map.insert(
                "token_relay".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#token_relay,
                )
                .await,
            );
            map.insert(
                "uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#uri,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpringCloudGatewayRouteConfigRoute {
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
                    r#classification_tags: {
                        let field_value = match fields_map.get("classification_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'classification_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filters: {
                        let field_value = match fields_map.get("filters") {
                            Some(value) => value,
                            None => bail!("Missing field 'filters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#order: {
                        let field_value = match fields_map.get("order") {
                            Some(value) => value,
                            None => bail!("Missing field 'order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#predicates: {
                        let field_value = match fields_map.get("predicates") {
                            Some(value) => value,
                            None => bail!("Missing field 'predicates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sso_validation_enabled: {
                        let field_value = match fields_map.get("sso_validation_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'sso_validation_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#title: {
                        let field_value = match fields_map.get("title") {
                            Some(value) => value,
                            None => bail!("Missing field 'title' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_relay: {
                        let field_value = match fields_map.get("token_relay") {
                            Some(value) => value,
                            None => bail!("Missing field 'token_relay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uri: {
                        let field_value = match fields_map.get("uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
