#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TaskNotebookInfrastructureSpec {
    /// Compute resources needed for a Task when using Dataproc Serverless.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "batch")]
    pub r#batch: Option<Box<super::super::types::dataplex::TaskNotebookInfrastructureSpecBatch>>,
    /// Container Image Runtime Configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "containerImage")]
    pub r#container_image: Option<Box<super::super::types::dataplex::TaskNotebookInfrastructureSpecContainerImage>>,
    /// Vpc network.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vpcNetwork")]
    pub r#vpc_network: Option<Box<super::super::types::dataplex::TaskNotebookInfrastructureSpecVpcNetwork>>,
}
