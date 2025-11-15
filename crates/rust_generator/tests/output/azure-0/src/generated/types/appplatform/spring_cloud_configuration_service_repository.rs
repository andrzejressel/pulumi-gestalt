#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudConfigurationServiceRepository {
    /// Specifies the ID of the Certificate Authority used when retrieving the Git Repository via HTTPS.
    #[builder(into)]
    #[serde(rename = "caCertificateId")]
    pub r#ca_certificate_id: Option<String>,
    /// Specifies the SSH public key of git repository.
    #[builder(into)]
    #[serde(rename = "hostKey")]
    pub r#host_key: Option<String>,
    /// Specifies the SSH key algorithm of git repository.
    #[builder(into)]
    #[serde(rename = "hostKeyAlgorithm")]
    pub r#host_key_algorithm: Option<String>,
    /// Specifies the label of the repository.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: String,
    /// Specifies the name which should be used for this repository.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies the password of git repository basic auth.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// Specifies the collection of patterns of the repository.
    #[builder(into)]
    #[serde(rename = "patterns")]
    pub r#patterns: Vec<String>,
    /// Specifies the SSH private key of git repository.
    #[builder(into)]
    #[serde(rename = "privateKey")]
    pub r#private_key: Option<String>,
    /// Specifies a list of searching path of the repository
    #[builder(into)]
    #[serde(rename = "searchPaths")]
    pub r#search_paths: Option<Vec<String>>,
    /// Specifies whether enable the strict host key checking.
    #[builder(into)]
    #[serde(rename = "strictHostKeyChecking")]
    pub r#strict_host_key_checking: Option<bool>,
    /// Specifies the URI of the repository.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
    /// Specifies the username of git repository basic auth.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
