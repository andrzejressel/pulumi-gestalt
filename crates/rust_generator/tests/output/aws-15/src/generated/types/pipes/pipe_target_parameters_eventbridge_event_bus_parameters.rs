#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipeTargetParametersEventbridgeEventBusParameters {
    /// A free-form string, with a maximum of 128 characters, used to decide what fields to expect in the event detail.
    #[builder(into)]
    #[serde(rename = "detailType")]
    pub r#detail_type: Option<String>,
    /// The URL subdomain of the endpoint. For example, if the URL for Endpoint is https://abcde.veo.endpoints.event.amazonaws.com, then the EndpointId is abcde.veo.
    #[builder(into)]
    #[serde(rename = "endpointId")]
    pub r#endpoint_id: Option<String>,
    /// List of AWS resources, identified by Amazon Resource Name (ARN), which the event primarily concerns. Any number, including zero, may be present.
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Option<Vec<String>>,
    /// Source resource of the pipe. This field typically requires an ARN (Amazon Resource Name). However, when using a self-managed Kafka cluster, you should use a different format. Instead of an ARN, use 'smk://' followed by the bootstrap server's address.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Option<String>,
    /// The time stamp of the event, per RFC3339. If no time stamp is provided, the time stamp of the PutEvents call is used. This is the JSON path to the field in the event e.g. $.detail.timestamp
    #[builder(into)]
    #[serde(rename = "time")]
    pub r#time: Option<String>,
}
