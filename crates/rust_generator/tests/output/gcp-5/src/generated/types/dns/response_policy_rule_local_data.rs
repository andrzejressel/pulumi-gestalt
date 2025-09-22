#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResponsePolicyRuleLocalData {
    /// All resource record sets for this selector, one per resource record type. The name must match the dns_name.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "localDatas")]
    pub r#local_datas: Vec<super::super::types::dns::ResponsePolicyRuleLocalDataLocalData>,
}
