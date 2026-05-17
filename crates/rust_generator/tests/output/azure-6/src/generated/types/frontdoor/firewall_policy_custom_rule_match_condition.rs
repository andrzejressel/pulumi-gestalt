#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirewallPolicyCustomRuleMatchCondition {
    /// Up to `600` possible values to match. Limit is in total across all `match_condition` blocks and `match_values` arguments. String value itself can be up to `256` characters long.
    #[builder(into)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Vec<String>,
    /// The request variable to compare with. Possible values are `Cookies`, `PostArgs`, `QueryString`, `RemoteAddr`, `RequestBody`, `RequestHeader`, `RequestMethod`, `RequestUri`, or `SocketAddr`.
    #[builder(into)]
    #[serde(rename = "matchVariable")]
    pub r#match_variable: String,
    /// Should the result of the condition be negated.
    #[builder(into)]
    #[serde(rename = "negationCondition")]
    pub r#negation_condition: Option<bool>,
    /// Comparison type to use for matching with the variable value. Possible values are `Any`, `BeginsWith`, `Contains`, `EndsWith`, `Equal`, `GeoMatch`, `GreaterThan`, `GreaterThanOrEqual`, `IPMatch`, `LessThan`, `LessThanOrEqual` or `RegEx`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
    /// Match against a specific key if the `match_variable` is `QueryString`, `PostArgs`, `RequestHeader` or `Cookies`.
    #[builder(into)]
    #[serde(rename = "selector")]
    pub r#selector: Option<String>,
    /// Up to `5` transforms to apply. Possible values are `Lowercase`, `RemoveNulls`, `Trim`, `Uppercase`, `URLDecode` or`URLEncode`.
    #[builder(into)]
    #[serde(rename = "transforms")]
    pub r#transforms: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirewallPolicyCustomRuleMatchCondition {
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
                "match_values".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#match_values,
                )
                .await,
            );
            map.insert(
                "match_variable".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#match_variable,
                )
                .await,
            );
            map.insert(
                "negation_condition".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#negation_condition,
                )
                .await,
            );
            map.insert(
                "operator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#operator,
                )
                .await,
            );
            map.insert(
                "selector".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#selector,
                )
                .await,
            );
            map.insert(
                "transforms".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transforms,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirewallPolicyCustomRuleMatchCondition {
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
                    r#match_values: {
                        let field_value = match fields_map.get("match_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'match_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#match_variable: {
                        let field_value = match fields_map.get("match_variable") {
                            Some(value) => value,
                            None => bail!("Missing field 'match_variable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#negation_condition: {
                        let field_value = match fields_map.get("negation_condition") {
                            Some(value) => value,
                            None => bail!("Missing field 'negation_condition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operator: {
                        let field_value = match fields_map.get("operator") {
                            Some(value) => value,
                            None => bail!("Missing field 'operator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#selector: {
                        let field_value = match fields_map.get("selector") {
                            Some(value) => value,
                            None => bail!("Missing field 'selector' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transforms: {
                        let field_value = match fields_map.get("transforms") {
                            Some(value) => value,
                            None => bail!("Missing field 'transforms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
