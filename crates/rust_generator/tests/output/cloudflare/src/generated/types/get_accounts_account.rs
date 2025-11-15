#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAccountsAccount {
    /// Whether 2FA is enforced on the account.
    #[builder(into)]
    #[serde(rename = "enforceTwofactor")]
    pub r#enforce_twofactor: Option<bool>,
    /// Account ID.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Account name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Account subscription type.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
