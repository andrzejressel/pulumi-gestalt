#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RulesetRuleRatelimit {
    /// List of parameters that define how Cloudflare tracks the request rate for this rule.
    #[builder(into)]
    #[serde(rename = "characteristics")]
    pub r#characteristics: Option<Vec<String>>,
    /// Criteria for counting HTTP requests to trigger the Rate Limiting action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions.
    #[builder(into)]
    #[serde(rename = "countingExpression")]
    pub r#counting_expression: Option<String>,
    /// Once the request rate is reached, the Rate Limiting rule blocks further requests for the period of time defined in this field.
    #[builder(into)]
    #[serde(rename = "mitigationTimeout")]
    pub r#mitigation_timeout: Option<i32>,
    /// The period of time to consider (in seconds) when evaluating the request rate.
    #[builder(into)]
    #[serde(rename = "period")]
    pub r#period: Option<i32>,
    /// The number of requests over the period of time that will trigger the Rate Limiting rule.
    #[builder(into)]
    #[serde(rename = "requestsPerPeriod")]
    pub r#requests_per_period: Option<i32>,
    /// Whether to include requests to origin within the Rate Limiting count.
    #[builder(into)]
    #[serde(rename = "requestsToOrigin")]
    pub r#requests_to_origin: Option<bool>,
    /// The maximum aggregate score over the period of time that will trigger Rate Limiting rule.
    #[builder(into)]
    #[serde(rename = "scorePerPeriod")]
    pub r#score_per_period: Option<i32>,
    /// Name of HTTP header in the response, set by the origin server, with the score for the current request.
    #[builder(into)]
    #[serde(rename = "scoreResponseHeaderName")]
    pub r#score_response_header_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RulesetRuleRatelimit {
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
                "characteristics".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#characteristics,
                )
                .await,
            );
            map.insert(
                "counting_expression".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#counting_expression,
                )
                .await,
            );
            map.insert(
                "mitigation_timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mitigation_timeout,
                )
                .await,
            );
            map.insert(
                "period".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#period,
                )
                .await,
            );
            map.insert(
                "requests_per_period".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#requests_per_period,
                )
                .await,
            );
            map.insert(
                "requests_to_origin".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#requests_to_origin,
                )
                .await,
            );
            map.insert(
                "score_per_period".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#score_per_period,
                )
                .await,
            );
            map.insert(
                "score_response_header_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#score_response_header_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RulesetRuleRatelimit {
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
                    r#characteristics: {
                        let field_value = match fields_map.get("characteristics") {
                            Some(value) => value,
                            None => bail!("Missing field 'characteristics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#counting_expression: {
                        let field_value = match fields_map.get("counting_expression") {
                            Some(value) => value,
                            None => bail!("Missing field 'counting_expression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mitigation_timeout: {
                        let field_value = match fields_map.get("mitigation_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'mitigation_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#period: {
                        let field_value = match fields_map.get("period") {
                            Some(value) => value,
                            None => bail!("Missing field 'period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#requests_per_period: {
                        let field_value = match fields_map.get("requests_per_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'requests_per_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#requests_to_origin: {
                        let field_value = match fields_map.get("requests_to_origin") {
                            Some(value) => value,
                            None => bail!("Missing field 'requests_to_origin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#score_per_period: {
                        let field_value = match fields_map.get("score_per_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'score_per_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#score_response_header_name: {
                        let field_value = match fields_map.get("score_response_header_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'score_response_header_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
