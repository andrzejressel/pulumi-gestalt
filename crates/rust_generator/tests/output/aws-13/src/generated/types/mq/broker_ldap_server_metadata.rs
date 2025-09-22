#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BrokerLdapServerMetadata {
    /// List of a fully qualified domain name of the LDAP server and an optional failover server.
    #[builder(into)]
    #[serde(rename = "hosts")]
    pub r#hosts: Option<Vec<String>>,
    /// Fully qualified name of the directory to search for a user’s groups.
    #[builder(into)]
    #[serde(rename = "roleBase")]
    pub r#role_base: Option<String>,
    /// Specifies the LDAP attribute that identifies the group name attribute in the object returned from the group membership query.
    #[builder(into)]
    #[serde(rename = "roleName")]
    pub r#role_name: Option<String>,
    /// Search criteria for groups.
    #[builder(into)]
    #[serde(rename = "roleSearchMatching")]
    pub r#role_search_matching: Option<String>,
    /// Whether the directory search scope is the entire sub-tree.
    #[builder(into)]
    #[serde(rename = "roleSearchSubtree")]
    pub r#role_search_subtree: Option<bool>,
    /// Service account password.
    #[builder(into)]
    #[serde(rename = "serviceAccountPassword")]
    pub r#service_account_password: Option<String>,
    /// Service account username.
    #[builder(into)]
    #[serde(rename = "serviceAccountUsername")]
    pub r#service_account_username: Option<String>,
    /// Fully qualified name of the directory where you want to search for users.
    #[builder(into)]
    #[serde(rename = "userBase")]
    pub r#user_base: Option<String>,
    /// Specifies the name of the LDAP attribute for the user group membership.
    #[builder(into)]
    #[serde(rename = "userRoleName")]
    pub r#user_role_name: Option<String>,
    /// Search criteria for users.
    #[builder(into)]
    #[serde(rename = "userSearchMatching")]
    pub r#user_search_matching: Option<String>,
    /// Whether the directory search scope is the entire sub-tree.
    #[builder(into)]
    #[serde(rename = "userSearchSubtree")]
    pub r#user_search_subtree: Option<bool>,
}
