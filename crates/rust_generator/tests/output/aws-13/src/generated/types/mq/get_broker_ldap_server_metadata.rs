#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBrokerLdapServerMetadata {
    #[builder(into)]
    #[serde(rename = "hosts")]
    pub r#hosts: Vec<String>,
    #[builder(into)]
    #[serde(rename = "roleBase")]
    pub r#role_base: String,
    #[builder(into)]
    #[serde(rename = "roleName")]
    pub r#role_name: String,
    #[builder(into)]
    #[serde(rename = "roleSearchMatching")]
    pub r#role_search_matching: String,
    #[builder(into)]
    #[serde(rename = "roleSearchSubtree")]
    pub r#role_search_subtree: bool,
    #[builder(into)]
    #[serde(rename = "serviceAccountPassword")]
    pub r#service_account_password: String,
    #[builder(into)]
    #[serde(rename = "serviceAccountUsername")]
    pub r#service_account_username: String,
    #[builder(into)]
    #[serde(rename = "userBase")]
    pub r#user_base: String,
    #[builder(into)]
    #[serde(rename = "userRoleName")]
    pub r#user_role_name: String,
    #[builder(into)]
    #[serde(rename = "userSearchMatching")]
    pub r#user_search_matching: String,
    #[builder(into)]
    #[serde(rename = "userSearchSubtree")]
    pub r#user_search_subtree: bool,
}
