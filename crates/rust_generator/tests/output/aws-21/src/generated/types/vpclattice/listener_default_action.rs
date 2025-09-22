#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListenerDefaultAction {
    #[builder(into)]
    #[serde(rename = "fixedResponse")]
    pub r#fixed_response: Option<Box<super::super::types::vpclattice::ListenerDefaultActionFixedResponse>>,
    /// Route requests to one or more target groups. See Forward blocks below.
    /// 
    /// > **NOTE:** You must specify exactly one of the following argument blocks: `fixed_response` or `forward`.
    #[builder(into)]
    #[serde(rename = "forwards")]
    pub r#forwards: Option<Vec<super::super::types::vpclattice::ListenerDefaultActionForward>>,
}
