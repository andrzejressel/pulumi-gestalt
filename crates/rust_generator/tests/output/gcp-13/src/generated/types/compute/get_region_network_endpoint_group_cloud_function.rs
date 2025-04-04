#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRegionNetworkEndpointGroupCloudFunction {
    /// A user-defined name of the Cloud Function.
    /// The function name is case-sensitive and must be 1-63 characters long.
    /// Example value: "func1".
    #[builder(into)]
    #[serde(rename = "function")]
    pub r#function: Box<String>,
    /// A template to parse function field from a request URL. URL mask allows
    /// for routing to multiple Cloud Functions without having to create
    /// multiple Network Endpoint Groups and backend services.
    /// 
    /// For example, request URLs "mydomain.com/function1" and "mydomain.com/function2"
    /// can be backed by the same Serverless NEG with URL mask "/". The URL mask
    /// will parse them to { function = "function1" } and { function = "function2" } respectively.
    #[builder(into)]
    #[serde(rename = "urlMask")]
    pub r#url_mask: Box<String>,
}
