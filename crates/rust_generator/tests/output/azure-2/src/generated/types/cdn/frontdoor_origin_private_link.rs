#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FrontdoorOriginPrivateLink {
    /// Specifies the location where the Private Link resource should exist. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// The ID of the Azure Resource to connect to via the Private Link.
    /// 
    /// > **Note:** the `private_link_target_id` property must specify the Resource ID of the Private Link Service when using Load Balancer as an Origin.
    #[builder(into)]
    #[serde(rename = "privateLinkTargetId")]
    pub r#private_link_target_id: String,
    /// Specifies the request message that will be submitted to the `private_link_target_id` when requesting the private link endpoint connection. Values must be between `1` and `140` characters in length. Defaults to `Access request for CDN FrontDoor Private Link Origin`.
    #[builder(into)]
    #[serde(rename = "requestMessage")]
    pub r#request_message: Option<String>,
    /// Specifies the type of target for this Private Link Endpoint. Possible values are `blob`, `blob_secondary`, `web` and `sites`.
    /// 
    /// > **NOTE:** `target_type` cannot be specified when using a Load Balancer as an Origin.
    #[builder(into)]
    #[serde(rename = "targetType")]
    pub r#target_type: Option<String>,
}
