#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ComputeInstanceSsh {
    /// Describes the port for connecting through SSH.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// Specifies the SSH rsa public key file as a string. Use "ssh-keygen -t rsa -b 2048" to generate your SSH key pairs.
    #[builder(into)]
    #[serde(rename = "publicKey")]
    pub r#public_key: String,
    /// The admin username of this Machine Learning Compute Instance.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
