#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationGatewayRewriteRuleSetRewriteRule {
    /// One or more `condition` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Option<Vec<super::super::types::network::ApplicationGatewayRewriteRuleSetRewriteRuleCondition>>,
    /// Unique name of the rewrite rule block
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// One or more `request_header_configuration` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "requestHeaderConfigurations")]
    pub r#request_header_configurations: Option<Vec<super::super::types::network::ApplicationGatewayRewriteRuleSetRewriteRuleRequestHeaderConfiguration>>,
    /// One or more `response_header_configuration` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "responseHeaderConfigurations")]
    pub r#response_header_configurations: Option<Vec<super::super::types::network::ApplicationGatewayRewriteRuleSetRewriteRuleResponseHeaderConfiguration>>,
    /// Rule sequence of the rewrite rule that determines the order of execution in a set.
    #[builder(into)]
    #[serde(rename = "ruleSequence")]
    pub r#rule_sequence: i32,
    /// One `url` block as defined below
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<Box<super::super::types::network::ApplicationGatewayRewriteRuleSetRewriteRuleUrl>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationGatewayRewriteRuleSetRewriteRule {
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
                "conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#conditions,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "request_header_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_header_configurations,
                )
                .await,
            );
            map.insert(
                "response_header_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#response_header_configurations,
                )
                .await,
            );
            map.insert(
                "rule_sequence".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rule_sequence,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationGatewayRewriteRuleSetRewriteRule {
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
                    r#conditions: {
                        let field_value = match fields_map.get("conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request_header_configurations: {
                        let field_value = match fields_map.get("request_header_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_header_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_header_configurations: {
                        let field_value = match fields_map.get("response_header_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_header_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rule_sequence: {
                        let field_value = match fields_map.get("rule_sequence") {
                            Some(value) => value,
                            None => bail!("Missing field 'rule_sequence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
