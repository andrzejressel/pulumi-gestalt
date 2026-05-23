#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainRuleBasedMatching {
    /// A block that configures information about the `AttributeTypesSelector` where the rule-based identity resolution uses to match profiles. Documented below.
    #[builder(into)]
    pub r#attribute_types_selector: Option<Box<super::super::types::customerprofiles::DomainRuleBasedMatchingAttributeTypesSelector>>,
    /// A block that specifies how the auto-merging process should resolve conflicts between different profiles. Documented below.
    #[builder(into)]
    pub r#conflict_resolution: Option<Box<super::super::types::customerprofiles::DomainRuleBasedMatchingConflictResolution>>,
    /// The flag that enables the rule-based matching process of duplicate profiles.
    #[builder(into)]
    pub r#enabled: bool,
    /// A block that specifies the configuration for exporting Identity Resolution results. Documented below.
    #[builder(into)]
    pub r#exporting_config: Option<Box<super::super::types::customerprofiles::DomainRuleBasedMatchingExportingConfig>>,
    /// A block that configures how the rule-based matching process should match profiles. You can have up to 15 `rule` in the `natching_rules`. Documented below.
    #[builder(into)]
    pub r#matching_rules: Option<Vec<super::super::types::customerprofiles::DomainRuleBasedMatchingMatchingRule>>,
    /// Indicates the maximum allowed rule level for matching.
    #[builder(into)]
    pub r#max_allowed_rule_level_for_matching: Option<i32>,
    /// Indicates the maximum allowed rule level for merging.
    #[builder(into)]
    pub r#max_allowed_rule_level_for_merging: Option<i32>,
    #[builder(into)]
    pub r#status: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainRuleBasedMatching {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "attributeTypesSelector",
                    &self.r#attribute_types_selector,
                ),
                to_pulumi_object_field(
                    "conflictResolution",
                    &self.r#conflict_resolution,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "exportingConfig",
                    &self.r#exporting_config,
                ),
                to_pulumi_object_field(
                    "matchingRules",
                    &self.r#matching_rules,
                ),
                to_pulumi_object_field(
                    "maxAllowedRuleLevelForMatching",
                    &self.r#max_allowed_rule_level_for_matching,
                ),
                to_pulumi_object_field(
                    "maxAllowedRuleLevelForMerging",
                    &self.r#max_allowed_rule_level_for_merging,
                ),
                to_pulumi_object_field(
                    "status",
                    &self.r#status,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("attributeTypesSelector") {
                            Some(value) => value,
                            None => bail!("Missing field 'attributeTypesSelector' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#conflict_resolution: {
                        let field_value = match fields_map.get("conflictResolution") {
                            Some(value) => value,
                            None => bail!("Missing field 'conflictResolution' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("exportingConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'exportingConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#matching_rules: {
                        let field_value = match fields_map.get("matchingRules") {
                            Some(value) => value,
                            None => bail!("Missing field 'matchingRules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_allowed_rule_level_for_matching: {
                        let field_value = match fields_map.get("maxAllowedRuleLevelForMatching") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxAllowedRuleLevelForMatching' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_allowed_rule_level_for_merging: {
                        let field_value = match fields_map.get("maxAllowedRuleLevelForMerging") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxAllowedRuleLevelForMerging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
