#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponseMessageGroup {
    /// Configuration block for the primary message that Amazon Lex should send to the user.
    /// See the `aws.lex.V2modelsIntent` resource for details on the `message` argument reference - they are identical.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Box<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponseMessageGroupMessage>,
    /// Configuration blocks for message variations to send to the user.
    /// When variations are defined, Amazon Lex chooses the primary message or one of the variations to send to the user.
    /// See the `aws.lex.V2modelsIntent` resource for details on the `variation` argument reference - they are identical.
    #[builder(into, default)]
    #[serde(rename = "variations")]
    pub r#variations: Box<Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponseMessageGroupVariation>>>,
}
