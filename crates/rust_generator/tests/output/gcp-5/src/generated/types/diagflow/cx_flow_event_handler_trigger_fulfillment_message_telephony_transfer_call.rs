#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxFlowEventHandlerTriggerFulfillmentMessageTelephonyTransferCall {
    /// Transfer the call to a phone number in E.164 format.
    #[builder(into)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: Box<String>,
}
