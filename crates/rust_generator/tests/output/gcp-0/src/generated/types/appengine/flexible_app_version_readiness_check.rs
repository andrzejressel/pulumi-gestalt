#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleAppVersionReadinessCheck {
    /// A maximum time limit on application initialization, measured from moment the application successfully
    /// replies to a healthcheck until it is ready to serve traffic. Default: "300s"
    #[builder(into)]
    #[serde(rename = "appStartTimeout")]
    pub r#app_start_timeout: Option<String>,
    /// Interval between health checks.  Default: "5s".
    #[builder(into)]
    #[serde(rename = "checkInterval")]
    pub r#check_interval: Option<String>,
    /// Number of consecutive failed checks required before removing traffic. Default: 2.
    #[builder(into)]
    #[serde(rename = "failureThreshold")]
    pub r#failure_threshold: Option<f64>,
    /// Host header to send when performing a HTTP Readiness check. Example: "myapp.appspot.com"
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Option<String>,
    /// The request path.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// Number of consecutive successful checks required before receiving traffic. Default: 2.
    #[builder(into)]
    #[serde(rename = "successThreshold")]
    pub r#success_threshold: Option<f64>,
    /// Time before the check is considered failed. Default: "4s"
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
}
