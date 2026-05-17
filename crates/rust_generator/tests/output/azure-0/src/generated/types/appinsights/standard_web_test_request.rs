#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StandardWebTestRequest {
    /// The WebTest request body.
    #[builder(into)]
    #[serde(rename = "body")]
    pub r#body: Option<String>,
    /// Should the following of redirects be enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "followRedirectsEnabled")]
    pub r#follow_redirects_enabled: Option<bool>,
    /// One or more `header` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<super::super::types::appinsights::StandardWebTestRequestHeader>>,
    /// Which HTTP verb to use for the call. Options are 'GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'HEAD', and 'OPTIONS'. Defaults to `GET`.
    #[builder(into)]
    #[serde(rename = "httpVerb")]
    pub r#http_verb: Option<String>,
    /// Should the parsing of dependend requests be enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "parseDependentRequestsEnabled")]
    pub r#parse_dependent_requests_enabled: Option<bool>,
    /// The WebTest request URL.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StandardWebTestRequest {
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
                "body".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#body,
                )
                .await,
            );
            map.insert(
                "follow_redirects_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#follow_redirects_enabled,
                )
                .await,
            );
            map.insert(
                "headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#headers,
                )
                .await,
            );
            map.insert(
                "http_verb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_verb,
                )
                .await,
            );
            map.insert(
                "parse_dependent_requests_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parse_dependent_requests_enabled,
                )
                .await,
            );
            map.insert(
                "url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#url,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StandardWebTestRequest {
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
                    r#body: {
                        let field_value = match fields_map.get("body") {
                            Some(value) => value,
                            None => bail!("Missing field 'body' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#follow_redirects_enabled: {
                        let field_value = match fields_map.get("follow_redirects_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'follow_redirects_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#headers: {
                        let field_value = match fields_map.get("headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_verb: {
                        let field_value = match fields_map.get("http_verb") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_verb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parse_dependent_requests_enabled: {
                        let field_value = match fields_map.get("parse_dependent_requests_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'parse_dependent_requests_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url: {
                        let field_value = match fields_map.get("url") {
                            Some(value) => value,
                            None => bail!("Missing field 'url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
