#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LogpushJobOutputOptions {
    /// String to be prepended before each batch.
    #[builder(into)]
    #[serde(rename = "batchPrefix")]
    pub r#batch_prefix: Option<String>,
    /// String to be appended after each batch.
    #[builder(into)]
    #[serde(rename = "batchSuffix")]
    pub r#batch_suffix: Option<String>,
    /// Mitigation for CVE-2021-44228. If set to true, will cause all occurrences of ${ in the generated files to be replaced with x{. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "cve20214428")]
    pub r#cve_20214428: Option<bool>,
    /// String to join fields. This field be ignored when record_template is set. Defaults to `,`.
    #[builder(into)]
    #[serde(rename = "fieldDelimiter")]
    pub r#field_delimiter: Option<String>,
    /// List of field names to be included in the Logpush output.
    #[builder(into)]
    #[serde(rename = "fieldNames")]
    pub r#field_names: Option<Vec<String>>,
    /// Specifies the output type. Available values: `ndjson`, `csv`. Defaults to `ndjson`.
    #[builder(into)]
    #[serde(rename = "outputType")]
    pub r#output_type: Option<String>,
    /// String to be inserted in-between the records as separator.
    #[builder(into)]
    #[serde(rename = "recordDelimiter")]
    pub r#record_delimiter: Option<String>,
    /// String to be prepended before each record. Defaults to `{`.
    #[builder(into)]
    #[serde(rename = "recordPrefix")]
    pub r#record_prefix: Option<String>,
    /// String to be appended after each record. Defaults to `}
    /// `.
    #[builder(into)]
    #[serde(rename = "recordSuffix")]
    pub r#record_suffix: Option<String>,
    /// String to use as template for each record instead of the default comma-separated list.
    #[builder(into)]
    #[serde(rename = "recordTemplate")]
    pub r#record_template: Option<String>,
    /// Specifies the sampling rate. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Option<f64>,
    /// Specifies the format for timestamps. Available values: `unixnano`, `unix`, `rfc3339`. Defaults to `unixnano`.
    #[builder(into)]
    #[serde(rename = "timestampFormat")]
    pub r#timestamp_format: Option<String>,
}
