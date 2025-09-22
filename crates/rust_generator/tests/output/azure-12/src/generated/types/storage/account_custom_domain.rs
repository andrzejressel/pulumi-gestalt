#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountCustomDomain {
    /// The Custom Domain Name to use for the Storage Account, which will be validated by Azure.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Should the Custom Domain Name be validated by using indirect CNAME validation?
    #[builder(into)]
    #[serde(rename = "useSubdomain")]
    pub r#use_subdomain: Option<bool>,
}
