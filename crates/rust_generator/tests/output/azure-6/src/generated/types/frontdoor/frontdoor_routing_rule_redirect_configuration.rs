#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorRoutingRuleRedirectConfiguration {
    /// The destination fragment in the portion of URL after '#'. Set this to add a fragment to the redirect URL.
    #[builder(into)]
    #[serde(rename = "customFragment")]
    pub r#custom_fragment: Option<String>,
    /// Set this to change the URL for the redirection.
    #[builder(into)]
    #[serde(rename = "customHost")]
    pub r#custom_host: Option<String>,
    /// The path to retain as per the incoming request, or update in the URL for the redirection.
    #[builder(into)]
    #[serde(rename = "customPath")]
    pub r#custom_path: Option<String>,
    /// Replace any existing query string from the incoming request URL.
    #[builder(into)]
    #[serde(rename = "customQueryString")]
    pub r#custom_query_string: Option<String>,
    /// Protocol to use when redirecting. Valid options are `HttpOnly`, `HttpsOnly`, or `MatchRequest`.
    #[builder(into)]
    #[serde(rename = "redirectProtocol")]
    pub r#redirect_protocol: String,
    /// Status code for the redirect. Valida options are `Moved`, `Found`, `TemporaryRedirect`, `PermanentRedirect`.
    #[builder(into)]
    #[serde(rename = "redirectType")]
    pub r#redirect_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FrontdoorRoutingRuleRedirectConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "custom_fragment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_fragment,
                )
                .await,
            );
            map.insert(
                "custom_host".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_host,
                )
                .await,
            );
            map.insert(
                "custom_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_path,
                )
                .await,
            );
            map.insert(
                "custom_query_string".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_query_string,
                )
                .await,
            );
            map.insert(
                "redirect_protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#redirect_protocol,
                )
                .await,
            );
            map.insert(
                "redirect_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#redirect_type,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FrontdoorRoutingRuleRedirectConfiguration {
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
                    r#custom_fragment: {
                        let field_value = match fields_map.get("custom_fragment") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_fragment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_host: {
                        let field_value = match fields_map.get("custom_host") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_host' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_path: {
                        let field_value = match fields_map.get("custom_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_query_string: {
                        let field_value = match fields_map.get("custom_query_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_query_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_protocol: {
                        let field_value = match fields_map.get("redirect_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_type: {
                        let field_value = match fields_map.get("redirect_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
