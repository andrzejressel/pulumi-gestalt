#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LifecyclePolicyPolicyDetailsSchedule {
    #[builder(into)]
    pub r#copy_tags: Option<bool>,
    /// See the `create_rule` block. Max of 1 per schedule.
    #[builder(into)]
    pub r#create_rule: Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleCreateRule>,
    /// See the `cross_region_copy_rule` block. Max of 3 per schedule.
    #[builder(into)]
    pub r#cross_region_copy_rules: Option<Vec<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleCrossRegionCopyRule>>,
    #[builder(into)]
    pub r#deprecate_rule: Option<Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleDeprecateRule>>,
    /// See the `fast_restore_rule` block. Max of 1 per schedule.
    #[builder(into)]
    pub r#fast_restore_rule: Option<Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleFastRestoreRule>>,
    #[builder(into)]
    pub r#name: String,
    #[builder(into)]
    pub r#retain_rule: Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleRetainRule>,
    /// See the `share_rule` block. Max of 1 per schedule.
    #[builder(into)]
    pub r#share_rule: Option<Box<super::super::types::dlm::LifecyclePolicyPolicyDetailsScheduleShareRule>>,
    /// A map of tag keys and their values. DLM lifecycle policies will already tag the snapshot with the tags on the volume. This configuration adds extra tags on top of these.
    #[builder(into)]
    pub r#tags_to_add: Option<std::collections::HashMap<String, String>>,
    /// A map of tag keys and variable values, where the values are determined when the policy is executed. Only `$(instance-id)` or `$(timestamp)` are valid values. Can only be used when `resource_types` is `INSTANCE`.
    #[builder(into)]
    pub r#variable_tags: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LifecyclePolicyPolicyDetailsSchedule {
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
                    "copyTags",
                    &self.r#copy_tags,
                ),
                to_pulumi_object_field(
                    "createRule",
                    &self.r#create_rule,
                ),
                to_pulumi_object_field(
                    "crossRegionCopyRules",
                    &self.r#cross_region_copy_rules,
                ),
                to_pulumi_object_field(
                    "deprecateRule",
                    &self.r#deprecate_rule,
                ),
                to_pulumi_object_field(
                    "fastRestoreRule",
                    &self.r#fast_restore_rule,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "retainRule",
                    &self.r#retain_rule,
                ),
                to_pulumi_object_field(
                    "shareRule",
                    &self.r#share_rule,
                ),
                to_pulumi_object_field(
                    "tagsToAdd",
                    &self.r#tags_to_add,
                ),
                to_pulumi_object_field(
                    "variableTags",
                    &self.r#variable_tags,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("copyTags") {
                            Some(value) => value,
                            None => bail!("Missing field 'copyTags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_rule: {
                        let field_value = match fields_map.get("createRule") {
                            Some(value) => value,
                            None => bail!("Missing field 'createRule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cross_region_copy_rules: {
                        let field_value = match fields_map.get("crossRegionCopyRules") {
                            Some(value) => value,
                            None => bail!("Missing field 'crossRegionCopyRules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deprecate_rule: {
                        let field_value = match fields_map.get("deprecateRule") {
                            Some(value) => value,
                            None => bail!("Missing field 'deprecateRule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fast_restore_rule: {
                        let field_value = match fields_map.get("fastRestoreRule") {
                            Some(value) => value,
                            None => bail!("Missing field 'fastRestoreRule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("retainRule") {
                            Some(value) => value,
                            None => bail!("Missing field 'retainRule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#share_rule: {
                        let field_value = match fields_map.get("shareRule") {
                            Some(value) => value,
                            None => bail!("Missing field 'shareRule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags_to_add: {
                        let field_value = match fields_map.get("tagsToAdd") {
                            Some(value) => value,
                            None => bail!("Missing field 'tagsToAdd' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#variable_tags: {
                        let field_value = match fields_map.get("variableTags") {
                            Some(value) => value,
                            None => bail!("Missing field 'variableTags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
