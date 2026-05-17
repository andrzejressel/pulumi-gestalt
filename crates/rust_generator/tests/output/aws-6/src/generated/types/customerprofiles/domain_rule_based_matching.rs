#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainRuleBasedMatching {
    /// A block that configures information about the `AttributeTypesSelector` where the rule-based identity resolution uses to match profiles. Documented below.
    #[builder(into)]
    #[serde(rename = "attributeTypesSelector")]
    pub r#attribute_types_selector: Option<Box<super::super::types::customerprofiles::DomainRuleBasedMatchingAttributeTypesSelector>>,
    /// A block that specifies how the auto-merging process should resolve conflicts between different profiles. Documented below.
    #[builder(into)]
    #[serde(rename = "conflictResolution")]
    pub r#conflict_resolution: Option<Box<super::super::types::customerprofiles::DomainRuleBasedMatchingConflictResolution>>,
    /// The flag that enables the rule-based matching process of duplicate profiles.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// A block that specifies the configuration for exporting Identity Resolution results. Documented below.
    #[builder(into)]
    #[serde(rename = "exportingConfig")]
    pub r#exporting_config: Option<Box<super::super::types::customerprofiles::DomainRuleBasedMatchingExportingConfig>>,
    /// A block that configures how the rule-based matching process should match profiles. You can have up to 15 `rule` in the `natching_rules`. Documented below.
    #[builder(into)]
    #[serde(rename = "matchingRules")]
    pub r#matching_rules: Option<Vec<super::super::types::customerprofiles::DomainRuleBasedMatchingMatchingRule>>,
    /// Indicates the maximum allowed rule level for matching.
    #[builder(into)]
    #[serde(rename = "maxAllowedRuleLevelForMatching")]
    pub r#max_allowed_rule_level_for_matching: Option<i32>,
    /// Indicates the maximum allowed rule level for merging.
    #[builder(into)]
    #[serde(rename = "maxAllowedRuleLevelForMerging")]
    pub r#max_allowed_rule_level_for_merging: Option<i32>,
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainRuleBasedMatching {
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
                "attribute_types_selector".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#attribute_types_selector,
                )
                .await,
            );
            map.insert(
                "conflict_resolution".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#conflict_resolution,
                )
                .await,
            );
            map.insert(
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "exporting_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exporting_config,
                )
                .await,
            );
            map.insert(
                "matching_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#matching_rules,
                )
                .await,
            );
            map.insert(
                "max_allowed_rule_level_for_matching".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_allowed_rule_level_for_matching,
                )
                .await,
            );
            map.insert(
                "max_allowed_rule_level_for_merging".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_allowed_rule_level_for_merging,
                )
                .await,
            );
            map.insert(
                "status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#status,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainRuleBasedMatching {
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
                    r#attribute_types_selector: {
                        let field_value = match fields_map.get("attribute_types_selector") {
                            Some(value) => value,
                            None => bail!("Missing field 'attribute_types_selector' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#conflict_resolution: {
                        let field_value = match fields_map.get("conflict_resolution") {
                            Some(value) => value,
                            None => bail!("Missing field 'conflict_resolution' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exporting_config: {
                        let field_value = match fields_map.get("exporting_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'exporting_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#matching_rules: {
                        let field_value = match fields_map.get("matching_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'matching_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_allowed_rule_level_for_matching: {
                        let field_value = match fields_map.get("max_allowed_rule_level_for_matching") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_allowed_rule_level_for_matching' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_allowed_rule_level_for_merging: {
                        let field_value = match fields_map.get("max_allowed_rule_level_for_merging") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_allowed_rule_level_for_merging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
