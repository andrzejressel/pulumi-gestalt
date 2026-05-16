#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubscriptionPushConfigNoWrapper {
    /// When true, writes the Pub/Sub message metadata to
    /// `x-goog-pubsub-<KEY>:<VAL>` headers of the HTTP request. Writes the
    /// Pub/Sub message attributes to `<KEY>:<VAL>` headers of the HTTP request.
    #[builder(into)]
    #[serde(rename = "writeMetadata")]
    pub r#write_metadata: bool,
}
