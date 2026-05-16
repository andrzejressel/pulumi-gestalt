#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterCertificateCommonNamesCommonName {
    /// The common or subject name of the certificate.
    #[builder(into)]
    #[serde(rename = "certificateCommonName")]
    pub r#certificate_common_name: String,
    /// The Issuer Thumbprint of the Certificate.
    /// 
    /// > **NOTE:** Certificate Issuer Thumbprint may become required in the future, `https://docs.microsoft.com/azure/service-fabric/service-fabric-create-cluster-using-cert-cn#download-and-update-a-sample-template`.
    #[builder(into)]
    #[serde(rename = "certificateIssuerThumbprint")]
    pub r#certificate_issuer_thumbprint: Option<String>,
}
