#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketWebsite {
    /// An absolute path to the document to return in case of a 4XX error.
    #[builder(into)]
    #[serde(rename = "errorDocument")]
    pub r#error_document: Option<String>,
    /// Amazon S3 returns this index document when requests are made to the root domain or any of the subfolders.
    #[builder(into)]
    #[serde(rename = "indexDocument")]
    pub r#index_document: Option<String>,
    /// A hostname to redirect all website requests for this bucket to. Hostname can optionally be prefixed with a protocol (`http://` or `https://`) to use when redirecting requests. The default is the protocol that is used in the original request.
    #[builder(into)]
    #[serde(rename = "redirectAllRequestsTo")]
    pub r#redirect_all_requests_to: Option<String>,
    /// A json array containing [routing rules](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules.html)
    /// describing redirect behavior and when redirects are applied.
    /// 
    /// The `CORS` object supports the following:
    #[builder(into)]
    #[serde(rename = "routingRules")]
    pub r#routing_rules: Option<String>,
}
