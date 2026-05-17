#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LifecyclePolicyPolicyDetailsScheduleCrossRegionCopyRule {
    #[builder(into)]
    #[serde(rename = "cmkArn")]
    pub r#cmk_arn: Option<String>,
    #[builder(into)]
    #[serde(rename = "copyTags")]
    pub r#copy_tags: Option<bool>,
    #[builder(into)]
    #[serde(rename = "deprecateRule")]
    pub r#deprecate_rule: Option<Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleCrossRegionCopyRuleDeprecateRule>>,
    #[builder(into)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: bool,
    #[builder(into)]
    #[serde(rename = "retainRule")]
    pub r#retain_rule: Option<Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleCrossRegionCopyRuleRetainRule>>,
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LifecyclePolicyPolicyDetailsScheduleCrossRegionCopyRule {
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
                "cmk_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cmk_arn,
                )
                .await,
            );
            map.insert(
                "copy_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#copy_tags,
                )
                .await,
            );
            map.insert(
                "deprecate_rule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#deprecate_rule,
                )
                .await,
            );
            map.insert(
                "encrypted".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encrypted,
                )
                .await,
            );
            map.insert(
                "retain_rule".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retain_rule,
                )
                .await,
            );
            map.insert(
                "target".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LifecyclePolicyPolicyDetailsScheduleCrossRegionCopyRule {
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
                    r#cmk_arn: {
                        let field_value = match fields_map.get("cmk_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'cmk_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#copy_tags: {
                        let field_value = match fields_map.get("copy_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'copy_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deprecate_rule: {
                        let field_value = match fields_map.get("deprecate_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'deprecate_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encrypted: {
                        let field_value = match fields_map.get("encrypted") {
                            Some(value) => value,
                            None => bail!("Missing field 'encrypted' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retain_rule: {
                        let field_value = match fields_map.get("retain_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'retain_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target: {
                        let field_value = match fields_map.get("target") {
                            Some(value) => value,
                            None => bail!("Missing field 'target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
