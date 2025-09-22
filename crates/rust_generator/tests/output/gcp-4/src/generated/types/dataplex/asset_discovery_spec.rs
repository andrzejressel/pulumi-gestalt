#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AssetDiscoverySpec {
    /// Optional. Configuration for CSV data.
    #[builder(into)]
    #[serde(rename = "csvOptions")]
    pub r#csv_options: Box<Option<super::super::types::dataplex::AssetDiscoverySpecCsvOptions>>,
    /// Required. Whether discovery is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Optional. The list of patterns to apply for selecting data to exclude during discovery. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names.
    #[builder(into)]
    #[serde(rename = "excludePatterns")]
    pub r#exclude_patterns: Option<Vec<String>>,
    /// Optional. The list of patterns to apply for selecting data to include during discovery if only a subset of the data should considered. For Cloud Storage bucket assets, these are interpreted as glob patterns used to match object names. For BigQuery dataset assets, these are interpreted as patterns to match table names.
    #[builder(into)]
    #[serde(rename = "includePatterns")]
    pub r#include_patterns: Option<Vec<String>>,
    /// Optional. Configuration for Json data.
    #[builder(into)]
    #[serde(rename = "jsonOptions")]
    pub r#json_options: Box<Option<super::super::types::dataplex::AssetDiscoverySpecJsonOptions>>,
    /// Optional. Cron schedule (https://en.wikipedia.org/wiki/Cron) for running discovery periodically. Successive discovery runs must be scheduled at least 60 minutes apart. The default value is to run discovery every 60 minutes. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: "CRON_TZ=${IANA_TIME_ZONE}" or TZ=${IANA_TIME_ZONE}". The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, "CRON_TZ=America/New_York 1 * * * *", or "TZ=America/New_York 1 * * * *".
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Option<String>,
}
