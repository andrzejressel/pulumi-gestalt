#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUse {
    #[builder(into)]
    #[serde(rename = "allow")]
    pub r#allow: Option<Box<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUseAllow>>,
    #[builder(into)]
    #[serde(rename = "block")]
    pub r#block: Option<Box<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUseBlock>>,
    /// Instructs AWS WAF to run a Captcha check against the web request. See `captcha` below for details.
    #[builder(into)]
    #[serde(rename = "captcha")]
    pub r#captcha: Option<Box<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUseCaptcha>>,
    /// Instructs AWS WAF to run a check against the request to verify that the request is coming from a legitimate client session. See `challenge` below for details.
    #[builder(into)]
    #[serde(rename = "challenge")]
    pub r#challenge: Option<Box<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUseChallenge>>,
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Option<Box<super::super::types::wafv2::WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUseCount>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUse {
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
                "allow".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow,
                )
                .await,
            );
            map.insert(
                "block".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#block,
                )
                .await,
            );
            map.insert(
                "captcha".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#captcha,
                )
                .await,
            );
            map.insert(
                "challenge".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#challenge,
                )
                .await,
            );
            map.insert(
                "count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#count,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclRuleStatementRuleGroupReferenceStatementRuleActionOverrideActionToUse {
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
                    r#allow: {
                        let field_value = match fields_map.get("allow") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#block: {
                        let field_value = match fields_map.get("block") {
                            Some(value) => value,
                            None => bail!("Missing field 'block' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#captcha: {
                        let field_value = match fields_map.get("captcha") {
                            Some(value) => value,
                            None => bail!("Missing field 'captcha' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#challenge: {
                        let field_value = match fields_map.get("challenge") {
                            Some(value) => value,
                            None => bail!("Missing field 'challenge' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#count: {
                        let field_value = match fields_map.get("count") {
                            Some(value) => value,
                            None => bail!("Missing field 'count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
