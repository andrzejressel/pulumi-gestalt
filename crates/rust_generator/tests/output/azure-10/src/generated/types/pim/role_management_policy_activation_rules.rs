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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "approvalStage",
                    &self.r#approval_stage,
                ),
                to_pulumi_object_field(
                    "maximumDuration",
                    &self.r#maximum_duration,
                ),
                to_pulumi_object_field(
                    "requireApproval",
                    &self.r#require_approval,
                ),
                to_pulumi_object_field(
                    "requireJustification",
                    &self.r#require_justification,
                ),
                to_pulumi_object_field(
                    "requireMultifactorAuthentication",
                    &self.r#require_multifactor_authentication,
                ),
                to_pulumi_object_field(
                    "requireTicketInfo",
                    &self.r#require_ticket_info,
                ),
                to_pulumi_object_field(
                    "requiredConditionalAccessAuthenticationContext",
                    &self.r#required_conditional_access_authentication_context,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("approvalStage") {
                            Some(value) => value,
                            None => bail!("Missing field 'approvalStage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maximum_duration: {
                        let field_value = match fields_map.get("maximumDuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'maximumDuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_approval: {
                        let field_value = match fields_map.get("requireApproval") {
                            Some(value) => value,
                            None => bail!("Missing field 'requireApproval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_justification: {
                        let field_value = match fields_map.get("requireJustification") {
                            Some(value) => value,
                            None => bail!("Missing field 'requireJustification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_multifactor_authentication: {
                        let field_value = match fields_map.get("requireMultifactorAuthentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'requireMultifactorAuthentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_ticket_info: {
                        let field_value = match fields_map.get("requireTicketInfo") {
                            Some(value) => value,
                            None => bail!("Missing field 'requireTicketInfo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#required_conditional_access_authentication_context: {
                        let field_value = match fields_map.get("requiredConditionalAccessAuthenticationContext") {
                            Some(value) => value,
                            None => bail!("Missing field 'requiredConditionalAccessAuthenticationContext' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
