#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BareMetalAdminClusterNodeAccessConfig {
    /// LoginUser is the user name used to access node machines.
    /// It defaults to "root" if not set.
    #[builder(into)]
    #[serde(rename = "loginUser")]
    pub r#login_user: Option<String>,
}
