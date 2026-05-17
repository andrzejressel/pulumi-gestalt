#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRoleManagementPolicyActivationRule {
    /// An `approval_stage` block as defined below.
    #[builder(into)]
    #[serde(rename = "approvalStages")]
    pub r#approval_stages: Vec<super::super::types::pim::GetRoleManagementPolicyActivationRuleApprovalStage>,
    /// (String) The maximum length of time an activated role can be valid, in an ISO8601 Duration format.
    #[builder(into)]
    #[serde(rename = "maximumDuration")]
    pub r#maximum_duration: String,
    /// (Boolean) Is approval required for activation.
    #[builder(into)]
    #[serde(rename = "requireApproval")]
    pub r#require_approval: bool,
    /// (Boolean) Is a justification required to create new assignments.
    #[builder(into)]
    #[serde(rename = "requireJustification")]
    pub r#require_justification: bool,
    /// (Boolean) Is multi-factor authentication required to create new assignments.
    #[builder(into)]
    #[serde(rename = "requireMultifactorAuthentication")]
    pub r#require_multifactor_authentication: bool,
    /// (Boolean) Is ticket information required to create new assignments.
    #[builder(into)]
    #[serde(rename = "requireTicketInfo")]
    pub r#require_ticket_info: bool,
    /// (String) The Entra ID Conditional Access context that must be present for activation.
    #[builder(into)]
    #[serde(rename = "requiredConditionalAccessAuthenticationContext")]
    pub r#required_conditional_access_authentication_context: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRoleManagementPolicyActivationRule {
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
                    "approval_stages",
                    &self.r#approval_stages,
                ),
                to_pulumi_object_field(
                    "maximum_duration",
                    &self.r#maximum_duration,
                ),
                to_pulumi_object_field(
                    "require_approval",
                    &self.r#require_approval,
                ),
                to_pulumi_object_field(
                    "require_justification",
                    &self.r#require_justification,
                ),
                to_pulumi_object_field(
                    "require_multifactor_authentication",
                    &self.r#require_multifactor_authentication,
                ),
                to_pulumi_object_field(
                    "require_ticket_info",
                    &self.r#require_ticket_info,
                ),
                to_pulumi_object_field(
                    "required_conditional_access_authentication_context",
                    &self.r#required_conditional_access_authentication_context,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRoleManagementPolicyActivationRule {
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
                    r#approval_stages: {
                        let field_value = match fields_map.get("approval_stages") {
                            Some(value) => value,
                            None => bail!("Missing field 'approval_stages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
