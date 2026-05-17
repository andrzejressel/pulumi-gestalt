#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LifecyclePolicyPolicyDetailsSchedule {
    #[builder(into)]
    #[serde(rename = "copyTags")]
    pub r#copy_tags: Option<bool>,
    /// See the `create_rule` block. Max of 1 per schedule.
    #[builder(into)]
    #[serde(rename = "createRule")]
    pub r#create_rule: Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleCreateRule>,
    /// See the `cross_region_copy_rule` block. Max of 3 per schedule.
    #[builder(into)]
    #[serde(rename = "crossRegionCopyRules")]
    pub r#cross_region_copy_rules: Option<Vec<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleCrossRegionCopyRule>>,
    #[builder(into)]
    #[serde(rename = "deprecateRule")]
    pub r#deprecate_rule: Option<Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleDeprecateRule>>,
    /// See the `fast_restore_rule` block. Max of 1 per schedule.
    #[builder(into)]
    #[serde(rename = "fastRestoreRule")]
    pub r#fast_restore_rule: Option<Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleFastRestoreRule>>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "retainRule")]
    pub r#retain_rule: Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleRetainRule>,
    /// See the `share_rule` block. Max of 1 per schedule.
    #[builder(into)]
    #[serde(rename = "shareRule")]
    pub r#share_rule: Option<Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleShareRule>>,
    /// A map of tag keys and their values. DLM lifecycle policies will already tag the snapshot with the tags on the volume. This configuration adds extra tags on top of these.
    #[builder(into)]
    #[serde(rename = "tagsToAdd")]
    pub r#tags_to_add: Option<std::collections::HashMap<String, String>>,
    /// A map of tag keys and variable values, where the values are determined when the policy is executed. Only `$(instance-id)` or `$(timestamp)` are valid values. Can only be used when `resource_types` is `INSTANCE`.
    #[builder(into)]
    #[serde(rename = "variableTags")]
    pub r#variable_tags: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LifecyclePolicyPolicyDetailsSchedule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "copy_tags",
                    &self.r#copy_tags,
                ),
                to_pulumi_object_field(
                    "create_rule",
                    &self.r#create_rule,
                ),
                to_pulumi_object_field(
                    "cross_region_copy_rules",
                    &self.r#cross_region_copy_rules,
                ),
                to_pulumi_object_field(
                    "deprecate_rule",
                    &self.r#deprecate_rule,
                ),
                to_pulumi_object_field(
                    "fast_restore_rule",
                    &self.r#fast_restore_rule,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "retain_rule",
                    &self.r#retain_rule,
                ),
                to_pulumi_object_field(
                    "share_rule",
                    &self.r#share_rule,
                ),
                to_pulumi_object_field(
                    "tags_to_add",
                    &self.r#tags_to_add,
                ),
                to_pulumi_object_field(
                    "variable_tags",
                    &self.r#variable_tags,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LifecyclePolicyPolicyDetailsSchedule {
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
                    r#copy_tags: {
                        let field_value = match fields_map.get("copy_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'copy_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_rule: {
                        let field_value = match fields_map.get("create_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'create_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cross_region_copy_rules: {
                        let field_value = match fields_map.get("cross_region_copy_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'cross_region_copy_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#fast_restore_rule: {
                        let field_value = match fields_map.get("fast_restore_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'fast_restore_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#retain_rule: {
                        let field_value = match fields_map.get("retain_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'retain_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#share_rule: {
                        let field_value = match fields_map.get("share_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'share_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags_to_add: {
                        let field_value = match fields_map.get("tags_to_add") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags_to_add' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#variable_tags: {
                        let field_value = match fields_map.get("variable_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'variable_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
