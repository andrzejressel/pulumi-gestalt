#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IoTHubFallbackRoute {
    /// The condition that is evaluated to apply the routing rule. Defaults to `true`. For grammar, see: <https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-query-language>.
    #[builder(into)]
    #[serde(rename = "condition")]
    pub r#condition: Option<String>,
    /// Used to specify whether the fallback route is enabled. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The endpoints to which messages that satisfy the condition are routed. Currently only 1 endpoint is allowed.
    #[builder(into)]
    #[serde(rename = "endpointNames")]
    pub r#endpoint_names: Option<Vec<String>>,
    /// The source that the routing rule is to be applied to, such as `DeviceMessages`. Possible values include: `Invalid`, `DeviceMessages`, `TwinChangeEvents`, `DeviceLifecycleEvents`, `DeviceConnectionStateEvents`, `DeviceJobLifecycleEvents` and `DigitalTwinChangeEvents`. Defaults to `DeviceMessages`.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Option<String>,
}
