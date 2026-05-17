#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RoleManagementPolicyActivationRules {
    /// An `approval_stage` block as defined below.
    #[builder(into)]
    #[serde(rename = "approvalStage")]
    pub r#approval_stage: Option<Box<super::super::types::pim::RoleManagementPolicyActivationRulesApprovalStage>>,
    /// The maximum length of time an activated role can be valid, in an ISO8601 Duration format (e.g. `PT8H`). Valid range is `PT30M` to `PT23H30M`, in 30 minute increments, or `PT1D`.
    #[builder(into)]
    #[serde(rename = "maximumDuration")]
    pub r#maximum_duration: Option<String>,
    /// Is approval required for activation. If `true` an `approval_stage` block must be provided.
    #[builder(into)]
    #[serde(rename = "requireApproval")]
    pub r#require_approval: Option<bool>,
    /// Is a justification required during activation of the role.
    #[builder(into)]
    #[serde(rename = "requireJustification")]
    pub r#require_justification: Option<bool>,
    /// Is multi-factor authentication required to activate the role. Conflicts with `required_conditional_access_authentication_context`.
    #[builder(into)]
    #[serde(rename = "requireMultifactorAuthentication")]
    pub r#require_multifactor_authentication: Option<bool>,
    /// Is ticket information requrired during activation of the role.
    #[builder(into)]
    #[serde(rename = "requireTicketInfo")]
    pub r#require_ticket_info: Option<bool>,
    /// The Entra ID Conditional Access context that must be present for activation. Conflicts with `require_multifactor_authentication`.
    #[builder(into)]
    #[serde(rename = "requiredConditionalAccessAuthenticationContext")]
    pub r#required_conditional_access_authentication_context: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RoleManagementPolicyActivationRules {
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
                "approval_stage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#approval_stage,
                )
                .await,
            );
            map.insert(
                "maximum_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maximum_duration,
                )
                .await,
            );
            map.insert(
                "require_approval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_approval,
                )
                .await,
            );
            map.insert(
                "require_justification".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_justification,
                )
                .await,
            );
            map.insert(
                "require_multifactor_authentication".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_multifactor_authentication,
                )
                .await,
            );
            map.insert(
                "require_ticket_info".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#require_ticket_info,
                )
                .await,
            );
            map.insert(
                "required_conditional_access_authentication_context".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#required_conditional_access_authentication_context,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RoleManagementPolicyActivationRules {
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
                    r#approval_stage: {
                        let field_value = match fields_map.get("approval_stage") {
                            Some(value) => value,
                            None => bail!("Missing field 'approval_stage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_duration: {
                        let field_value = match fields_map.get("maximum_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximum_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_approval: {
                        let field_value = match fields_map.get("require_approval") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_approval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_justification: {
                        let field_value = match fields_map.get("require_justification") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_justification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_multifactor_authentication: {
                        let field_value = match fields_map.get("require_multifactor_authentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_multifactor_authentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_ticket_info: {
                        let field_value = match fields_map.get("require_ticket_info") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_ticket_info' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#required_conditional_access_authentication_context: {
                        let field_value = match fields_map.get("required_conditional_access_authentication_context") {
                            Some(value) => value,
                            None => bail!("Missing field 'required_conditional_access_authentication_context' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
