#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificate {
    #[builder(into)]
    #[serde(rename = "files")]
    pub r#files: Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificateFile>,
    #[builder(into)]
    #[serde(rename = "sds")]
    pub r#sds: Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendVirtualServiceClientPolicyTlCertificateSd>,
}
