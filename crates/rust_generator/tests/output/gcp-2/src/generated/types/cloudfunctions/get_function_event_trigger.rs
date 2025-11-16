#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFunctionEventTrigger {
    /// The type of event to observe. For example: `"google.storage.object.finalize"`.
    /// See the documentation on [calling Cloud Functions](https://cloud.google.com/functions/docs/calling/)
    /// for a full reference of accepted triggers.
    #[builder(into)]
    #[serde(rename = "eventType")]
    pub r#event_type: String,
    /// Policy for failed executions. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "failurePolicies")]
    pub r#failure_policies: Vec<super::super::types::cloudfunctions::GetFunctionEventTriggerFailurePolicy>,
    /// The name of the resource whose events are being observed, for example, `"myBucket"`
    #[builder(into)]
    #[serde(rename = "resource")]
    pub r#resource: String,
}
