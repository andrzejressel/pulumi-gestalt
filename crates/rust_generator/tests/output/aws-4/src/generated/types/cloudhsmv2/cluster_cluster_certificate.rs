#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClusterCertificate {
    /// The HSM hardware certificate issued (signed) by AWS CloudHSM.
    #[builder(into)]
    #[serde(rename = "awsHardwareCertificate")]
    pub r#aws_hardware_certificate: Option<String>,
    /// The cluster certificate issued (signed) by the issuing certificate authority (CA) of the cluster's owner.
    #[builder(into)]
    #[serde(rename = "clusterCertificate")]
    pub r#cluster_certificate: Option<String>,
    /// The certificate signing request (CSR). Available only in `UNINITIALIZED` state after an HSM instance is added to the cluster.
    #[builder(into)]
    #[serde(rename = "clusterCsr")]
    pub r#cluster_csr: Option<String>,
    /// The HSM certificate issued (signed) by the HSM hardware.
    #[builder(into)]
    #[serde(rename = "hsmCertificate")]
    pub r#hsm_certificate: Option<String>,
    /// The HSM hardware certificate issued (signed) by the hardware manufacturer.
    #[builder(into)]
    #[serde(rename = "manufacturerHardwareCertificate")]
    pub r#manufacturer_hardware_certificate: Option<String>,
}
