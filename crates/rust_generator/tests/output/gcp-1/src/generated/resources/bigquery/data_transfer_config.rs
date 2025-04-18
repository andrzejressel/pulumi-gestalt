/// Represents a data transfer configuration. A transfer configuration
/// contains all metadata needed to perform a data transfer.
///
///
/// To get more information about Config, see:
///
/// * [API documentation](https://cloud.google.com/bigquery/docs/reference/datatransfer/rest/v1/projects.locations.transferConfigs/create)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/bigquery/docs/reference/datatransfer/rest/)
///
///
///
/// ## Example Usage
///
/// ### Bigquerydatatransfer Config Scheduled Query
///
///
/// ```yaml
/// resources:
///   permissions:
///     type: gcp:projects:IAMMember
///     properties:
///       project: ${project.projectId}
///       role: roles/iam.serviceAccountTokenCreator
///       member: serviceAccount:service-${project.number}@gcp-sa-bigquerydatatransfer.iam.gserviceaccount.com
///   queryConfig:
///     type: gcp:bigquery:DataTransferConfig
///     name: query_config
///     properties:
///       displayName: my-query
///       location: asia-northeast1
///       dataSourceId: scheduled_query
///       schedule: first sunday of quarter 00:00
///       destinationDatasetId: ${myDataset.datasetId}
///       params:
///         destination_table_name_template: my_table
///         write_disposition: WRITE_APPEND
///         query: SELECT name FROM tabl WHERE x = 'y'
///     options:
///       dependsOn:
///         - ${permissions}
///   myDataset:
///     type: gcp:bigquery:Dataset
///     name: my_dataset
///     properties:
///       datasetId: my_dataset
///       friendlyName: foo
///       description: bar
///       location: asia-northeast1
///     options:
///       dependsOn:
///         - ${permissions}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Bigquerydatatransfer Config Cmek
///
///
/// ```yaml
/// resources:
///   permissions:
///     type: gcp:projects:IAMMember
///     properties:
///       project: ${project.projectId}
///       role: roles/iam.serviceAccountTokenCreator
///       member: serviceAccount:service-${project.number}@gcp-sa-bigquerydatatransfer.iam.gserviceaccount.com
///   queryConfigCmek:
///     type: gcp:bigquery:DataTransferConfig
///     name: query_config_cmek
///     properties:
///       displayName: ""
///       location: asia-northeast1
///       dataSourceId: scheduled_query
///       schedule: first sunday of quarter 00:00
///       destinationDatasetId: ${myDataset.datasetId}
///       params:
///         destination_table_name_template: my_table
///         write_disposition: WRITE_APPEND
///         query: SELECT name FROM tabl WHERE x = 'y'
///       encryptionConfiguration:
///         kmsKeyName: ${cryptoKey.id}
///     options:
///       dependsOn:
///         - ${permissions}
///   myDataset:
///     type: gcp:bigquery:Dataset
///     name: my_dataset
///     properties:
///       datasetId: example_dataset
///       friendlyName: foo
///       description: bar
///       location: asia-northeast1
///     options:
///       dependsOn:
///         - ${permissions}
///   cryptoKey:
///     type: gcp:kms:CryptoKey
///     name: crypto_key
///     properties:
///       name: example-key
///       keyRing: ${keyRing.id}
///   keyRing:
///     type: gcp:kms:KeyRing
///     name: key_ring
///     properties:
///       name: example-keyring
///       location: us
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Bigquerydatatransfer Config Salesforce
///
///
/// ```yaml
/// resources:
///   myDataset:
///     type: gcp:bigquery:Dataset
///     name: my_dataset
///     properties:
///       datasetId: my_dataset
///       description: My dataset
///       location: asia-northeast1
///   salesforceConfig:
///     type: gcp:bigquery:DataTransferConfig
///     name: salesforce_config
///     properties:
///       displayName: my-salesforce-config
///       location: asia-northeast1
///       dataSourceId: salesforce
///       schedule: first sunday of quarter 00:00
///       destinationDatasetId: ${myDataset.datasetId}
///       params:
///         connector.authentication.oauth.clientId: client-id
///         connector.authentication.oauth.clientSecret: client-secret
///         connector.authentication.oauth.myDomain: MyDomainName
///         assets: '["asset-a","asset-b"]'
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Config can be imported using any of these accepted formats:
///
/// * `{{project}}/{{name}}`
///
/// * `{{project}} {{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Config can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigquery/dataTransferConfig:DataTransferConfig default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/dataTransferConfig:DataTransferConfig default "{{project}} {{name}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/dataTransferConfig:DataTransferConfig default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_transfer_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataTransferConfigArgs {
        /// The number of days to look back to automatically refresh the data.
        /// For example, if dataRefreshWindowDays = 10, then every day BigQuery
        /// reingests data for [today-10, today-1], rather than ingesting data for
        /// just [today-1]. Only valid if the data source supports the feature.
        /// Set the value to 0 to use the default value.
        #[builder(into, default)]
        pub data_refresh_window_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The data source id. Cannot be changed once the transfer config is created.
        #[builder(into)]
        pub data_source_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The BigQuery target dataset id.
        #[builder(into, default)]
        pub destination_dataset_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When set to true, no runs are scheduled for a given transfer.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The user specified display name for the transfer config.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Email notifications will be sent according to these preferences to the
        /// email address of the user who owns this transfer config.
        /// Structure is documented below.
        #[builder(into, default)]
        pub email_preferences: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::DataTransferConfigEmailPreferences>,
        >,
        /// Represents the encryption configuration for a transfer.
        /// Structure is documented below.
        #[builder(into, default)]
        pub encryption_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::bigquery::DataTransferConfigEncryptionConfiguration,
            >,
        >,
        /// The geographic location where the transfer config should reside.
        /// Examples: US, EU, asia-northeast1. The default value is US.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Pub/Sub topic where notifications will be sent after transfer runs
        /// associated with this transfer config finish.
        #[builder(into, default)]
        pub notification_pubsub_topic: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Parameters specific to each data source. For more information see the bq tab in the 'Setting up a data transfer'
        /// section for each data source. For example the parameters for Cloud Storage transfers are listed here:
        /// https://cloud.google.com/bigquery-transfer/docs/cloud-storage-transfer#bq
        /// **NOTE** : If you are attempting to update a parameter that cannot be updated (due to api limitations) please force recreation of the resource.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub params: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Data transfer schedule. If the data source does not support a custom
        /// schedule, this should be empty. If it is empty, the default value for
        /// the data source will be used. The specified times are in UTC. Examples
        /// of valid format: 1st,3rd monday of month 15:30, every wed,fri of jan,
        /// jun 13:15, and first sunday of quarter 00:00. See more explanation
        /// about the format here:
        /// https://cloud.google.com/appengine/docs/flexible/python/scheduling-jobs-with-cron-yaml#the_schedule_format
        /// NOTE: The minimum interval time between recurring transfers depends
        /// on the data source; refer to the documentation for your data source.
        #[builder(into, default)]
        pub schedule: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Options customizing the data transfer schedule.
        /// Structure is documented below.
        #[builder(into, default)]
        pub schedule_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::DataTransferConfigScheduleOptions>,
        >,
        /// Different parameters are configured primarily using the the `params` field on this
        /// resource. This block contains the parameters which contain secrets or passwords so that they can be marked
        /// sensitive and hidden from plan output. The name of the field, eg: secret_access_key, will be the key
        /// in the `params` map in the api request.
        /// Credentials may not be specified in both locations and will cause an error. Changing from one location
        /// to a different credential configuration in the config will require an apply to update state.
        /// Structure is documented below.
        #[builder(into, default)]
        pub sensitive_params: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::DataTransferConfigSensitiveParams>,
        >,
        /// Service account email. If this field is set, transfer config will
        /// be created with this service account credentials. It requires that
        /// requesting user calling this API has permissions to act as this service account.
        #[builder(into, default)]
        pub service_account_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DataTransferConfigResult {
        /// The number of days to look back to automatically refresh the data.
        /// For example, if dataRefreshWindowDays = 10, then every day BigQuery
        /// reingests data for [today-10, today-1], rather than ingesting data for
        /// just [today-1]. Only valid if the data source supports the feature.
        /// Set the value to 0 to use the default value.
        pub data_refresh_window_days: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The data source id. Cannot be changed once the transfer config is created.
        pub data_source_id: pulumi_gestalt_rust::Output<String>,
        /// The BigQuery target dataset id.
        pub destination_dataset_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// When set to true, no runs are scheduled for a given transfer.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The user specified display name for the transfer config.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Email notifications will be sent according to these preferences to the
        /// email address of the user who owns this transfer config.
        /// Structure is documented below.
        pub email_preferences: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::DataTransferConfigEmailPreferences>,
        >,
        /// Represents the encryption configuration for a transfer.
        /// Structure is documented below.
        pub encryption_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::bigquery::DataTransferConfigEncryptionConfiguration,
            >,
        >,
        /// The geographic location where the transfer config should reside.
        /// Examples: US, EU, asia-northeast1. The default value is US.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource name of the transfer config. Transfer config names have the
        /// form projects/{projectId}/locations/{location}/transferConfigs/{configId}
        /// or projects/{projectId}/transferConfigs/{configId},
        /// where configId is usually a uuid, but this is not required.
        /// The name is ignored when creating a transfer config.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Pub/Sub topic where notifications will be sent after transfer runs
        /// associated with this transfer config finish.
        pub notification_pubsub_topic: pulumi_gestalt_rust::Output<Option<String>>,
        /// Parameters specific to each data source. For more information see the bq tab in the 'Setting up a data transfer'
        /// section for each data source. For example the parameters for Cloud Storage transfers are listed here:
        /// https://cloud.google.com/bigquery-transfer/docs/cloud-storage-transfer#bq
        /// **NOTE** : If you are attempting to update a parameter that cannot be updated (due to api limitations) please force recreation of the resource.
        ///
        ///
        /// - - -
        pub params: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Data transfer schedule. If the data source does not support a custom
        /// schedule, this should be empty. If it is empty, the default value for
        /// the data source will be used. The specified times are in UTC. Examples
        /// of valid format: 1st,3rd monday of month 15:30, every wed,fri of jan,
        /// jun 13:15, and first sunday of quarter 00:00. See more explanation
        /// about the format here:
        /// https://cloud.google.com/appengine/docs/flexible/python/scheduling-jobs-with-cron-yaml#the_schedule_format
        /// NOTE: The minimum interval time between recurring transfers depends
        /// on the data source; refer to the documentation for your data source.
        pub schedule: pulumi_gestalt_rust::Output<Option<String>>,
        /// Options customizing the data transfer schedule.
        /// Structure is documented below.
        pub schedule_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::DataTransferConfigScheduleOptions>,
        >,
        /// Different parameters are configured primarily using the the `params` field on this
        /// resource. This block contains the parameters which contain secrets or passwords so that they can be marked
        /// sensitive and hidden from plan output. The name of the field, eg: secret_access_key, will be the key
        /// in the `params` map in the api request.
        /// Credentials may not be specified in both locations and will cause an error. Changing from one location
        /// to a different credential configuration in the config will require an apply to update state.
        /// Structure is documented below.
        pub sensitive_params: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::DataTransferConfigSensitiveParams>,
        >,
        /// Service account email. If this field is set, transfer config will
        /// be created with this service account credentials. It requires that
        /// requesting user calling this API has permissions to act as this service account.
        pub service_account_name: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataTransferConfigArgs,
    ) -> DataTransferConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_refresh_window_days_binding = args
            .data_refresh_window_days
            .get_output(context);
        let data_source_id_binding = args.data_source_id.get_output(context);
        let destination_dataset_id_binding = args
            .destination_dataset_id
            .get_output(context);
        let disabled_binding = args.disabled.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let email_preferences_binding = args.email_preferences.get_output(context);
        let encryption_configuration_binding = args
            .encryption_configuration
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let notification_pubsub_topic_binding = args
            .notification_pubsub_topic
            .get_output(context);
        let params_binding = args.params.get_output(context);
        let project_binding = args.project.get_output(context);
        let schedule_binding = args.schedule.get_output(context);
        let schedule_options_binding = args.schedule_options.get_output(context);
        let sensitive_params_binding = args.sensitive_params.get_output(context);
        let service_account_name_binding = args.service_account_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:bigquery/dataTransferConfig:DataTransferConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataRefreshWindowDays".into(),
                    value: &data_refresh_window_days_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataSourceId".into(),
                    value: &data_source_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationDatasetId".into(),
                    value: &destination_dataset_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailPreferences".into(),
                    value: &email_preferences_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionConfiguration".into(),
                    value: &encryption_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationPubsubTopic".into(),
                    value: &notification_pubsub_topic_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "params".into(),
                    value: &params_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedule".into(),
                    value: &schedule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduleOptions".into(),
                    value: &schedule_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sensitiveParams".into(),
                    value: &sensitive_params_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAccountName".into(),
                    value: &service_account_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataTransferConfigResult {
            data_refresh_window_days: o.get_field("dataRefreshWindowDays"),
            data_source_id: o.get_field("dataSourceId"),
            destination_dataset_id: o.get_field("destinationDatasetId"),
            disabled: o.get_field("disabled"),
            display_name: o.get_field("displayName"),
            email_preferences: o.get_field("emailPreferences"),
            encryption_configuration: o.get_field("encryptionConfiguration"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            notification_pubsub_topic: o.get_field("notificationPubsubTopic"),
            params: o.get_field("params"),
            project: o.get_field("project"),
            schedule: o.get_field("schedule"),
            schedule_options: o.get_field("scheduleOptions"),
            sensitive_params: o.get_field("sensitiveParams"),
            service_account_name: o.get_field("serviceAccountName"),
        }
    }
}
