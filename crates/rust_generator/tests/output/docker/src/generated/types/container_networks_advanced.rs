#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContainerNetworksAdvanced {
    /// The network aliases of the container in the specific network.
    #[builder(into)]
    #[serde(rename = "aliases")]
    pub r#aliases: Option<Vec<String>>,
    /// The IPV4 address of the container in the specific network.
    #[builder(into)]
    #[serde(rename = "ipv4Address")]
    pub r#ipv_4_address: Option<String>,
    /// The IPV6 address of the container in the specific network.
    #[builder(into)]
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: Option<String>,
    /// The name or id of the network to use. You can use `name` or `id` attribute from a `docker.Network` resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
