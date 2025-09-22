#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountNetworkProfile {
    /// An `account_access` block as defined below.
    #[builder(into)]
    #[serde(rename = "accountAccess")]
    pub r#account_access: Box<Option<super::super::types::batch::AccountNetworkProfileAccountAccess>>,
    /// A `node_management_access` block as defined below.
    /// 
    /// > **NOTE:** At least one of `account_access` or `node_management_access` must be specified.
    #[builder(into)]
    #[serde(rename = "nodeManagementAccess")]
    pub r#node_management_access: Box<Option<super::super::types::batch::AccountNetworkProfileNodeManagementAccess>>,
}
