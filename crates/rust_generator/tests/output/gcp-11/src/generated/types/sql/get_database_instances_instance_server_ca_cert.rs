#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDatabaseInstancesInstanceServerCaCert {
    /// The CA Certificate used to connect to the SQL Instance via SSL.
    #[builder(into)]
    #[serde(rename = "cert")]
    pub r#cert: String,
    /// The CN valid for the CA Cert.
    #[builder(into)]
    #[serde(rename = "commonName")]
    pub r#common_name: String,
    /// Creation time of the CA Cert.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: String,
    /// Expiration time of the CA Cert.
    #[builder(into)]
    #[serde(rename = "expirationTime")]
    pub r#expiration_time: String,
    /// SHA Fingerprint of the CA Cert.
    #[builder(into)]
    #[serde(rename = "sha1Fingerprint")]
    pub r#sha_1_fingerprint: String,
}
