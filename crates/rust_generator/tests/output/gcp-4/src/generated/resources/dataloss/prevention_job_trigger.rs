/// A job trigger configuration.
///
///
/// To get more information about JobTrigger, see:
///
/// * [API documentation](https://cloud.google.com/dlp/docs/reference/rest/v2/projects.jobTriggers)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dlp/docs/creating-job-triggers)
///
/// ## Example Usage
///
/// ### Dlp Job Trigger Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = prevention_job_trigger::create(
///         "basic",
///         PreventionJobTriggerArgs::builder()
///             .description("Description")
///             .display_name("Displayname")
///             .inspect_job(
///                 PreventionJobTriggerInspectJob::builder()
///                     .actions(
///                         vec![
///                             PreventionJobTriggerInspectJobAction::builder()
///                             .saveFindings(PreventionJobTriggerInspectJobActionSaveFindings::builder()
///                             .outputConfig(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfig::builder()
///                             .table(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfigTable::builder()
///                             .datasetId("dataset").projectId("project").build_struct())
///                             .build_struct()).build_struct()).build_struct(),
///                         ],
///                     )
///                     .inspectTemplateName("fake")
///                     .storageConfig(
///                         PreventionJobTriggerInspectJobStorageConfig::builder()
///                             .cloudStorageOptions(
///                                 PreventionJobTriggerInspectJobStorageConfigCloudStorageOptions::builder()
///                                     .fileSet(
///                                         PreventionJobTriggerInspectJobStorageConfigCloudStorageOptionsFileSet::builder()
///                                             .url("gs://mybucket/directory/")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .parent("projects/my-project-name")
///             .triggers(
///                 vec![
///                     PreventionJobTriggerTrigger::builder()
///                     .schedule(PreventionJobTriggerTriggerSchedule::builder()
///                     .recurrencePeriodDuration("86400s").build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dlp Job Trigger Bigquery Row Limit
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bigqueryRowLimit = prevention_job_trigger::create(
///         "bigqueryRowLimit",
///         PreventionJobTriggerArgs::builder()
///             .description("Description")
///             .display_name("Displayname")
///             .inspect_job(
///                 PreventionJobTriggerInspectJob::builder()
///                     .actions(
///                         vec![
///                             PreventionJobTriggerInspectJobAction::builder()
///                             .saveFindings(PreventionJobTriggerInspectJobActionSaveFindings::builder()
///                             .outputConfig(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfig::builder()
///                             .table(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfigTable::builder()
///                             .datasetId("dataset").projectId("project").build_struct())
///                             .build_struct()).build_struct()).build_struct(),
///                         ],
///                     )
///                     .inspectTemplateName("fake")
///                     .storageConfig(
///                         PreventionJobTriggerInspectJobStorageConfig::builder()
///                             .bigQueryOptions(
///                                 PreventionJobTriggerInspectJobStorageConfigBigQueryOptions::builder()
///                                     .rowsLimit(1000)
///                                     .sampleMethod("RANDOM_START")
///                                     .tableReference(
///                                         PreventionJobTriggerInspectJobStorageConfigBigQueryOptionsTableReference::builder()
///                                             .datasetId("dataset")
///                                             .projectId("project")
///                                             .tableId("table_to_scan")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .parent("projects/my-project-name")
///             .triggers(
///                 vec![
///                     PreventionJobTriggerTrigger::builder()
///                     .schedule(PreventionJobTriggerTriggerSchedule::builder()
///                     .recurrencePeriodDuration("86400s").build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dlp Job Trigger Bigquery Row Limit Percentage
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let bigqueryRowLimitPercentage = prevention_job_trigger::create(
///         "bigqueryRowLimitPercentage",
///         PreventionJobTriggerArgs::builder()
///             .description("Description")
///             .display_name("Displayname")
///             .inspect_job(
///                 PreventionJobTriggerInspectJob::builder()
///                     .actions(
///                         vec![
///                             PreventionJobTriggerInspectJobAction::builder()
///                             .saveFindings(PreventionJobTriggerInspectJobActionSaveFindings::builder()
///                             .outputConfig(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfig::builder()
///                             .table(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfigTable::builder()
///                             .datasetId("dataset").projectId("project").build_struct())
///                             .build_struct()).build_struct()).build_struct(),
///                         ],
///                     )
///                     .inspectTemplateName("fake")
///                     .storageConfig(
///                         PreventionJobTriggerInspectJobStorageConfig::builder()
///                             .bigQueryOptions(
///                                 PreventionJobTriggerInspectJobStorageConfigBigQueryOptions::builder()
///                                     .rowsLimitPercent(50)
///                                     .sampleMethod("RANDOM_START")
///                                     .tableReference(
///                                         PreventionJobTriggerInspectJobStorageConfigBigQueryOptionsTableReference::builder()
///                                             .datasetId("dataset")
///                                             .projectId("project")
///                                             .tableId("table_to_scan")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .parent("projects/my-project-name")
///             .triggers(
///                 vec![
///                     PreventionJobTriggerTrigger::builder()
///                     .schedule(PreventionJobTriggerTriggerSchedule::builder()
///                     .recurrencePeriodDuration("86400s").build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dlp Job Trigger Job Notification Emails
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let jobNotificationEmails = prevention_job_trigger::create(
///         "jobNotificationEmails",
///         PreventionJobTriggerArgs::builder()
///             .description("Description for the job_trigger created by terraform")
///             .display_name("TerraformDisplayName")
///             .inspect_job(
///                 PreventionJobTriggerInspectJob::builder()
///                     .actions(
///                         vec![
///                             PreventionJobTriggerInspectJobAction::builder()
///                             .jobNotificationEmails(PreventionJobTriggerInspectJobActionJobNotificationEmails::builder()
///                             .build_struct()).build_struct(),
///                         ],
///                     )
///                     .inspectTemplateName("sample-inspect-template")
///                     .storageConfig(
///                         PreventionJobTriggerInspectJobStorageConfig::builder()
///                             .cloudStorageOptions(
///                                 PreventionJobTriggerInspectJobStorageConfigCloudStorageOptions::builder()
///                                     .fileSet(
///                                         PreventionJobTriggerInspectJobStorageConfigCloudStorageOptionsFileSet::builder()
///                                             .url("gs://mybucket/directory/")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .parent("projects/my-project-name")
///             .triggers(
///                 vec![
///                     PreventionJobTriggerTrigger::builder()
///                     .schedule(PreventionJobTriggerTriggerSchedule::builder()
///                     .recurrencePeriodDuration("86400s").build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dlp Job Trigger Deidentify
///
///
/// ```yaml
/// resources:
///   deidentify:
///     type: gcp:dataloss:PreventionJobTrigger
///     properties:
///       parent: projects/my-project-name
///       description: Description for the job_trigger created by terraform
///       displayName: TerraformDisplayName
///       triggers:
///         - schedule:
///             recurrencePeriodDuration: 86400s
///       inspectJob:
///         inspectTemplateName: sample-inspect-template
///         actions:
///           - deidentify:
///               cloudStorageOutput: gs://samplebucket/dir/
///               fileTypesToTransforms:
///                 - CSV
///                 - TSV
///               transformationDetailsStorageConfig:
///                 table:
///                   projectId: my-project-name
///                   datasetId: ${default.datasetId}
///                   tableId: ${defaultTable.tableId}
///               transformationConfig:
///                 deidentifyTemplate: sample-deidentify-template
///                 imageRedactTemplate: sample-image-redact-template
///                 structuredDeidentifyTemplate: sample-structured-deidentify-template
///         storageConfig:
///           cloudStorageOptions:
///             fileSet:
///               url: gs://mybucket/directory/
///   default:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: tf_test
///       friendlyName: terraform-test
///       description: Description for the dataset created by terraform
///       location: US
///       defaultTableExpirationMs: 3.6e+06
///       labels:
///         env: default
///   defaultTable:
///     type: gcp:bigquery:Table
///     name: default
///     properties:
///       datasetId: ${default.datasetId}
///       tableId: tf_test
///       deletionProtection: false
///       timePartitioning:
///         type: DAY
///       labels:
///         env: default
///       schema: |2
///             [
///             {
///               "name": "quantity",
///               "type": "NUMERIC",
///               "mode": "NULLABLE",
///               "description": "The quantity"
///             },
///             {
///               "name": "name",
///               "type": "STRING",
///               "mode": "NULLABLE",
///               "description": "Name of the object"
///             }
///             ]
/// ```
/// ### Dlp Job Trigger Hybrid
///
///
/// ```yaml
/// resources:
///   hybridTrigger:
///     type: gcp:dataloss:PreventionJobTrigger
///     name: hybrid_trigger
///     properties:
///       parent: projects/my-project-name
///       triggers:
///         - manual: {}
///       inspectJob:
///         inspectTemplateName: fake
///         actions:
///           - saveFindings:
///               outputConfig:
///                 table:
///                   projectId: project
///                   datasetId: dataset
///         storageConfig:
///           hybridOptions:
///             description: Hybrid job trigger for data from the comments field of a table that contains customer appointment bookings
///             requiredFindingLabelKeys:
///               - appointment-bookings-comments
///             labels:
///               env: prod
///             tableOptions:
///               identifyingFields:
///                 - name: booking_id
/// ```
/// ### Dlp Job Trigger Inspect
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let inspect = prevention_job_trigger::create(
///         "inspect",
///         PreventionJobTriggerArgs::builder()
///             .description("Description")
///             .display_name("Displayname")
///             .inspect_job(
///                 PreventionJobTriggerInspectJob::builder()
///                     .actions(
///                         vec![
///                             PreventionJobTriggerInspectJobAction::builder()
///                             .saveFindings(PreventionJobTriggerInspectJobActionSaveFindings::builder()
///                             .outputConfig(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfig::builder()
///                             .table(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfigTable::builder()
///                             .datasetId("dataset").projectId("project").build_struct())
///                             .build_struct()).build_struct()).build_struct(),
///                         ],
///                     )
///                     .inspectConfig(
///                         PreventionJobTriggerInspectJobInspectConfig::builder()
///                             .customInfoTypes(
///                                 vec![
///                                     PreventionJobTriggerInspectJobInspectConfigCustomInfoType::builder()
///                                     .infoType(PreventionJobTriggerInspectJobInspectConfigCustomInfoTypeInfoType::builder()
///                                     .name("MY_CUSTOM_TYPE").build_struct())
///                                     .likelihood("UNLIKELY")
///                                     .regex(PreventionJobTriggerInspectJobInspectConfigCustomInfoTypeRegex::builder()
///                                     .pattern("test*").build_struct()).build_struct(),
///                                 ],
///                             )
///                             .infoTypes(
///                                 vec![
///                                     PreventionJobTriggerInspectJobInspectConfigInfoType::builder()
///                                     .name("EMAIL_ADDRESS").build_struct(),
///                                 ],
///                             )
///                             .limits(
///                                 PreventionJobTriggerInspectJobInspectConfigLimits::builder()
///                                     .maxFindingsPerItem(10)
///                                     .maxFindingsPerRequest(50)
///                                     .build_struct(),
///                             )
///                             .minLikelihood("UNLIKELY")
///                             .ruleSets(
///                                 vec![
///                                     PreventionJobTriggerInspectJobInspectConfigRuleSet::builder()
///                                     .infoTypes(vec![PreventionJobTriggerInspectJobInspectConfigRuleSetInfoType::builder()
///                                     .name("EMAIL_ADDRESS").build_struct(),])
///                                     .rules(vec![PreventionJobTriggerInspectJobInspectConfigRuleSetRule::builder()
///                                     .exclusionRule(PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRule::builder()
///                                     .matchingType("MATCHING_TYPE_FULL_MATCH")
///                                     .regex(PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleRegex::builder()
///                                     .pattern(".+@example.com").build_struct()).build_struct())
///                                     .build_struct(),]).build_struct(),
///                                     PreventionJobTriggerInspectJobInspectConfigRuleSet::builder()
///                                     .infoTypes(vec![PreventionJobTriggerInspectJobInspectConfigRuleSetInfoType::builder()
///                                     .name("MY_CUSTOM_TYPE").build_struct(),])
///                                     .rules(vec![PreventionJobTriggerInspectJobInspectConfigRuleSetRule::builder()
///                                     .hotwordRule(PreventionJobTriggerInspectJobInspectConfigRuleSetRuleHotwordRule::builder()
///                                     .hotwordRegex(PreventionJobTriggerInspectJobInspectConfigRuleSetRuleHotwordRuleHotwordRegex::builder()
///                                     .pattern("example*").build_struct())
///                                     .likelihoodAdjustment(PreventionJobTriggerInspectJobInspectConfigRuleSetRuleHotwordRuleLikelihoodAdjustment::builder()
///                                     .fixedLikelihood("VERY_LIKELY").build_struct())
///                                     .proximity(PreventionJobTriggerInspectJobInspectConfigRuleSetRuleHotwordRuleProximity::builder()
///                                     .windowBefore(50).build_struct()).build_struct())
///                                     .build_struct(),]).build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .inspectTemplateName("fake")
///                     .storageConfig(
///                         PreventionJobTriggerInspectJobStorageConfig::builder()
///                             .cloudStorageOptions(
///                                 PreventionJobTriggerInspectJobStorageConfigCloudStorageOptions::builder()
///                                     .fileSet(
///                                         PreventionJobTriggerInspectJobStorageConfigCloudStorageOptionsFileSet::builder()
///                                             .url("gs://mybucket/directory/")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .parent("projects/my-project-name")
///             .triggers(
///                 vec![
///                     PreventionJobTriggerTrigger::builder()
///                     .schedule(PreventionJobTriggerTriggerSchedule::builder()
///                     .recurrencePeriodDuration("86400s").build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dlp Job Trigger Publish To Stackdriver
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let publishToStackdriver = prevention_job_trigger::create(
///         "publishToStackdriver",
///         PreventionJobTriggerArgs::builder()
///             .description("Description for the job_trigger created by terraform")
///             .display_name("TerraformDisplayName")
///             .inspect_job(
///                 PreventionJobTriggerInspectJob::builder()
///                     .actions(
///                         vec![
///                             PreventionJobTriggerInspectJobAction::builder()
///                             .publishToStackdriver(PreventionJobTriggerInspectJobActionPublishToStackdriver::builder()
///                             .build_struct()).build_struct(),
///                         ],
///                     )
///                     .inspectTemplateName("sample-inspect-template")
///                     .storageConfig(
///                         PreventionJobTriggerInspectJobStorageConfig::builder()
///                             .cloudStorageOptions(
///                                 PreventionJobTriggerInspectJobStorageConfigCloudStorageOptions::builder()
///                                     .fileSet(
///                                         PreventionJobTriggerInspectJobStorageConfigCloudStorageOptionsFileSet::builder()
///                                             .url("gs://mybucket/directory/")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .parent("projects/my-project-name")
///             .triggers(
///                 vec![
///                     PreventionJobTriggerTrigger::builder()
///                     .schedule(PreventionJobTriggerTriggerSchedule::builder()
///                     .recurrencePeriodDuration("86400s").build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dlp Job Trigger With Id
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let withTriggerId = prevention_job_trigger::create(
///         "withTriggerId",
///         PreventionJobTriggerArgs::builder()
///             .description("Starting description")
///             .display_name("display")
///             .inspect_job(
///                 PreventionJobTriggerInspectJob::builder()
///                     .actions(
///                         vec![
///                             PreventionJobTriggerInspectJobAction::builder()
///                             .saveFindings(PreventionJobTriggerInspectJobActionSaveFindings::builder()
///                             .outputConfig(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfig::builder()
///                             .table(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfigTable::builder()
///                             .datasetId("dataset123").projectId("project").build_struct())
///                             .build_struct()).build_struct()).build_struct(),
///                         ],
///                     )
///                     .inspectTemplateName("fake")
///                     .storageConfig(
///                         PreventionJobTriggerInspectJobStorageConfig::builder()
///                             .cloudStorageOptions(
///                                 PreventionJobTriggerInspectJobStorageConfigCloudStorageOptions::builder()
///                                     .fileSet(
///                                         PreventionJobTriggerInspectJobStorageConfigCloudStorageOptionsFileSet::builder()
///                                             .url("gs://mybucket/directory/")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .parent("projects/my-project-name")
///             .trigger_id("id-")
///             .triggers(
///                 vec![
///                     PreventionJobTriggerTrigger::builder()
///                     .schedule(PreventionJobTriggerTriggerSchedule::builder()
///                     .recurrencePeriodDuration("86400s").build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dlp Job Trigger Multiple Actions
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = prevention_job_trigger::create(
///         "basic",
///         PreventionJobTriggerArgs::builder()
///             .description("Description")
///             .display_name("Displayname")
///             .inspect_job(
///                 PreventionJobTriggerInspectJob::builder()
///                     .actions(
///                         vec![
///                             PreventionJobTriggerInspectJobAction::builder()
///                             .saveFindings(PreventionJobTriggerInspectJobActionSaveFindings::builder()
///                             .outputConfig(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfig::builder()
///                             .table(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfigTable::builder()
///                             .datasetId("dataset").projectId("project").build_struct())
///                             .build_struct()).build_struct()).build_struct(),
///                             PreventionJobTriggerInspectJobAction::builder()
///                             .pubSub(PreventionJobTriggerInspectJobActionPubSub::builder()
///                             .topic("projects/project/topics/topic-name").build_struct())
///                             .build_struct(),
///                         ],
///                     )
///                     .inspectTemplateName("fake")
///                     .storageConfig(
///                         PreventionJobTriggerInspectJobStorageConfig::builder()
///                             .cloudStorageOptions(
///                                 PreventionJobTriggerInspectJobStorageConfigCloudStorageOptions::builder()
///                                     .fileSet(
///                                         PreventionJobTriggerInspectJobStorageConfigCloudStorageOptionsFileSet::builder()
///                                             .url("gs://mybucket/directory/")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .parent("projects/my-project-name")
///             .triggers(
///                 vec![
///                     PreventionJobTriggerTrigger::builder()
///                     .schedule(PreventionJobTriggerTriggerSchedule::builder()
///                     .recurrencePeriodDuration("86400s").build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dlp Job Trigger Cloud Storage Optional Timespan Autopopulation
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = prevention_job_trigger::create(
///         "basic",
///         PreventionJobTriggerArgs::builder()
///             .description("Description")
///             .display_name("Displayname")
///             .inspect_job(
///                 PreventionJobTriggerInspectJob::builder()
///                     .actions(
///                         vec![
///                             PreventionJobTriggerInspectJobAction::builder()
///                             .saveFindings(PreventionJobTriggerInspectJobActionSaveFindings::builder()
///                             .outputConfig(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfig::builder()
///                             .table(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfigTable::builder()
///                             .datasetId("dataset").projectId("project").build_struct())
///                             .build_struct()).build_struct()).build_struct(),
///                         ],
///                     )
///                     .inspectTemplateName("fake")
///                     .storageConfig(
///                         PreventionJobTriggerInspectJobStorageConfig::builder()
///                             .cloudStorageOptions(
///                                 PreventionJobTriggerInspectJobStorageConfigCloudStorageOptions::builder()
///                                     .fileSet(
///                                         PreventionJobTriggerInspectJobStorageConfigCloudStorageOptionsFileSet::builder()
///                                             .url("gs://mybucket/directory/")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .timespanConfig(
///                                 PreventionJobTriggerInspectJobStorageConfigTimespanConfig::builder()
///                                     .enableAutoPopulationOfTimespanConfig(true)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .parent("projects/my-project-name")
///             .triggers(
///                 vec![
///                     PreventionJobTriggerTrigger::builder()
///                     .schedule(PreventionJobTriggerTriggerSchedule::builder()
///                     .recurrencePeriodDuration("86400s").build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dlp Job Trigger Timespan Config Big Query
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let timespanConfigBigQuery = prevention_job_trigger::create(
///         "timespanConfigBigQuery",
///         PreventionJobTriggerArgs::builder()
///             .description("BigQuery DLP Job Trigger with timespan config and row limit")
///             .display_name("bigquery-dlp-job-trigger-limit-timespan")
///             .inspect_job(
///                 PreventionJobTriggerInspectJob::builder()
///                     .actions(
///                         vec![
///                             PreventionJobTriggerInspectJobAction::builder()
///                             .saveFindings(PreventionJobTriggerInspectJobActionSaveFindings::builder()
///                             .outputConfig(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfig::builder()
///                             .table(PreventionJobTriggerInspectJobActionSaveFindingsOutputConfigTable::builder()
///                             .datasetId("output").projectId("project").build_struct())
///                             .build_struct()).build_struct()).build_struct(),
///                         ],
///                     )
///                     .inspectTemplateName(
///                         "projects/test/locations/global/inspectTemplates/6425492983381733900",
///                     )
///                     .storageConfig(
///                         PreventionJobTriggerInspectJobStorageConfig::builder()
///                             .bigQueryOptions(
///                                 PreventionJobTriggerInspectJobStorageConfigBigQueryOptions::builder()
///                                     .sampleMethod("")
///                                     .tableReference(
///                                         PreventionJobTriggerInspectJobStorageConfigBigQueryOptionsTableReference::builder()
///                                             .datasetId("dataset")
///                                             .projectId("project")
///                                             .tableId("table")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .timespanConfig(
///                                 PreventionJobTriggerInspectJobStorageConfigTimespanConfig::builder()
///                                     .startTime("2023-01-01T00:00:23Z")
///                                     .timestampField(
///                                         PreventionJobTriggerInspectJobStorageConfigTimespanConfigTimestampField::builder()
///                                             .name("timestamp")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .parent("projects/my-project-name")
///             .triggers(
///                 vec![
///                     PreventionJobTriggerTrigger::builder()
///                     .schedule(PreventionJobTriggerTriggerSchedule::builder()
///                     .recurrencePeriodDuration("86400s").build_struct()).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// JobTrigger can be imported using any of these accepted formats:
///
/// * `{{parent}}/jobTriggers/{{name}}`
///
/// * `{{parent}}/{{name}}`
///
/// When using the `pulumi import` command, JobTrigger can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataloss/preventionJobTrigger:PreventionJobTrigger default {{parent}}/jobTriggers/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataloss/preventionJobTrigger:PreventionJobTrigger default {{parent}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod prevention_job_trigger {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PreventionJobTriggerArgs {
        /// A description of the job trigger.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User set display name of the job trigger.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Controls what and how to inspect for findings.
        #[builder(into, default)]
        pub inspect_job: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataloss::PreventionJobTriggerInspectJob>,
        >,
        /// The parent of the trigger, either in the format `projects/{{project}}`
        /// or `projects/{{project}}/locations/{{location}}`
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the trigger is currently active. Default value: "HEALTHY" Possible values: ["PAUSED", "HEALTHY", "CANCELLED"]
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The trigger id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular
        /// expression: [a-zA-Z\d-_]+. The maximum length is 100 characters. Can be empty to allow the system to generate one.
        #[builder(into, default)]
        pub trigger_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// What event needs to occur for a new job to be started.
        /// Structure is documented below.
        #[builder(into)]
        pub triggers: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::dataloss::PreventionJobTriggerTrigger>,
        >,
    }
    #[allow(dead_code)]
    pub struct PreventionJobTriggerResult {
        /// The creation timestamp of an inspectTemplate. Set by the server.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A description of the job trigger.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// User set display name of the job trigger.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Controls what and how to inspect for findings.
        pub inspect_job: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataloss::PreventionJobTriggerInspectJob>,
        >,
        /// The timestamp of the last time this trigger executed.
        pub last_run_time: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the job trigger. Set by the server.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parent of the trigger, either in the format `projects/{{project}}`
        /// or `projects/{{project}}/locations/{{location}}`
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Whether the trigger is currently active. Default value: "HEALTHY" Possible values: ["PAUSED", "HEALTHY", "CANCELLED"]
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// The trigger id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular
        /// expression: [a-zA-Z\d-_]+. The maximum length is 100 characters. Can be empty to allow the system to generate one.
        pub trigger_id: pulumi_gestalt_rust::Output<String>,
        /// What event needs to occur for a new job to be started.
        /// Structure is documented below.
        pub triggers: pulumi_gestalt_rust::Output<
            Vec<super::super::types::dataloss::PreventionJobTriggerTrigger>,
        >,
        /// The last update timestamp of an inspectTemplate. Set by the server.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PreventionJobTriggerArgs,
    ) -> PreventionJobTriggerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let inspect_job_binding = args.inspect_job.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let status_binding = args.status.get_output(context);
        let trigger_id_binding = args.trigger_id.get_output(context);
        let triggers_binding = args.triggers.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataloss/preventionJobTrigger:PreventionJobTrigger".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inspectJob".into(),
                    value: &inspect_job_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggerId".into(),
                    value: &trigger_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "triggers".into(),
                    value: &triggers_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PreventionJobTriggerResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            inspect_job: o.get_field("inspectJob"),
            last_run_time: o.get_field("lastRunTime"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            status: o.get_field("status"),
            trigger_id: o.get_field("triggerId"),
            triggers: o.get_field("triggers"),
            update_time: o.get_field("updateTime"),
        }
    }
}
