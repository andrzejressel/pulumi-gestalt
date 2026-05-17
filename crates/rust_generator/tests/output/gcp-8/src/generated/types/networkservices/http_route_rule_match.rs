#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HttpRouteRuleMatch {
    /// The HTTP request path value should exactly match this value.
    #[builder(into)]
    #[serde(rename = "fullPathMatch")]
    pub r#full_path_match: Option<String>,
    /// Specifies a list of HTTP request headers to match against.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<super::super::types::networkservices::HttpRouteRuleMatchHeader>>,
    /// Specifies if prefixMatch and fullPathMatch matches are case sensitive. The default value is false.
    #[builder(into)]
    #[serde(rename = "ignoreCase")]
    pub r#ignore_case: Option<bool>,
    /// The HTTP request path value must begin with specified prefixMatch. prefixMatch must begin with a /.
    #[builder(into)]
    #[serde(rename = "prefixMatch")]
    pub r#prefix_match: Option<String>,
    /// Specifies a list of query parameters to match against.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "queryParameters")]
    pub r#query_parameters: Option<Vec<super::super::types::networkservices::HttpRouteRuleMatchQueryParameter>>,
    /// The HTTP request path value must satisfy the regular expression specified by regexMatch after removing any query parameters and anchor supplied with the original URL. For regular expression grammar, please see https://github.com/google/re2/wiki/Syntax
    #[builder(into)]
    #[serde(rename = "regexMatch")]
    pub r#regex_match: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HttpRouteRuleMatch {
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
                "full_path_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#full_path_match,
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
                "ignore_case".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ignore_case,
                )
                .await,
            );
            map.insert(
                "prefix_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#prefix_match,
                )
                .await,
            );
            map.insert(
                "query_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_parameters,
                )
                .await,
            );
            map.insert(
                "regex_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#regex_match,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HttpRouteRuleMatch {
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
                    r#full_path_match: {
                        let field_value = match fields_map.get("full_path_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'full_path_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#ignore_case: {
                        let field_value = match fields_map.get("ignore_case") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_case' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_match: {
                        let field_value = match fields_map.get("prefix_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_parameters: {
                        let field_value = match fields_map.get("query_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regex_match: {
                        let field_value = match fields_map.get("regex_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'regex_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
