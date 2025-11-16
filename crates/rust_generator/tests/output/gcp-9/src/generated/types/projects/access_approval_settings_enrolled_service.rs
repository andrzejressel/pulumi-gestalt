#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessApprovalSettingsEnrolledService {
    /// The product for which Access Approval will be enrolled. Allowed values are listed (case-sensitive):
    /// all
    /// appengine.googleapis.com
    /// bigquery.googleapis.com
    /// bigtable.googleapis.com
    /// cloudkms.googleapis.com
    /// compute.googleapis.com
    /// dataflow.googleapis.com
    /// iam.googleapis.com
    /// pubsub.googleapis.com
    /// storage.googleapis.com
    #[builder(into)]
    #[serde(rename = "cloudProduct")]
    pub r#cloud_product: String,
    /// The enrollment level of the service.
    /// Default value is `BLOCK_ALL`.
    /// Possible values are: `BLOCK_ALL`.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "enrollmentLevel")]
    pub r#enrollment_level: Option<String>,
}
