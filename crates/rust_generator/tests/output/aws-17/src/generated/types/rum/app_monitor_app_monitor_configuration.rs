#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AppMonitorAppMonitorConfiguration {
    /// If you set this to `true`, RUM web client sets two cookies, a session cookie  and a user cookie. The cookies allow the RUM web client to collect data relating to the number of users an application has and the behavior of the application across a sequence of events. Cookies are stored in the top-level domain of the current page.
    #[builder(into)]
    #[serde(rename = "allowCookies")]
    pub r#allow_cookies: Option<bool>,
    /// If you set this to `true`, RUM enables X-Ray tracing for the user sessions  that RUM samples. RUM adds an X-Ray trace header to allowed HTTP requests. It also records an X-Ray segment for allowed HTTP requests.
    #[builder(into)]
    #[serde(rename = "enableXray")]
    pub r#enable_xray: Option<bool>,
    /// A list of URLs in your website or application to exclude from RUM data collection.
    #[builder(into)]
    #[serde(rename = "excludedPages")]
    pub r#excluded_pages: Option<Vec<String>>,
    /// A list of pages in the CloudWatch RUM console that are to be displayed with a "favorite" icon.
    #[builder(into)]
    #[serde(rename = "favoritePages")]
    pub r#favorite_pages: Option<Vec<String>>,
    /// The ARN of the guest IAM role that is attached to the Amazon Cognito identity pool that is used to authorize the sending of data to RUM.
    #[builder(into)]
    #[serde(rename = "guestRoleArn")]
    pub r#guest_role_arn: Option<String>,
    /// The ID of the Amazon Cognito identity pool that is used to authorize the sending of data to RUM.
    #[builder(into)]
    #[serde(rename = "identityPoolId")]
    pub r#identity_pool_id: Option<String>,
    /// If this app monitor is to collect data from only certain pages in your application, this structure lists those pages.
    #[builder(into)]
    #[serde(rename = "includedPages")]
    pub r#included_pages: Option<Vec<String>>,
    /// Specifies the percentage of user sessions to use for RUM data collection. Choosing a higher percentage gives you more data but also incurs more costs. The number you specify is the percentage of user sessions that will be used. Default value is `0.1`.
    #[builder(into)]
    #[serde(rename = "sessionSampleRate")]
    pub r#session_sample_rate: Option<f64>,
    /// An array that lists the types of telemetry data that this app monitor is to collect. Valid values are `errors`, `performance`, and `http`.
    #[builder(into)]
    #[serde(rename = "telemetries")]
    pub r#telemetries: Option<Vec<String>>,
}
