#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutomationRuleCriteria {
    /// The AWS account ID in which a finding was generated. Documented below.
    #[builder(into)]
    #[serde(rename = "awsAccountIds")]
    pub r#aws_account_ids: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaAwsAccountId>>,
    /// The name of the AWS account in which a finding was generated. Documented below.
    #[builder(into)]
    #[serde(rename = "awsAccountNames")]
    pub r#aws_account_names: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaAwsAccountName>>,
    /// The name of the company for the product that generated the finding. For control-based findings, the company is AWS. Documented below.
    #[builder(into)]
    #[serde(rename = "companyNames")]
    pub r#company_names: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaCompanyName>>,
    /// The unique identifier of a standard in which a control is enabled. Documented below.
    #[builder(into)]
    #[serde(rename = "complianceAssociatedStandardsIds")]
    pub r#compliance_associated_standards_ids: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaComplianceAssociatedStandardsId>>,
    /// The security control ID for which a finding was generated. Security control IDs are the same across standards. Documented below.
    #[builder(into)]
    #[serde(rename = "complianceSecurityControlIds")]
    pub r#compliance_security_control_ids: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaComplianceSecurityControlId>>,
    /// The result of a security check. This field is only used for findings generated from controls. Documented below.
    #[builder(into)]
    #[serde(rename = "complianceStatuses")]
    pub r#compliance_statuses: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaComplianceStatus>>,
    /// The likelihood that a finding accurately identifies the behavior or issue that it was intended to identify. `Confidence` is scored on a 0–100 basis using a ratio scale. A value of `0` means 0 percent confidence, and a value of `100` means 100 percent confidence. Documented below.
    #[builder(into)]
    #[serde(rename = "confidences")]
    pub r#confidences: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaConfidence>>,
    /// A timestamp that indicates when this finding record was created. Documented below.
    #[builder(into)]
    #[serde(rename = "createdAts")]
    pub r#created_ats: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaCreatedAt>>,
    /// The level of importance that is assigned to the resources that are associated with a finding. Documented below.
    #[builder(into)]
    #[serde(rename = "criticalities")]
    pub r#criticalities: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaCriticality>>,
    /// A finding's description. Documented below.
    #[builder(into)]
    #[serde(rename = "descriptions")]
    pub r#descriptions: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaDescription>>,
    /// A timestamp that indicates when the potential security issue captured by a finding was first observed by the security findings product. Documented below.
    #[builder(into)]
    #[serde(rename = "firstObservedAts")]
    pub r#first_observed_ats: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaFirstObservedAt>>,
    /// The identifier for the solution-specific component that generated a finding. Documented below.
    #[builder(into)]
    #[serde(rename = "generatorIds")]
    pub r#generator_ids: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaGeneratorId>>,
    /// The product-specific identifier for a finding. Documented below.
    #[builder(into)]
    #[serde(rename = "ids")]
    pub r#ids: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaId>>,
    /// A timestamp that indicates when the potential security issue captured by a finding was most recently observed by the security findings product. Documented below.
    #[builder(into)]
    #[serde(rename = "lastObservedAts")]
    pub r#last_observed_ats: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaLastObservedAt>>,
    /// The text of a user-defined note that's added to a finding. Documented below.
    #[builder(into)]
    #[serde(rename = "noteTexts")]
    pub r#note_texts: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaNoteText>>,
    /// The timestamp of when the note was updated. Documented below.
    #[builder(into)]
    #[serde(rename = "noteUpdatedAts")]
    pub r#note_updated_ats: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaNoteUpdatedAt>>,
    /// The principal that created a note. Documented below.
    #[builder(into)]
    #[serde(rename = "noteUpdatedBies")]
    pub r#note_updated_bies: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaNoteUpdatedBy>>,
    /// The Amazon Resource Name (ARN) for a third-party product that generated a finding in Security Hub. Documented below.
    #[builder(into)]
    #[serde(rename = "productArns")]
    pub r#product_arns: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaProductArn>>,
    /// Provides the name of the product that generated the finding. For control-based findings, the product name is Security Hub. Documented below.
    #[builder(into)]
    #[serde(rename = "productNames")]
    pub r#product_names: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaProductName>>,
    /// Provides the current state of a finding. Documented below.
    #[builder(into)]
    #[serde(rename = "recordStates")]
    pub r#record_states: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaRecordState>>,
    /// The product-generated identifier for a related finding.  Documented below.
    #[builder(into)]
    #[serde(rename = "relatedFindingsIds")]
    pub r#related_findings_ids: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaRelatedFindingsId>>,
    /// The ARN for the product that generated a related finding. Documented below.
    #[builder(into)]
    #[serde(rename = "relatedFindingsProductArns")]
    pub r#related_findings_product_arns: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaRelatedFindingsProductArn>>,
    /// The Amazon Resource Name (ARN) of the application that is related to a finding. Documented below.
    #[builder(into)]
    #[serde(rename = "resourceApplicationArns")]
    pub r#resource_application_arns: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourceApplicationArn>>,
    /// The name of the application that is related to a finding. Documented below.
    #[builder(into)]
    #[serde(rename = "resourceApplicationNames")]
    pub r#resource_application_names: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourceApplicationName>>,
    /// Custom fields and values about the resource that a finding pertains to. Documented below.
    #[builder(into)]
    #[serde(rename = "resourceDetailsOthers")]
    pub r#resource_details_others: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourceDetailsOther>>,
    /// The identifier for the given resource type. For AWS resources that are identified by Amazon Resource Names (ARNs), this is the ARN. For AWS resources that lack ARNs, this is the identifier as defined by the AWS service that created the resource. For non-AWS resources, this is a unique identifier that is associated with the resource. Documented below.
    #[builder(into)]
    #[serde(rename = "resourceIds")]
    pub r#resource_ids: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourceId>>,
    /// The partition in which the resource that the finding pertains to is located. A partition is a group of AWS Regions. Each AWS account is scoped to one partition. Documented below.
    #[builder(into)]
    #[serde(rename = "resourcePartitions")]
    pub r#resource_partitions: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourcePartition>>,
    /// The AWS Region where the resource that a finding pertains to is located. Documented below.
    #[builder(into)]
    #[serde(rename = "resourceRegions")]
    pub r#resource_regions: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourceRegion>>,
    /// A list of AWS tags associated with a resource at the time the finding was processed. Documented below.
    #[builder(into)]
    #[serde(rename = "resourceTags")]
    pub r#resource_tags: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourceTag>>,
    /// The type of resource that the finding pertains to. Documented below.
    #[builder(into)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaResourceType>>,
    /// The severity value of the finding. Documented below.
    #[builder(into)]
    #[serde(rename = "severityLabels")]
    pub r#severity_labels: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaSeverityLabel>>,
    /// Provides a URL that links to a page about the current finding in the finding product. Documented below.
    #[builder(into)]
    #[serde(rename = "sourceUrls")]
    pub r#source_urls: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaSourceUrl>>,
    /// A finding's title. Documented below.
    #[builder(into)]
    #[serde(rename = "titles")]
    pub r#titles: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaTitle>>,
    /// One or more finding types in the format of namespace/category/classifier that classify a finding. Documented below.
    #[builder(into)]
    #[serde(rename = "types")]
    pub r#types: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaType>>,
    /// A timestamp that indicates when the finding record was most recently updated. Documented below.
    #[builder(into)]
    #[serde(rename = "updatedAts")]
    pub r#updated_ats: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaUpdatedAt>>,
    /// A list of user-defined name and value string pairs added to a finding. Documented below.
    #[builder(into)]
    #[serde(rename = "userDefinedFields")]
    pub r#user_defined_fields: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaUserDefinedField>>,
    /// Provides the veracity of a finding. Documented below.
    #[builder(into)]
    #[serde(rename = "verificationStates")]
    pub r#verification_states: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaVerificationState>>,
    /// Provides information about the status of the investigation into a finding. Documented below.
    #[builder(into)]
    #[serde(rename = "workflowStatuses")]
    pub r#workflow_statuses: Option<Vec<super::super::types::securityhub::AutomationRuleCriteriaWorkflowStatus>>,
}
