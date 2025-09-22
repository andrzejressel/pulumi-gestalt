#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReportPlanReportSetting {
    /// Specifies the list of accounts a report covers.
    #[builder(into)]
    #[serde(rename = "accounts")]
    pub r#accounts: Option<Vec<String>>,
    /// Specifies the Amazon Resource Names (ARNs) of the frameworks a report covers.
    #[builder(into)]
    #[serde(rename = "frameworkArns")]
    pub r#framework_arns: Option<Vec<String>>,
    /// Specifies the number of frameworks a report covers.
    #[builder(into)]
    #[serde(rename = "numberOfFrameworks")]
    pub r#number_of_frameworks: Option<i32>,
    /// Specifies the list of Organizational Units a report covers.
    #[builder(into)]
    #[serde(rename = "organizationUnits")]
    pub r#organization_units: Option<Vec<String>>,
    /// Specifies the list of regions a report covers.
    #[builder(into)]
    #[serde(rename = "regions")]
    pub r#regions: Option<Vec<String>>,
    /// Identifies the report template for the report. Reports are built using a report template. The report templates are: `RESOURCE_COMPLIANCE_REPORT` | `CONTROL_COMPLIANCE_REPORT` | `BACKUP_JOB_REPORT` | `COPY_JOB_REPORT` | `RESTORE_JOB_REPORT`.
    #[builder(into)]
    #[serde(rename = "reportTemplate")]
    pub r#report_template: String,
}
