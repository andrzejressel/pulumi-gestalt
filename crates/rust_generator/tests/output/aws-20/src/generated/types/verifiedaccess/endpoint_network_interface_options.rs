#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointNetworkInterfaceOptions {
    #[builder(into, default)]
    #[serde(rename = "networkInterfaceId")]
    pub r#network_interface_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
}
