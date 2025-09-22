#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GuestPoliciesPackageRepositoryZypper {
    /// The location of the repository directory.
    #[builder(into)]
    #[serde(rename = "baseUrl")]
    pub r#base_url: String,
    /// The display name of the repository.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// URIs of GPG keys.
    #[builder(into)]
    #[serde(rename = "gpgKeys")]
    pub r#gpg_keys: Option<Vec<String>>,
    /// A one word, unique name for this repository. This is the repo id in the zypper config file and also the displayName
    /// if displayName is omitted. This id is also used as the unique identifier when checking for guest policy conflicts.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
}
