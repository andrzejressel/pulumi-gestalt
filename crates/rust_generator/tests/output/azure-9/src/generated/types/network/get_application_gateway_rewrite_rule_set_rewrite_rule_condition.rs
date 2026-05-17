#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationGatewayRewriteRuleSetRewriteRuleCondition {
    /// Whether a case insensitive comparison is performed.
    #[builder(into)]
    #[serde(rename = "ignoreCase")]
    pub r#ignore_case: bool,
    /// Whether the result of the condition evaluation is negated.
    #[builder(into)]
    #[serde(rename = "negate")]
    pub r#negate: bool,
    /// The pattern, either fixed string or regular expression, that evaluates the truthfulness of the condition.
    #[builder(into)]
    #[serde(rename = "pattern")]
    pub r#pattern: String,
    /// The [variable](https://docs.microsoft.com/azure/application-gateway/rewrite-http-headers#server-variables) of the condition.
    #[builder(into)]
    #[serde(rename = "variable")]
    pub r#variable: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetApplicationGatewayRewriteRuleSetRewriteRuleCondition {
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
                "ignore_case".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ignore_case,
                )
                .await,
            );
            map.insert(
                "negate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#negate,
                )
                .await,
            );
            map.insert(
                "pattern".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pattern,
                )
                .await,
            );
            map.insert(
                "variable".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#variable,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetApplicationGatewayRewriteRuleSetRewriteRuleCondition {
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
                    r#ignore_case: {
                        let field_value = match fields_map.get("ignore_case") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_case' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#negate: {
                        let field_value = match fields_map.get("negate") {
                            Some(value) => value,
                            None => bail!("Missing field 'negate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pattern: {
                        let field_value = match fields_map.get("pattern") {
                            Some(value) => value,
                            None => bail!("Missing field 'pattern' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#variable: {
                        let field_value = match fields_map.get("variable") {
                            Some(value) => value,
                            None => bail!("Missing field 'variable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
