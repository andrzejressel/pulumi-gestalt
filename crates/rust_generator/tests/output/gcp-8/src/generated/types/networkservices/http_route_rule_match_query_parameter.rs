#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HttpRouteRuleMatchQueryParameter {
    /// The value of the query parameter must exactly match the contents of exactMatch.
    #[builder(into)]
    #[serde(rename = "exactMatch")]
    pub r#exact_match: Option<String>,
    /// Specifies that the QueryParameterMatcher matches if request contains query parameter, irrespective of whether the parameter has a value or not.
    #[builder(into)]
    #[serde(rename = "presentMatch")]
    pub r#present_match: Option<bool>,
    /// The name of the query parameter to match.
    #[builder(into)]
    #[serde(rename = "queryParameter")]
    pub r#query_parameter: Option<String>,
    /// The value of the query parameter must match the regular expression specified by regexMatch.For regular expression grammar, please see https://github.com/google/re2/wiki/Syntax
    #[builder(into)]
    #[serde(rename = "regexMatch")]
    pub r#regex_match: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HttpRouteRuleMatchQueryParameter {
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
                "exact_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exact_match,
                )
                .await,
            );
            map.insert(
                "present_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#present_match,
                )
                .await,
            );
            map.insert(
                "query_parameter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query_parameter,
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
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HttpRouteRuleMatchQueryParameter {
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
                    r#exact_match: {
                        let field_value = match fields_map.get("exact_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'exact_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#present_match: {
                        let field_value = match fields_map.get("present_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'present_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#query_parameter: {
                        let field_value = match fields_map.get("query_parameter") {
                            Some(value) => value,
                            None => bail!("Missing field 'query_parameter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
