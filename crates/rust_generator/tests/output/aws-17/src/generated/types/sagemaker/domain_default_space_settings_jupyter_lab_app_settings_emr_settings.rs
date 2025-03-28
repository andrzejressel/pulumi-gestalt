#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainDefaultSpaceSettingsJupyterLabAppSettingsEmrSettings {
    /// An array of Amazon Resource Names (ARNs) of the IAM roles that the execution role of SageMaker can assume for performing operations or tasks related to Amazon EMR clusters or Amazon EMR Serverless applications. These roles define the permissions and access policies required when performing Amazon EMR-related operations, such as listing, connecting to, or terminating Amazon EMR clusters or Amazon EMR Serverless applications. They are typically used in cross-account access scenarios, where the Amazon EMR resources (clusters or serverless applications) are located in a different AWS account than the SageMaker domain.
    #[builder(into, default)]
    #[serde(rename = "assumableRoleArns")]
    pub r#assumable_role_arns: Box<Option<Vec<String>>>,
    /// An array of Amazon Resource Names (ARNs) of the IAM roles used by the Amazon EMR cluster instances or job execution environments to access other AWS services and resources needed during the runtime of your Amazon EMR or Amazon EMR Serverless workloads, such as Amazon S3 for data access, Amazon CloudWatch for logging, or other AWS services based on the particular workload requirements.
    #[builder(into, default)]
    #[serde(rename = "executionRoleArns")]
    pub r#execution_role_arns: Box<Option<Vec<String>>>,
}
