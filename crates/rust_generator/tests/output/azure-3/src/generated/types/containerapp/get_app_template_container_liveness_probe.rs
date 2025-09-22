#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAppTemplateContainerLivenessProbe {
    /// The number of consecutive failures required to consider this probe as failed. Possible values are between `1` and `30`. Defaults to `3`.
    #[builder(into)]
    #[serde(rename = "failureCountThreshold")]
    pub r#failure_count_threshold: i32,
    /// A `header` block as detailed below.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Vec<super::super::types::containerapp::GetAppTemplateContainerLivenessProbeHeader>,
    /// The value for the host header which should be sent with this probe. If unspecified, the IP Address of the Pod is used as the host header. Setting a value for `Host` in `headers` can be used to override this for `HTTP` and `HTTPS` type probes.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: String,
    /// The number of seconds elapsed after the container has started before the probe is initiated. Possible values are between `0` and `60`. Defaults to `0` seconds.
    #[builder(into)]
    #[serde(rename = "initialDelay")]
    pub r#initial_delay: i32,
    /// How often, in seconds, the probe should run. Possible values are between `1` and `240`. Defaults to `10`
    #[builder(into)]
    #[serde(rename = "intervalSeconds")]
    pub r#interval_seconds: i32,
    /// The path in the container at which to mount this volume.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// The port number on which to connect. Possible values are between `1` and `65535`.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    /// The time in seconds after the container is sent the termination signal before the process if forcibly killed.
    #[builder(into)]
    #[serde(rename = "terminationGracePeriodSeconds")]
    pub r#termination_grace_period_seconds: i32,
    /// Time in seconds after which the probe times out. Possible values are in the range `1` - `240`. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: i32,
    /// The transport method for the Ingress. Possible values include `auto`, `http`, and `http2`. Defaults to `auto`
    #[builder(into)]
    #[serde(rename = "transport")]
    pub r#transport: String,
}
