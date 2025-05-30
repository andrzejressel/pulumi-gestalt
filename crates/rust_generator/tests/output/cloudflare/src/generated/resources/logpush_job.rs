/// ## Example Usage
///
/// ## Import
///
/// Import an account-scoped job.
///
/// ```sh
/// $ pulumi import cloudflare:index/logpushJob:LogpushJob example account/<account_id>/<job_id>
/// ```
///
/// Import a zone-scoped job.
///
/// ```sh
/// $ pulumi import cloudflare:index/logpushJob:LogpushJob example zone/<zone_id>/<job_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod logpush_job {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogpushJobArgs {
        /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The kind of the dataset to use with the logpush job. Available values: `access_requests`, `casb_findings`, `firewall_events`, `http_requests`, `spectrum_events`, `nel_reports`, `audit_logs`, `gateway_dns`, `gateway_http`, `gateway_network`, `dns_logs`, `network_analytics_logs`, `workers_trace_events`, `device_posture_results`, `zero_trust_network_sessions`, `magic_ids_detections`, `page_shield_events`.
        #[builder(into)]
        pub dataset: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Uniquely identifies a resource (such as an s3 bucket) where data will be pushed. Additional configuration parameters supported by the destination may be included. See [Logpush destination documentation](https://developers.cloudflare.com/logs/reference/logpush-api-configuration#destination).
        #[builder(into)]
        pub destination_conf: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to enable the job.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Use filters to select the events to include and/or remove from your logs. For more information, refer to [Filters](https://developers.cloudflare.com/logs/reference/logpush-api-configuration/filters/).
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A higher frequency will result in logs being pushed on faster with smaller files. `low` frequency will push logs less often with larger files. Available values: `high`, `low`. Defaults to `high`.
        #[builder(into, default)]
        pub frequency: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The kind of logpush job to create. Available values: `edge`, `instant-logs`, `""`.
        #[builder(into, default)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration string for the Logshare API. It specifies things like requested fields and timestamp formats. See [Logpush options documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#options).
        #[builder(into, default)]
        pub logpull_options: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum uncompressed file size of a batch of logs. Value must be between 5MB and 1GB.
        #[builder(into, default)]
        pub max_upload_bytes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The maximum interval in seconds for log batches. Value must be between 30 and 300.
        #[builder(into, default)]
        pub max_upload_interval_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The maximum number of log lines per batch. Value must be between 1000 and 1,000,000.
        #[builder(into, default)]
        pub max_upload_records: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the logpush job to create.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Structured replacement for logpull*options. When including this field, the logpull*option field will be ignored.
        #[builder(into, default)]
        pub output_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::LogpushJobOutputOptions>,
        >,
        /// Ownership challenge token to prove destination ownership, required when destination is Amazon S3, Google Cloud Storage, Microsoft Azure or Sumo Logic. See [Developer documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#usage).
        #[builder(into, default)]
        pub ownership_challenge: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LogpushJobResult {
        /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The kind of the dataset to use with the logpush job. Available values: `access_requests`, `casb_findings`, `firewall_events`, `http_requests`, `spectrum_events`, `nel_reports`, `audit_logs`, `gateway_dns`, `gateway_http`, `gateway_network`, `dns_logs`, `network_analytics_logs`, `workers_trace_events`, `device_posture_results`, `zero_trust_network_sessions`, `magic_ids_detections`, `page_shield_events`.
        pub dataset: pulumi_gestalt_rust::Output<String>,
        /// Uniquely identifies a resource (such as an s3 bucket) where data will be pushed. Additional configuration parameters supported by the destination may be included. See [Logpush destination documentation](https://developers.cloudflare.com/logs/reference/logpush-api-configuration#destination).
        pub destination_conf: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable the job.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Use filters to select the events to include and/or remove from your logs. For more information, refer to [Filters](https://developers.cloudflare.com/logs/reference/logpush-api-configuration/filters/).
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// A higher frequency will result in logs being pushed on faster with smaller files. `low` frequency will push logs less often with larger files. Available values: `high`, `low`. Defaults to `high`.
        pub frequency: pulumi_gestalt_rust::Output<Option<String>>,
        /// The kind of logpush job to create. Available values: `edge`, `instant-logs`, `""`.
        pub kind: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration string for the Logshare API. It specifies things like requested fields and timestamp formats. See [Logpush options documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#options).
        pub logpull_options: pulumi_gestalt_rust::Output<Option<String>>,
        /// The maximum uncompressed file size of a batch of logs. Value must be between 5MB and 1GB.
        pub max_upload_bytes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The maximum interval in seconds for log batches. Value must be between 30 and 300.
        pub max_upload_interval_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The maximum number of log lines per batch. Value must be between 1000 and 1,000,000.
        pub max_upload_records: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the logpush job to create.
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Structured replacement for logpull*options. When including this field, the logpull*option field will be ignored.
        pub output_options: pulumi_gestalt_rust::Output<
            Option<super::types::LogpushJobOutputOptions>,
        >,
        /// Ownership challenge token to prove destination ownership, required when destination is Amazon S3, Google Cloud Storage, Microsoft Azure or Sumo Logic. See [Developer documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#usage).
        pub ownership_challenge: pulumi_gestalt_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
        pub zone_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogpushJobArgs,
    ) -> LogpushJobResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let dataset_binding = args.dataset.get_output(context);
        let destination_conf_binding = args.destination_conf.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let frequency_binding = args.frequency.get_output(context);
        let kind_binding = args.kind.get_output(context);
        let logpull_options_binding = args.logpull_options.get_output(context);
        let max_upload_bytes_binding = args.max_upload_bytes.get_output(context);
        let max_upload_interval_seconds_binding = args
            .max_upload_interval_seconds
            .get_output(context);
        let max_upload_records_binding = args.max_upload_records.get_output(context);
        let name_binding = args.name.get_output(context);
        let output_options_binding = args.output_options.get_output(context);
        let ownership_challenge_binding = args.ownership_challenge.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/logpushJob:LogpushJob".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataset".into(),
                    value: &dataset_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationConf".into(),
                    value: &destination_conf_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frequency".into(),
                    value: &frequency_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kind".into(),
                    value: &kind_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logpullOptions".into(),
                    value: &logpull_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxUploadBytes".into(),
                    value: &max_upload_bytes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxUploadIntervalSeconds".into(),
                    value: &max_upload_interval_seconds_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxUploadRecords".into(),
                    value: &max_upload_records_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outputOptions".into(),
                    value: &output_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ownershipChallenge".into(),
                    value: &ownership_challenge_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LogpushJobResult {
            account_id: o.get_field("accountId"),
            dataset: o.get_field("dataset"),
            destination_conf: o.get_field("destinationConf"),
            enabled: o.get_field("enabled"),
            filter: o.get_field("filter"),
            frequency: o.get_field("frequency"),
            kind: o.get_field("kind"),
            logpull_options: o.get_field("logpullOptions"),
            max_upload_bytes: o.get_field("maxUploadBytes"),
            max_upload_interval_seconds: o.get_field("maxUploadIntervalSeconds"),
            max_upload_records: o.get_field("maxUploadRecords"),
            name: o.get_field("name"),
            output_options: o.get_field("outputOptions"),
            ownership_challenge: o.get_field("ownershipChallenge"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
