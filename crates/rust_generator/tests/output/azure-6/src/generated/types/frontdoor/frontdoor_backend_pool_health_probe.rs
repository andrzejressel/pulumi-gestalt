#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorBackendPoolHealthProbe {
    /// Is this health probe enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The ID of the FrontDoor.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The number of seconds between each Health Probe. Defaults to `120`.
    #[builder(into)]
    #[serde(rename = "intervalInSeconds")]
    pub r#interval_in_seconds: Option<i32>,
    /// Specifies the name of the Health Probe.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The path to use for the Health Probe. Default is `/`.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Specifies HTTP method the health probe uses when querying the backend pool instances. Possible values include: `GET` and `HEAD`. Defaults to `GET`.
    /// 
    /// > **NOTE:** Use the `HEAD` method if you do not need to check the response body of your health probe.
    #[builder(into)]
    #[serde(rename = "probeMethod")]
    pub r#probe_method: Option<String>,
    /// Protocol scheme to use for the Health Probe. Possible values are `Http` and `Https`. Defaults to `Http`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
}
