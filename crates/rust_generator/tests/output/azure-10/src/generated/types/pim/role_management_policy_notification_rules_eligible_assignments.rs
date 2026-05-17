#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RoleManagementPolicyNotificationRulesEligibleAssignments {
    /// Admin notification settings
    #[builder(into)]
    #[serde(rename = "adminNotifications")]
    pub r#admin_notifications: Option<Box<super::super::types::pim::RoleManagementPolicyNotificationRulesEligibleAssignmentsAdminNotifications>>,
    /// Approver notification settings
    #[builder(into)]
    #[serde(rename = "approverNotifications")]
    pub r#approver_notifications: Option<Box<super::super::types::pim::RoleManagementPolicyNotificationRulesEligibleAssignmentsApproverNotifications>>,
    /// Assignee notification settings
    #[builder(into)]
    #[serde(rename = "assigneeNotifications")]
    pub r#assignee_notifications: Option<Box<super::super::types::pim::RoleManagementPolicyNotificationRulesEligibleAssignmentsAssigneeNotifications>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RoleManagementPolicyNotificationRulesEligibleAssignments {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "admin_notifications".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#admin_notifications,
                )
                .await,
            );
            map.insert(
                "approver_notifications".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#approver_notifications,
                )
                .await,
            );
            map.insert(
                "assignee_notifications".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#assignee_notifications,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RoleManagementPolicyNotificationRulesEligibleAssignments {
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
                    r#admin_notifications: {
                        let field_value = match fields_map.get("admin_notifications") {
                            Some(value) => value,
                            None => bail!("Missing field 'admin_notifications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#approver_notifications: {
                        let field_value = match fields_map.get("approver_notifications") {
                            Some(value) => value,
                            None => bail!("Missing field 'approver_notifications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#assignee_notifications: {
                        let field_value = match fields_map.get("assignee_notifications") {
                            Some(value) => value,
                            None => bail!("Missing field 'assignee_notifications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
