#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RoleManagementPolicyNotificationRules {
    /// A `notification_target` block as defined below to configure notfications on active role assignments.
    #[builder(into)]
    #[serde(rename = "activeAssignments")]
    pub r#active_assignments: Option<Box<super::super::types::pim::RoleManagementPolicyNotificationRulesActiveAssignments>>,
    /// A `notification_target` block as defined below for configuring notifications on activation of eligible role.
    #[builder(into)]
    #[serde(rename = "eligibleActivations")]
    pub r#eligible_activations: Option<Box<super::super::types::pim::RoleManagementPolicyNotificationRulesEligibleActivations>>,
    /// A `notification_target` block as defined below to configure notification on eligible role assignments.
    /// 
    /// At least one `notification_target` block must be provided.
    #[builder(into)]
    #[serde(rename = "eligibleAssignments")]
    pub r#eligible_assignments: Option<Box<super::super::types::pim::RoleManagementPolicyNotificationRulesEligibleAssignments>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RoleManagementPolicyNotificationRules {
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
                "active_assignments".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#active_assignments,
                )
                .await,
            );
            map.insert(
                "eligible_activations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#eligible_activations,
                )
                .await,
            );
            map.insert(
                "eligible_assignments".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#eligible_assignments,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RoleManagementPolicyNotificationRules {
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
                        let field_value = match fields_map.get("active_assignments") {
                            Some(value) => value,
                            None => bail!("Missing field 'active_assignments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#eligible_activations: {
                        let field_value = match fields_map.get("eligible_activations") {
                            Some(value) => value,
                            None => bail!("Missing field 'eligible_activations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#eligible_assignments: {
                        let field_value = match fields_map.get("eligible_assignments") {
                            Some(value) => value,
                            None => bail!("Missing field 'eligible_assignments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
