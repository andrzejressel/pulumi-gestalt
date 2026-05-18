#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEntitlementApprovalWorkflowManualApprovalStep {
    /// How many users from the above list need to approve.
    /// If there are not enough distinct users in the list above then the workflow
    /// will indefinitely block. Should always be greater than 0. Currently 1 is the only
    /// supported value.
    #[builder(into)]
    #[serde(rename = "approvalsNeeded")]
    pub r#approvals_needed: i32,
    /// Optional. Additional email addresses to be notified when a grant is pending approval.
    #[builder(into)]
    #[serde(rename = "approverEmailRecipients")]
    pub r#approver_email_recipients: Vec<String>,
    /// The potential set of approvers in this step. This list should contain at only one entry.
    #[builder(into)]
    #[serde(rename = "approvers")]
    pub r#approvers: Vec<super::super::types::privilegedaccessmanager::GetEntitlementApprovalWorkflowManualApprovalStepApprover>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetEntitlementApprovalWorkflowManualApprovalStep {
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
                    "approvals_needed",
                    &self.r#approvals_needed,
                ),
                to_pulumi_object_field(
                    "approver_email_recipients",
                    &self.r#approver_email_recipients,
                ),
                to_pulumi_object_field(
                    "approvers",
                    &self.r#approvers,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetEntitlementApprovalWorkflowManualApprovalStep {
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
                    r#approvals_needed: {
                        let field_value = match fields_map.get("approvals_needed") {
                            Some(value) => value,
                            None => bail!("Missing field 'approvals_needed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#approver_email_recipients: {
                        let field_value = match fields_map.get("approver_email_recipients") {
                            Some(value) => value,
                            None => bail!("Missing field 'approver_email_recipients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#approvers: {
                        let field_value = match fields_map.get("approvers") {
                            Some(value) => value,
                            None => bail!("Missing field 'approvers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
