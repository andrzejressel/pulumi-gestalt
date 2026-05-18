#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutomationRule {
    /// Optional. The `AdvanceRolloutRule` will automatically advance a successful Rollout.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "advanceRolloutRule")]
    pub r#advance_rollout_rule: Option<Box<super::super::types::clouddeploy::AutomationRuleAdvanceRolloutRule>>,
    /// Optional. `PromoteReleaseRule` will automatically promote a release from the current target to a specified target.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "promoteReleaseRule")]
    pub r#promote_release_rule: Option<Box<super::super::types::clouddeploy::AutomationRulePromoteReleaseRule>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AutomationRule {
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
                    "advance_rollout_rule",
                    &self.r#advance_rollout_rule,
                ),
                to_pulumi_object_field(
                    "promote_release_rule",
                    &self.r#promote_release_rule,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AutomationRule {
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
                    r#advance_rollout_rule: {
                        let field_value = match fields_map.get("advance_rollout_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'advance_rollout_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#promote_release_rule: {
                        let field_value = match fields_map.get("promote_release_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'promote_release_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
