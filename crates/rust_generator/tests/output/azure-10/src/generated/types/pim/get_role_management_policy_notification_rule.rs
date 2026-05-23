#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRoleManagementPolicyNotificationRule {
    /// A `notification_target` block as defined below with the details of notfications on active role assignments.
    #[builder(into)]
    pub r#active_assignments: Vec<super::super::types::pim::GetRoleManagementPolicyNotificationRuleActiveAssignment>,
    /// A `notification_target` block as defined below with the details of notifications on activation of eligible role.
    #[builder(into)]
    pub r#eligible_activations: Vec<super::super::types::pim::GetRoleManagementPolicyNotificationRuleEligibleActivation>,
    /// A `notification_target` block as defined below with the details of notifications on eligible role assignments.
    #[builder(into)]
    pub r#eligible_assignments: Vec<super::super::types::pim::GetRoleManagementPolicyNotificationRuleEligibleAssignment>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRoleManagementPolicyNotificationRule {
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
                    "activeAssignments",
                    &self.r#active_assignments,
                ),
                to_pulumi_object_field(
                    "eligibleActivations",
                    &self.r#eligible_activations,
                ),
                to_pulumi_object_field(
                    "eligibleAssignments",
                    &self.r#eligible_assignments,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRoleManagementPolicyNotificationRule {
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
                    r#active_assignments: {
                        let field_value = match fields_map.get("activeAssignments") {
                            Some(value) => value,
                            None => bail!("Missing field 'activeAssignments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#eligible_activations: {
                        let field_value = match fields_map.get("eligibleActivations") {
                            Some(value) => value,
                            None => bail!("Missing field 'eligibleActivations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#eligible_assignments: {
                        let field_value = match fields_map.get("eligibleAssignments") {
                            Some(value) => value,
                            None => bail!("Missing field 'eligibleAssignments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
