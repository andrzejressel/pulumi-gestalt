#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetStandardsControlAssociationsStandardsControlAssociation {
    /// Enablement status of a control in a specific standard.
    #[builder(into)]
    #[serde(rename = "associationStatus")]
    pub r#association_status: String,
    /// List of underlying requirements in the compliance framework related to the standard.
    #[builder(into)]
    #[serde(rename = "relatedRequirements")]
    pub r#related_requirements: Vec<String>,
    /// ARN of the security control.
    #[builder(into)]
    #[serde(rename = "securityControlArn")]
    pub r#security_control_arn: String,
    /// The identifier of the control (identified with `SecurityControlId`, `SecurityControlArn`, or a mix of both parameters).
    #[builder(into)]
    #[serde(rename = "securityControlId")]
    pub r#security_control_id: String,
    /// ARN of the standard.
    #[builder(into)]
    #[serde(rename = "standardsArn")]
    pub r#standards_arn: String,
    /// Description of the standard.
    #[builder(into)]
    #[serde(rename = "standardsControlDescription")]
    pub r#standards_control_description: String,
    /// Title of the standard.
    #[builder(into)]
    #[serde(rename = "standardsControlTitle")]
    pub r#standards_control_title: String,
    /// Last time that a control's enablement status in a specified standard was updated.
    #[builder(into)]
    #[serde(rename = "updatedAt")]
    pub r#updated_at: String,
    /// Reason for updating a control's enablement status in a specified standard.
    #[builder(into)]
    #[serde(rename = "updatedReason")]
    pub r#updated_reason: String,
}
