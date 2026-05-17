#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InsightFilters {
    /// AWS account ID that a finding is generated in. See String_Filter below for more details.
    #[builder(into)]
    #[serde(rename = "awsAccountIds")]
    pub r#aws_account_ids: Option<Vec<super::super::types::securityhub::InsightFiltersAwsAccountId>>,
    /// The name of the findings provider (company) that owns the solution (product) that generates findings. See String_Filter below for more details.
    #[builder(into)]
    #[serde(rename = "companyNames")]
    pub r#company_names: Option<Vec<super::super::types::securityhub::InsightFiltersCompanyName>>,
    /// Exclusive to findings that are generated as the result of a check run against a specific rule in a supported standard, such as CIS AWS Foundations. Contains security standard-related finding details. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "complianceStatuses")]
    pub r#compliance_statuses: Option<Vec<super::super::types::securityhub::InsightFiltersComplianceStatus>>,
    /// A finding's confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify. Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence. See Number Filter below for more details.
    #[builder(into)]
    #[serde(rename = "confidences")]
    pub r#confidences: Option<Vec<super::super::types::securityhub::InsightFiltersConfidence>>,
    /// An ISO8601-formatted timestamp that indicates when the security-findings provider captured the potential security issue that a finding captured. See Date Filter below for more details.
    #[builder(into)]
    #[serde(rename = "createdAts")]
    pub r#created_ats: Option<Vec<super::super::types::securityhub::InsightFiltersCreatedAt>>,
    /// The level of importance assigned to the resources associated with the finding. A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources. See Number Filter below for more details.
    #[builder(into)]
    #[serde(rename = "criticalities")]
    pub r#criticalities: Option<Vec<super::super::types::securityhub::InsightFiltersCriticality>>,
    /// A finding's description. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "descriptions")]
    pub r#descriptions: Option<Vec<super::super::types::securityhub::InsightFiltersDescription>>,
    /// The finding provider value for the finding confidence. Confidence is defined as the likelihood that a finding accurately identifies the behavior or issue that it was intended to identify. Confidence is scored on a 0-100 basis using a ratio scale, where 0 means zero percent confidence and 100 means 100 percent confidence. See Number Filter below for more details.
    #[builder(into)]
    #[serde(rename = "findingProviderFieldsConfidences")]
    pub r#finding_provider_fields_confidences: Option<Vec<super::super::types::securityhub::InsightFiltersFindingProviderFieldsConfidence>>,
    /// The finding provider value for the level of importance assigned to the resources associated with the findings. A score of 0 means that the underlying resources have no criticality, and a score of 100 is reserved for the most critical resources. See Number Filter below for more details.
    #[builder(into)]
    #[serde(rename = "findingProviderFieldsCriticalities")]
    pub r#finding_provider_fields_criticalities: Option<Vec<super::super::types::securityhub::InsightFiltersFindingProviderFieldsCriticality>>,
    /// The finding identifier of a related finding that is identified by the finding provider. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "findingProviderFieldsRelatedFindingsIds")]
    pub r#finding_provider_fields_related_findings_ids: Option<Vec<super::super::types::securityhub::InsightFiltersFindingProviderFieldsRelatedFindingsId>>,
    /// The ARN of the solution that generated a related finding that is identified by the finding provider. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "findingProviderFieldsRelatedFindingsProductArns")]
    pub r#finding_provider_fields_related_findings_product_arns: Option<Vec<super::super::types::securityhub::InsightFiltersFindingProviderFieldsRelatedFindingsProductArn>>,
    /// The finding provider value for the severity label. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "findingProviderFieldsSeverityLabels")]
    pub r#finding_provider_fields_severity_labels: Option<Vec<super::super::types::securityhub::InsightFiltersFindingProviderFieldsSeverityLabel>>,
    /// The finding provider's original value for the severity. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "findingProviderFieldsSeverityOriginals")]
    pub r#finding_provider_fields_severity_originals: Option<Vec<super::super::types::securityhub::InsightFiltersFindingProviderFieldsSeverityOriginal>>,
    /// One or more finding types that the finding provider assigned to the finding. Uses the format of `namespace/category/classifier` that classify a finding. Valid namespace values include: `Software and Configuration Checks`, `TTPs`, `Effects`, `Unusual Behaviors`, and `Sensitive Data Identifications`. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "findingProviderFieldsTypes")]
    pub r#finding_provider_fields_types: Option<Vec<super::super::types::securityhub::InsightFiltersFindingProviderFieldsType>>,
    /// An ISO8601-formatted timestamp that indicates when the security-findings provider first observed the potential security issue that a finding captured. See Date Filter below for more details.
    #[builder(into)]
    #[serde(rename = "firstObservedAts")]
    pub r#first_observed_ats: Option<Vec<super::super::types::securityhub::InsightFiltersFirstObservedAt>>,
    /// The identifier for the solution-specific component (a discrete unit of logic) that generated a finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "generatorIds")]
    pub r#generator_ids: Option<Vec<super::super::types::securityhub::InsightFiltersGeneratorId>>,
    /// The security findings provider-specific identifier for a finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "ids")]
    pub r#ids: Option<Vec<super::super::types::securityhub::InsightFiltersId>>,
    /// A keyword for a finding. See Keyword Filter below for more details.
    #[builder(into)]
    #[serde(rename = "keywords")]
    pub r#keywords: Option<Vec<super::super::types::securityhub::InsightFiltersKeyword>>,
    /// An ISO8601-formatted timestamp that indicates when the security-findings provider most recently observed the potential security issue that a finding captured. See Date Filter below for more details.
    #[builder(into)]
    #[serde(rename = "lastObservedAts")]
    pub r#last_observed_ats: Option<Vec<super::super::types::securityhub::InsightFiltersLastObservedAt>>,
    /// The name of the malware that was observed. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "malwareNames")]
    pub r#malware_names: Option<Vec<super::super::types::securityhub::InsightFiltersMalwareName>>,
    /// The filesystem path of the malware that was observed. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "malwarePaths")]
    pub r#malware_paths: Option<Vec<super::super::types::securityhub::InsightFiltersMalwarePath>>,
    /// The state of the malware that was observed. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "malwareStates")]
    pub r#malware_states: Option<Vec<super::super::types::securityhub::InsightFiltersMalwareState>>,
    /// The type of the malware that was observed. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "malwareTypes")]
    pub r#malware_types: Option<Vec<super::super::types::securityhub::InsightFiltersMalwareType>>,
    /// The destination domain of network-related information about a finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "networkDestinationDomains")]
    pub r#network_destination_domains: Option<Vec<super::super::types::securityhub::InsightFiltersNetworkDestinationDomain>>,
    /// The destination IPv4 address of network-related information about a finding. See Ip Filter below for more details.
    #[builder(into)]
    #[serde(rename = "networkDestinationIpv4s")]
    pub r#network_destination_ipv_4_s: Option<Vec<super::super::types::securityhub::InsightFiltersNetworkDestinationIpv4>>,
    /// The destination IPv6 address of network-related information about a finding. See Ip Filter below for more details.
    #[builder(into)]
    #[serde(rename = "networkDestinationIpv6s")]
    pub r#network_destination_ipv_6_s: Option<Vec<super::super::types::securityhub::InsightFiltersNetworkDestinationIpv6>>,
    /// The destination port of network-related information about a finding. See Number Filter below for more details.
    #[builder(into)]
    #[serde(rename = "networkDestinationPorts")]
    pub r#network_destination_ports: Option<Vec<super::super::types::securityhub::InsightFiltersNetworkDestinationPort>>,
    /// Indicates the direction of network traffic associated with a finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "networkDirections")]
    pub r#network_directions: Option<Vec<super::super::types::securityhub::InsightFiltersNetworkDirection>>,
    /// The protocol of network-related information about a finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "networkProtocols")]
    pub r#network_protocols: Option<Vec<super::super::types::securityhub::InsightFiltersNetworkProtocol>>,
    /// The source domain of network-related information about a finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "networkSourceDomains")]
    pub r#network_source_domains: Option<Vec<super::super::types::securityhub::InsightFiltersNetworkSourceDomain>>,
    /// The source IPv4 address of network-related information about a finding. See Ip Filter below for more details.
    #[builder(into)]
    #[serde(rename = "networkSourceIpv4s")]
    pub r#network_source_ipv_4_s: Option<Vec<super::super::types::securityhub::InsightFiltersNetworkSourceIpv4>>,
    /// The source IPv6 address of network-related information about a finding. See Ip Filter below for more details.
    #[builder(into)]
    #[serde(rename = "networkSourceIpv6s")]
    pub r#network_source_ipv_6_s: Option<Vec<super::super::types::securityhub::InsightFiltersNetworkSourceIpv6>>,
    /// The source media access control (MAC) address of network-related information about a finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "networkSourceMacs")]
    pub r#network_source_macs: Option<Vec<super::super::types::securityhub::InsightFiltersNetworkSourceMac>>,
    /// The source port of network-related information about a finding. See Number Filter below for more details.
    #[builder(into)]
    #[serde(rename = "networkSourcePorts")]
    pub r#network_source_ports: Option<Vec<super::super::types::securityhub::InsightFiltersNetworkSourcePort>>,
    /// The text of a note. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "noteTexts")]
    pub r#note_texts: Option<Vec<super::super::types::securityhub::InsightFiltersNoteText>>,
    /// The timestamp of when the note was updated. See Date Filter below for more details.
    #[builder(into)]
    #[serde(rename = "noteUpdatedAts")]
    pub r#note_updated_ats: Option<Vec<super::super::types::securityhub::InsightFiltersNoteUpdatedAt>>,
    /// The principal that created a note. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "noteUpdatedBies")]
    pub r#note_updated_bies: Option<Vec<super::super::types::securityhub::InsightFiltersNoteUpdatedBy>>,
    /// The date/time that the process was launched. See Date Filter below for more details.
    #[builder(into)]
    #[serde(rename = "processLaunchedAts")]
    pub r#process_launched_ats: Option<Vec<super::super::types::securityhub::InsightFiltersProcessLaunchedAt>>,
    /// The name of the process. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "processNames")]
    pub r#process_names: Option<Vec<super::super::types::securityhub::InsightFiltersProcessName>>,
    /// The parent process ID. See Number Filter below for more details.
    #[builder(into)]
    #[serde(rename = "processParentPids")]
    pub r#process_parent_pids: Option<Vec<super::super::types::securityhub::InsightFiltersProcessParentPid>>,
    /// The path to the process executable. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "processPaths")]
    pub r#process_paths: Option<Vec<super::super::types::securityhub::InsightFiltersProcessPath>>,
    /// The process ID. See Number Filter below for more details.
    #[builder(into)]
    #[serde(rename = "processPids")]
    pub r#process_pids: Option<Vec<super::super::types::securityhub::InsightFiltersProcessPid>>,
    /// The date/time that the process was terminated. See Date Filter below for more details.
    #[builder(into)]
    #[serde(rename = "processTerminatedAts")]
    pub r#process_terminated_ats: Option<Vec<super::super::types::securityhub::InsightFiltersProcessTerminatedAt>>,
    /// The ARN generated by Security Hub that uniquely identifies a third-party company (security findings provider) after this provider's product (solution that generates findings) is registered with Security Hub. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "productArns")]
    pub r#product_arns: Option<Vec<super::super::types::securityhub::InsightFiltersProductArn>>,
    /// A data type where security-findings providers can include additional solution-specific details that aren't part of the defined `AwsSecurityFinding` format. See Map Filter below for more details.
    #[builder(into)]
    #[serde(rename = "productFields")]
    pub r#product_fields: Option<Vec<super::super::types::securityhub::InsightFiltersProductField>>,
    /// The name of the solution (product) that generates findings. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "productNames")]
    pub r#product_names: Option<Vec<super::super::types::securityhub::InsightFiltersProductName>>,
    /// The recommendation of what to do about the issue described in a finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "recommendationTexts")]
    pub r#recommendation_texts: Option<Vec<super::super::types::securityhub::InsightFiltersRecommendationText>>,
    /// The updated record state for the finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "recordStates")]
    pub r#record_states: Option<Vec<super::super::types::securityhub::InsightFiltersRecordState>>,
    /// The solution-generated identifier for a related finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "relatedFindingsIds")]
    pub r#related_findings_ids: Option<Vec<super::super::types::securityhub::InsightFiltersRelatedFindingsId>>,
    /// The ARN of the solution that generated a related finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "relatedFindingsProductArns")]
    pub r#related_findings_product_arns: Option<Vec<super::super::types::securityhub::InsightFiltersRelatedFindingsProductArn>>,
    /// The IAM profile ARN of the instance. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceAwsEc2InstanceIamInstanceProfileArns")]
    pub r#resource_aws_ec_2_instance_iam_instance_profile_arns: Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceIamInstanceProfileArn>>,
    /// The Amazon Machine Image (AMI) ID of the instance. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceAwsEc2InstanceImageIds")]
    pub r#resource_aws_ec_2_instance_image_ids: Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceImageId>>,
    /// The IPv4 addresses associated with the instance. See Ip Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceAwsEc2InstanceIpv4Addresses")]
    pub r#resource_aws_ec_2_instance_ipv_4_addresses: Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceIpv4Address>>,
    /// The IPv6 addresses associated with the instance. See Ip Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceAwsEc2InstanceIpv6Addresses")]
    pub r#resource_aws_ec_2_instance_ipv_6_addresses: Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceIpv6Address>>,
    /// The key name associated with the instance. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceAwsEc2InstanceKeyNames")]
    pub r#resource_aws_ec_2_instance_key_names: Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceKeyName>>,
    /// The date and time the instance was launched. See Date Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceAwsEc2InstanceLaunchedAts")]
    pub r#resource_aws_ec_2_instance_launched_ats: Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceLaunchedAt>>,
    /// The identifier of the subnet that the instance was launched in. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceAwsEc2InstanceSubnetIds")]
    pub r#resource_aws_ec_2_instance_subnet_ids: Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceSubnetId>>,
    /// The instance type of the instance. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceAwsEc2InstanceTypes")]
    pub r#resource_aws_ec_2_instance_types: Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceType>>,
    /// The identifier of the VPC that the instance was launched in. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceAwsEc2InstanceVpcIds")]
    pub r#resource_aws_ec_2_instance_vpc_ids: Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsEc2InstanceVpcId>>,
    /// The creation date/time of the IAM access key related to a finding. See Date Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceAwsIamAccessKeyCreatedAts")]
    pub r#resource_aws_iam_access_key_created_ats: Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsIamAccessKeyCreatedAt>>,
    /// The status of the IAM access key related to a finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceAwsIamAccessKeyStatuses")]
    pub r#resource_aws_iam_access_key_statuses: Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsIamAccessKeyStatus>>,
    /// The user associated with the IAM access key related to a finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceAwsIamAccessKeyUserNames")]
    pub r#resource_aws_iam_access_key_user_names: Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsIamAccessKeyUserName>>,
    /// The canonical user ID of the owner of the S3 bucket. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceAwsS3BucketOwnerIds")]
    pub r#resource_aws_s_3_bucket_owner_ids: Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsS3BucketOwnerId>>,
    /// The display name of the owner of the S3 bucket. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceAwsS3BucketOwnerNames")]
    pub r#resource_aws_s_3_bucket_owner_names: Option<Vec<super::super::types::securityhub::InsightFiltersResourceAwsS3BucketOwnerName>>,
    /// The identifier of the image related to a finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceContainerImageIds")]
    pub r#resource_container_image_ids: Option<Vec<super::super::types::securityhub::InsightFiltersResourceContainerImageId>>,
    /// The name of the image related to a finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceContainerImageNames")]
    pub r#resource_container_image_names: Option<Vec<super::super::types::securityhub::InsightFiltersResourceContainerImageName>>,
    /// The date/time that the container was started. See Date Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceContainerLaunchedAts")]
    pub r#resource_container_launched_ats: Option<Vec<super::super::types::securityhub::InsightFiltersResourceContainerLaunchedAt>>,
    /// The name of the container related to a finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceContainerNames")]
    pub r#resource_container_names: Option<Vec<super::super::types::securityhub::InsightFiltersResourceContainerName>>,
    /// The details of a resource that doesn't have a specific subfield for the resource type defined. See Map Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceDetailsOthers")]
    pub r#resource_details_others: Option<Vec<super::super::types::securityhub::InsightFiltersResourceDetailsOther>>,
    /// The canonical identifier for the given resource type. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceIds")]
    pub r#resource_ids: Option<Vec<super::super::types::securityhub::InsightFiltersResourceId>>,
    /// The canonical AWS partition name that the Region is assigned to. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourcePartitions")]
    pub r#resource_partitions: Option<Vec<super::super::types::securityhub::InsightFiltersResourcePartition>>,
    /// The canonical AWS external Region name where this resource is located. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceRegions")]
    pub r#resource_regions: Option<Vec<super::super::types::securityhub::InsightFiltersResourceRegion>>,
    /// A list of AWS tags associated with a resource at the time the finding was processed. See Map Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceTags")]
    pub r#resource_tags: Option<Vec<super::super::types::securityhub::InsightFiltersResourceTag>>,
    /// Specifies the type of the resource that details are provided for. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Option<Vec<super::super::types::securityhub::InsightFiltersResourceType>>,
    /// The label of a finding's severity. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "severityLabels")]
    pub r#severity_labels: Option<Vec<super::super::types::securityhub::InsightFiltersSeverityLabel>>,
    /// A URL that links to a page about the current finding in the security-findings provider's solution. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "sourceUrls")]
    pub r#source_urls: Option<Vec<super::super::types::securityhub::InsightFiltersSourceUrl>>,
    /// The category of a threat intelligence indicator. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "threatIntelIndicatorCategories")]
    pub r#threat_intel_indicator_categories: Option<Vec<super::super::types::securityhub::InsightFiltersThreatIntelIndicatorCategory>>,
    /// The date/time of the last observation of a threat intelligence indicator. See Date Filter below for more details.
    #[builder(into)]
    #[serde(rename = "threatIntelIndicatorLastObservedAts")]
    pub r#threat_intel_indicator_last_observed_ats: Option<Vec<super::super::types::securityhub::InsightFiltersThreatIntelIndicatorLastObservedAt>>,
    /// The URL for more details from the source of the threat intelligence. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "threatIntelIndicatorSourceUrls")]
    pub r#threat_intel_indicator_source_urls: Option<Vec<super::super::types::securityhub::InsightFiltersThreatIntelIndicatorSourceUrl>>,
    /// The source of the threat intelligence. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "threatIntelIndicatorSources")]
    pub r#threat_intel_indicator_sources: Option<Vec<super::super::types::securityhub::InsightFiltersThreatIntelIndicatorSource>>,
    /// The type of a threat intelligence indicator. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "threatIntelIndicatorTypes")]
    pub r#threat_intel_indicator_types: Option<Vec<super::super::types::securityhub::InsightFiltersThreatIntelIndicatorType>>,
    /// The value of a threat intelligence indicator. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "threatIntelIndicatorValues")]
    pub r#threat_intel_indicator_values: Option<Vec<super::super::types::securityhub::InsightFiltersThreatIntelIndicatorValue>>,
    /// A finding's title. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "titles")]
    pub r#titles: Option<Vec<super::super::types::securityhub::InsightFiltersTitle>>,
    /// A finding type in the format of `namespace/category/classifier` that classifies a finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "types")]
    pub r#types: Option<Vec<super::super::types::securityhub::InsightFiltersType>>,
    /// An ISO8601-formatted timestamp that indicates when the security-findings provider last updated the finding record. See Date Filter below for more details.
    #[builder(into)]
    #[serde(rename = "updatedAts")]
    pub r#updated_ats: Option<Vec<super::super::types::securityhub::InsightFiltersUpdatedAt>>,
    /// A list of name/value string pairs associated with the finding. These are custom, user-defined fields added to a finding. See Map Filter below for more details.
    #[builder(into)]
    #[serde(rename = "userDefinedValues")]
    pub r#user_defined_values: Option<Vec<super::super::types::securityhub::InsightFiltersUserDefinedValue>>,
    /// The veracity of a finding. See String Filter below for more details.
    #[builder(into)]
    #[serde(rename = "verificationStates")]
    pub r#verification_states: Option<Vec<super::super::types::securityhub::InsightFiltersVerificationState>>,
    /// The status of the investigation into a finding. See Workflow Status Filter below for more details.
    #[builder(into)]
    #[serde(rename = "workflowStatuses")]
    pub r#workflow_statuses: Option<Vec<super::super::types::securityhub::InsightFiltersWorkflowStatus>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InsightFilters {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "aws_account_ids",
                    &self.r#aws_account_ids,
                ),
                to_pulumi_object_field(
                    "company_names",
                    &self.r#company_names,
                ),
                to_pulumi_object_field(
                    "compliance_statuses",
                    &self.r#compliance_statuses,
                ),
                to_pulumi_object_field(
                    "confidences",
                    &self.r#confidences,
                ),
                to_pulumi_object_field(
                    "created_ats",
                    &self.r#created_ats,
                ),
                to_pulumi_object_field(
                    "criticalities",
                    &self.r#criticalities,
                ),
                to_pulumi_object_field(
                    "descriptions",
                    &self.r#descriptions,
                ),
                to_pulumi_object_field(
                    "finding_provider_fields_confidences",
                    &self.r#finding_provider_fields_confidences,
                ),
                to_pulumi_object_field(
                    "finding_provider_fields_criticalities",
                    &self.r#finding_provider_fields_criticalities,
                ),
                to_pulumi_object_field(
                    "finding_provider_fields_related_findings_ids",
                    &self.r#finding_provider_fields_related_findings_ids,
                ),
                to_pulumi_object_field(
                    "finding_provider_fields_related_findings_product_arns",
                    &self.r#finding_provider_fields_related_findings_product_arns,
                ),
                to_pulumi_object_field(
                    "finding_provider_fields_severity_labels",
                    &self.r#finding_provider_fields_severity_labels,
                ),
                to_pulumi_object_field(
                    "finding_provider_fields_severity_originals",
                    &self.r#finding_provider_fields_severity_originals,
                ),
                to_pulumi_object_field(
                    "finding_provider_fields_types",
                    &self.r#finding_provider_fields_types,
                ),
                to_pulumi_object_field(
                    "first_observed_ats",
                    &self.r#first_observed_ats,
                ),
                to_pulumi_object_field(
                    "generator_ids",
                    &self.r#generator_ids,
                ),
                to_pulumi_object_field(
                    "ids",
                    &self.r#ids,
                ),
                to_pulumi_object_field(
                    "keywords",
                    &self.r#keywords,
                ),
                to_pulumi_object_field(
                    "last_observed_ats",
                    &self.r#last_observed_ats,
                ),
                to_pulumi_object_field(
                    "malware_names",
                    &self.r#malware_names,
                ),
                to_pulumi_object_field(
                    "malware_paths",
                    &self.r#malware_paths,
                ),
                to_pulumi_object_field(
                    "malware_states",
                    &self.r#malware_states,
                ),
                to_pulumi_object_field(
                    "malware_types",
                    &self.r#malware_types,
                ),
                to_pulumi_object_field(
                    "network_destination_domains",
                    &self.r#network_destination_domains,
                ),
                to_pulumi_object_field(
                    "network_destination_ipv_4_s",
                    &self.r#network_destination_ipv_4_s,
                ),
                to_pulumi_object_field(
                    "network_destination_ipv_6_s",
                    &self.r#network_destination_ipv_6_s,
                ),
                to_pulumi_object_field(
                    "network_destination_ports",
                    &self.r#network_destination_ports,
                ),
                to_pulumi_object_field(
                    "network_directions",
                    &self.r#network_directions,
                ),
                to_pulumi_object_field(
                    "network_protocols",
                    &self.r#network_protocols,
                ),
                to_pulumi_object_field(
                    "network_source_domains",
                    &self.r#network_source_domains,
                ),
                to_pulumi_object_field(
                    "network_source_ipv_4_s",
                    &self.r#network_source_ipv_4_s,
                ),
                to_pulumi_object_field(
                    "network_source_ipv_6_s",
                    &self.r#network_source_ipv_6_s,
                ),
                to_pulumi_object_field(
                    "network_source_macs",
                    &self.r#network_source_macs,
                ),
                to_pulumi_object_field(
                    "network_source_ports",
                    &self.r#network_source_ports,
                ),
                to_pulumi_object_field(
                    "note_texts",
                    &self.r#note_texts,
                ),
                to_pulumi_object_field(
                    "note_updated_ats",
                    &self.r#note_updated_ats,
                ),
                to_pulumi_object_field(
                    "note_updated_bies",
                    &self.r#note_updated_bies,
                ),
                to_pulumi_object_field(
                    "process_launched_ats",
                    &self.r#process_launched_ats,
                ),
                to_pulumi_object_field(
                    "process_names",
                    &self.r#process_names,
                ),
                to_pulumi_object_field(
                    "process_parent_pids",
                    &self.r#process_parent_pids,
                ),
                to_pulumi_object_field(
                    "process_paths",
                    &self.r#process_paths,
                ),
                to_pulumi_object_field(
                    "process_pids",
                    &self.r#process_pids,
                ),
                to_pulumi_object_field(
                    "process_terminated_ats",
                    &self.r#process_terminated_ats,
                ),
                to_pulumi_object_field(
                    "product_arns",
                    &self.r#product_arns,
                ),
                to_pulumi_object_field(
                    "product_fields",
                    &self.r#product_fields,
                ),
                to_pulumi_object_field(
                    "product_names",
                    &self.r#product_names,
                ),
                to_pulumi_object_field(
                    "recommendation_texts",
                    &self.r#recommendation_texts,
                ),
                to_pulumi_object_field(
                    "record_states",
                    &self.r#record_states,
                ),
                to_pulumi_object_field(
                    "related_findings_ids",
                    &self.r#related_findings_ids,
                ),
                to_pulumi_object_field(
                    "related_findings_product_arns",
                    &self.r#related_findings_product_arns,
                ),
                to_pulumi_object_field(
                    "resource_aws_ec_2_instance_iam_instance_profile_arns",
                    &self.r#resource_aws_ec_2_instance_iam_instance_profile_arns,
                ),
                to_pulumi_object_field(
                    "resource_aws_ec_2_instance_image_ids",
                    &self.r#resource_aws_ec_2_instance_image_ids,
                ),
                to_pulumi_object_field(
                    "resource_aws_ec_2_instance_ipv_4_addresses",
                    &self.r#resource_aws_ec_2_instance_ipv_4_addresses,
                ),
                to_pulumi_object_field(
                    "resource_aws_ec_2_instance_ipv_6_addresses",
                    &self.r#resource_aws_ec_2_instance_ipv_6_addresses,
                ),
                to_pulumi_object_field(
                    "resource_aws_ec_2_instance_key_names",
                    &self.r#resource_aws_ec_2_instance_key_names,
                ),
                to_pulumi_object_field(
                    "resource_aws_ec_2_instance_launched_ats",
                    &self.r#resource_aws_ec_2_instance_launched_ats,
                ),
                to_pulumi_object_field(
                    "resource_aws_ec_2_instance_subnet_ids",
                    &self.r#resource_aws_ec_2_instance_subnet_ids,
                ),
                to_pulumi_object_field(
                    "resource_aws_ec_2_instance_types",
                    &self.r#resource_aws_ec_2_instance_types,
                ),
                to_pulumi_object_field(
                    "resource_aws_ec_2_instance_vpc_ids",
                    &self.r#resource_aws_ec_2_instance_vpc_ids,
                ),
                to_pulumi_object_field(
                    "resource_aws_iam_access_key_created_ats",
                    &self.r#resource_aws_iam_access_key_created_ats,
                ),
                to_pulumi_object_field(
                    "resource_aws_iam_access_key_statuses",
                    &self.r#resource_aws_iam_access_key_statuses,
                ),
                to_pulumi_object_field(
                    "resource_aws_iam_access_key_user_names",
                    &self.r#resource_aws_iam_access_key_user_names,
                ),
                to_pulumi_object_field(
                    "resource_aws_s_3_bucket_owner_ids",
                    &self.r#resource_aws_s_3_bucket_owner_ids,
                ),
                to_pulumi_object_field(
                    "resource_aws_s_3_bucket_owner_names",
                    &self.r#resource_aws_s_3_bucket_owner_names,
                ),
                to_pulumi_object_field(
                    "resource_container_image_ids",
                    &self.r#resource_container_image_ids,
                ),
                to_pulumi_object_field(
                    "resource_container_image_names",
                    &self.r#resource_container_image_names,
                ),
                to_pulumi_object_field(
                    "resource_container_launched_ats",
                    &self.r#resource_container_launched_ats,
                ),
                to_pulumi_object_field(
                    "resource_container_names",
                    &self.r#resource_container_names,
                ),
                to_pulumi_object_field(
                    "resource_details_others",
                    &self.r#resource_details_others,
                ),
                to_pulumi_object_field(
                    "resource_ids",
                    &self.r#resource_ids,
                ),
                to_pulumi_object_field(
                    "resource_partitions",
                    &self.r#resource_partitions,
                ),
                to_pulumi_object_field(
                    "resource_regions",
                    &self.r#resource_regions,
                ),
                to_pulumi_object_field(
                    "resource_tags",
                    &self.r#resource_tags,
                ),
                to_pulumi_object_field(
                    "resource_types",
                    &self.r#resource_types,
                ),
                to_pulumi_object_field(
                    "severity_labels",
                    &self.r#severity_labels,
                ),
                to_pulumi_object_field(
                    "source_urls",
                    &self.r#source_urls,
                ),
                to_pulumi_object_field(
                    "threat_intel_indicator_categories",
                    &self.r#threat_intel_indicator_categories,
                ),
                to_pulumi_object_field(
                    "threat_intel_indicator_last_observed_ats",
                    &self.r#threat_intel_indicator_last_observed_ats,
                ),
                to_pulumi_object_field(
                    "threat_intel_indicator_source_urls",
                    &self.r#threat_intel_indicator_source_urls,
                ),
                to_pulumi_object_field(
                    "threat_intel_indicator_sources",
                    &self.r#threat_intel_indicator_sources,
                ),
                to_pulumi_object_field(
                    "threat_intel_indicator_types",
                    &self.r#threat_intel_indicator_types,
                ),
                to_pulumi_object_field(
                    "threat_intel_indicator_values",
                    &self.r#threat_intel_indicator_values,
                ),
                to_pulumi_object_field(
                    "titles",
                    &self.r#titles,
                ),
                to_pulumi_object_field(
                    "types",
                    &self.r#types,
                ),
                to_pulumi_object_field(
                    "updated_ats",
                    &self.r#updated_ats,
                ),
                to_pulumi_object_field(
                    "user_defined_values",
                    &self.r#user_defined_values,
                ),
                to_pulumi_object_field(
                    "verification_states",
                    &self.r#verification_states,
                ),
                to_pulumi_object_field(
                    "workflow_statuses",
                    &self.r#workflow_statuses,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InsightFilters {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#aws_account_ids: {
                        let field_value = match fields_map.get("aws_account_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_account_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#company_names: {
                        let field_value = match fields_map.get("company_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'company_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compliance_statuses: {
                        let field_value = match fields_map.get("compliance_statuses") {
                            Some(value) => value,
                            None => bail!("Missing field 'compliance_statuses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#confidences: {
                        let field_value = match fields_map.get("confidences") {
                            Some(value) => value,
                            None => bail!("Missing field 'confidences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#created_ats: {
                        let field_value = match fields_map.get("created_ats") {
                            Some(value) => value,
                            None => bail!("Missing field 'created_ats' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#criticalities: {
                        let field_value = match fields_map.get("criticalities") {
                            Some(value) => value,
                            None => bail!("Missing field 'criticalities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#descriptions: {
                        let field_value = match fields_map.get("descriptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'descriptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#finding_provider_fields_confidences: {
                        let field_value = match fields_map.get("finding_provider_fields_confidences") {
                            Some(value) => value,
                            None => bail!("Missing field 'finding_provider_fields_confidences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#finding_provider_fields_criticalities: {
                        let field_value = match fields_map.get("finding_provider_fields_criticalities") {
                            Some(value) => value,
                            None => bail!("Missing field 'finding_provider_fields_criticalities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#finding_provider_fields_related_findings_ids: {
                        let field_value = match fields_map.get("finding_provider_fields_related_findings_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'finding_provider_fields_related_findings_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#finding_provider_fields_related_findings_product_arns: {
                        let field_value = match fields_map.get("finding_provider_fields_related_findings_product_arns") {
                            Some(value) => value,
                            None => bail!("Missing field 'finding_provider_fields_related_findings_product_arns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#finding_provider_fields_severity_labels: {
                        let field_value = match fields_map.get("finding_provider_fields_severity_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'finding_provider_fields_severity_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#finding_provider_fields_severity_originals: {
                        let field_value = match fields_map.get("finding_provider_fields_severity_originals") {
                            Some(value) => value,
                            None => bail!("Missing field 'finding_provider_fields_severity_originals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#finding_provider_fields_types: {
                        let field_value = match fields_map.get("finding_provider_fields_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'finding_provider_fields_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#first_observed_ats: {
                        let field_value = match fields_map.get("first_observed_ats") {
                            Some(value) => value,
                            None => bail!("Missing field 'first_observed_ats' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#generator_ids: {
                        let field_value = match fields_map.get("generator_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'generator_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ids: {
                        let field_value = match fields_map.get("ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#keywords: {
                        let field_value = match fields_map.get("keywords") {
                            Some(value) => value,
                            None => bail!("Missing field 'keywords' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_observed_ats: {
                        let field_value = match fields_map.get("last_observed_ats") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_observed_ats' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#malware_names: {
                        let field_value = match fields_map.get("malware_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'malware_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#malware_paths: {
                        let field_value = match fields_map.get("malware_paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'malware_paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#malware_states: {
                        let field_value = match fields_map.get("malware_states") {
                            Some(value) => value,
                            None => bail!("Missing field 'malware_states' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#malware_types: {
                        let field_value = match fields_map.get("malware_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'malware_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_destination_domains: {
                        let field_value = match fields_map.get("network_destination_domains") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_destination_domains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_destination_ipv_4_s: {
                        let field_value = match fields_map.get("network_destination_ipv_4_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_destination_ipv_4_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_destination_ipv_6_s: {
                        let field_value = match fields_map.get("network_destination_ipv_6_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_destination_ipv_6_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_destination_ports: {
                        let field_value = match fields_map.get("network_destination_ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_destination_ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_directions: {
                        let field_value = match fields_map.get("network_directions") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_directions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_protocols: {
                        let field_value = match fields_map.get("network_protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_source_domains: {
                        let field_value = match fields_map.get("network_source_domains") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_source_domains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_source_ipv_4_s: {
                        let field_value = match fields_map.get("network_source_ipv_4_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_source_ipv_4_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_source_ipv_6_s: {
                        let field_value = match fields_map.get("network_source_ipv_6_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_source_ipv_6_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_source_macs: {
                        let field_value = match fields_map.get("network_source_macs") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_source_macs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_source_ports: {
                        let field_value = match fields_map.get("network_source_ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_source_ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#note_texts: {
                        let field_value = match fields_map.get("note_texts") {
                            Some(value) => value,
                            None => bail!("Missing field 'note_texts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#note_updated_ats: {
                        let field_value = match fields_map.get("note_updated_ats") {
                            Some(value) => value,
                            None => bail!("Missing field 'note_updated_ats' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#note_updated_bies: {
                        let field_value = match fields_map.get("note_updated_bies") {
                            Some(value) => value,
                            None => bail!("Missing field 'note_updated_bies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#process_launched_ats: {
                        let field_value = match fields_map.get("process_launched_ats") {
                            Some(value) => value,
                            None => bail!("Missing field 'process_launched_ats' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#process_names: {
                        let field_value = match fields_map.get("process_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'process_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#process_parent_pids: {
                        let field_value = match fields_map.get("process_parent_pids") {
                            Some(value) => value,
                            None => bail!("Missing field 'process_parent_pids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#process_paths: {
                        let field_value = match fields_map.get("process_paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'process_paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#process_pids: {
                        let field_value = match fields_map.get("process_pids") {
                            Some(value) => value,
                            None => bail!("Missing field 'process_pids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#process_terminated_ats: {
                        let field_value = match fields_map.get("process_terminated_ats") {
                            Some(value) => value,
                            None => bail!("Missing field 'process_terminated_ats' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#product_arns: {
                        let field_value = match fields_map.get("product_arns") {
                            Some(value) => value,
                            None => bail!("Missing field 'product_arns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#product_fields: {
                        let field_value = match fields_map.get("product_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'product_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#product_names: {
                        let field_value = match fields_map.get("product_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'product_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recommendation_texts: {
                        let field_value = match fields_map.get("recommendation_texts") {
                            Some(value) => value,
                            None => bail!("Missing field 'recommendation_texts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_states: {
                        let field_value = match fields_map.get("record_states") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_states' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#related_findings_ids: {
                        let field_value = match fields_map.get("related_findings_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'related_findings_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#related_findings_product_arns: {
                        let field_value = match fields_map.get("related_findings_product_arns") {
                            Some(value) => value,
                            None => bail!("Missing field 'related_findings_product_arns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_aws_ec_2_instance_iam_instance_profile_arns: {
                        let field_value = match fields_map.get("resource_aws_ec_2_instance_iam_instance_profile_arns") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_aws_ec_2_instance_iam_instance_profile_arns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_aws_ec_2_instance_image_ids: {
                        let field_value = match fields_map.get("resource_aws_ec_2_instance_image_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_aws_ec_2_instance_image_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_aws_ec_2_instance_ipv_4_addresses: {
                        let field_value = match fields_map.get("resource_aws_ec_2_instance_ipv_4_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_aws_ec_2_instance_ipv_4_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_aws_ec_2_instance_ipv_6_addresses: {
                        let field_value = match fields_map.get("resource_aws_ec_2_instance_ipv_6_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_aws_ec_2_instance_ipv_6_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_aws_ec_2_instance_key_names: {
                        let field_value = match fields_map.get("resource_aws_ec_2_instance_key_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_aws_ec_2_instance_key_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_aws_ec_2_instance_launched_ats: {
                        let field_value = match fields_map.get("resource_aws_ec_2_instance_launched_ats") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_aws_ec_2_instance_launched_ats' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_aws_ec_2_instance_subnet_ids: {
                        let field_value = match fields_map.get("resource_aws_ec_2_instance_subnet_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_aws_ec_2_instance_subnet_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_aws_ec_2_instance_types: {
                        let field_value = match fields_map.get("resource_aws_ec_2_instance_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_aws_ec_2_instance_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_aws_ec_2_instance_vpc_ids: {
                        let field_value = match fields_map.get("resource_aws_ec_2_instance_vpc_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_aws_ec_2_instance_vpc_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_aws_iam_access_key_created_ats: {
                        let field_value = match fields_map.get("resource_aws_iam_access_key_created_ats") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_aws_iam_access_key_created_ats' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_aws_iam_access_key_statuses: {
                        let field_value = match fields_map.get("resource_aws_iam_access_key_statuses") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_aws_iam_access_key_statuses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_aws_iam_access_key_user_names: {
                        let field_value = match fields_map.get("resource_aws_iam_access_key_user_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_aws_iam_access_key_user_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_aws_s_3_bucket_owner_ids: {
                        let field_value = match fields_map.get("resource_aws_s_3_bucket_owner_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_aws_s_3_bucket_owner_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_aws_s_3_bucket_owner_names: {
                        let field_value = match fields_map.get("resource_aws_s_3_bucket_owner_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_aws_s_3_bucket_owner_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_container_image_ids: {
                        let field_value = match fields_map.get("resource_container_image_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_container_image_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_container_image_names: {
                        let field_value = match fields_map.get("resource_container_image_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_container_image_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_container_launched_ats: {
                        let field_value = match fields_map.get("resource_container_launched_ats") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_container_launched_ats' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_container_names: {
                        let field_value = match fields_map.get("resource_container_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_container_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_details_others: {
                        let field_value = match fields_map.get("resource_details_others") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_details_others' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_ids: {
                        let field_value = match fields_map.get("resource_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_partitions: {
                        let field_value = match fields_map.get("resource_partitions") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_partitions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_regions: {
                        let field_value = match fields_map.get("resource_regions") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_regions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_tags: {
                        let field_value = match fields_map.get("resource_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_types: {
                        let field_value = match fields_map.get("resource_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#severity_labels: {
                        let field_value = match fields_map.get("severity_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'severity_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_urls: {
                        let field_value = match fields_map.get("source_urls") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_urls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threat_intel_indicator_categories: {
                        let field_value = match fields_map.get("threat_intel_indicator_categories") {
                            Some(value) => value,
                            None => bail!("Missing field 'threat_intel_indicator_categories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threat_intel_indicator_last_observed_ats: {
                        let field_value = match fields_map.get("threat_intel_indicator_last_observed_ats") {
                            Some(value) => value,
                            None => bail!("Missing field 'threat_intel_indicator_last_observed_ats' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threat_intel_indicator_source_urls: {
                        let field_value = match fields_map.get("threat_intel_indicator_source_urls") {
                            Some(value) => value,
                            None => bail!("Missing field 'threat_intel_indicator_source_urls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threat_intel_indicator_sources: {
                        let field_value = match fields_map.get("threat_intel_indicator_sources") {
                            Some(value) => value,
                            None => bail!("Missing field 'threat_intel_indicator_sources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threat_intel_indicator_types: {
                        let field_value = match fields_map.get("threat_intel_indicator_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'threat_intel_indicator_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threat_intel_indicator_values: {
                        let field_value = match fields_map.get("threat_intel_indicator_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'threat_intel_indicator_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#titles: {
                        let field_value = match fields_map.get("titles") {
                            Some(value) => value,
                            None => bail!("Missing field 'titles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#types: {
                        let field_value = match fields_map.get("types") {
                            Some(value) => value,
                            None => bail!("Missing field 'types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#updated_ats: {
                        let field_value = match fields_map.get("updated_ats") {
                            Some(value) => value,
                            None => bail!("Missing field 'updated_ats' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_defined_values: {
                        let field_value = match fields_map.get("user_defined_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_defined_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#verification_states: {
                        let field_value = match fields_map.get("verification_states") {
                            Some(value) => value,
                            None => bail!("Missing field 'verification_states' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workflow_statuses: {
                        let field_value = match fields_map.get("workflow_statuses") {
                            Some(value) => value,
                            None => bail!("Missing field 'workflow_statuses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
