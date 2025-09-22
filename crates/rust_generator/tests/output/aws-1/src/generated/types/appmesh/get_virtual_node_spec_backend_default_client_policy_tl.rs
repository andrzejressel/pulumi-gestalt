#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVirtualNodeSpecBackendDefaultClientPolicyTl {
    #[builder(into)]
    #[serde(rename = "certificates")]
    pub r#certificates: Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendDefaultClientPolicyTlCertificate>,
    #[builder(into)]
    #[serde(rename = "enforce")]
    pub r#enforce: bool,
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Vec<i32>,
    #[builder(into)]
    #[serde(rename = "validations")]
    pub r#validations: Vec<super::super::types::appmesh::GetVirtualNodeSpecBackendDefaultClientPolicyTlValidation>,
}
