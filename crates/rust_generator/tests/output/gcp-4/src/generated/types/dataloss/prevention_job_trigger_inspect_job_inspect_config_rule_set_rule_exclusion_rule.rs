#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRule {
    /// Dictionary which defines the rule.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dictionary")]
    pub r#dictionary: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleDictionary>>,
    /// Drop if the hotword rule is contained in the proximate context.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "excludeByHotword")]
    pub r#exclude_by_hotword: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleExcludeByHotword>>,
    /// Set of infoTypes for which findings would affect this rule.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "excludeInfoTypes")]
    pub r#exclude_info_types: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleExcludeInfoTypes>>,
    /// How the rule is applied. See the documentation for more information: https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#MatchingType
    /// Possible values are: `MATCHING_TYPE_FULL_MATCH`, `MATCHING_TYPE_PARTIAL_MATCH`, `MATCHING_TYPE_INVERSE_MATCH`.
    #[builder(into)]
    #[serde(rename = "matchingType")]
    pub r#matching_type: String,
    /// Regular expression which defines the rule.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleRegex>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "dictionary",
                    &self.r#dictionary,
                ),
                to_pulumi_object_field(
                    "exclude_by_hotword",
                    &self.r#exclude_by_hotword,
                ),
                to_pulumi_object_field(
                    "exclude_info_types",
                    &self.r#exclude_info_types,
                ),
                to_pulumi_object_field(
                    "matching_type",
                    &self.r#matching_type,
                ),
                to_pulumi_object_field(
                    "regex",
                    &self.r#regex,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRule {
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
                    r#dictionary: {
                        let field_value = match fields_map.get("dictionary") {
                            Some(value) => value,
                            None => bail!("Missing field 'dictionary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclude_by_hotword: {
                        let field_value = match fields_map.get("exclude_by_hotword") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude_by_hotword' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclude_info_types: {
                        let field_value = match fields_map.get("exclude_info_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude_info_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#matching_type: {
                        let field_value = match fields_map.get("matching_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'matching_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regex: {
                        let field_value = match fields_map.get("regex") {
                            Some(value) => value,
                            None => bail!("Missing field 'regex' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
