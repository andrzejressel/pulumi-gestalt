#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTemplate {
    /// Optional metadata for this Revision, including labels and annotations.
    /// Name will be generated by the Configuration. To set minimum instances
    /// for this revision, use the "autoscaling.knative.dev/minScale" annotation
    /// key. To set maximum instances for this revision, use the
    /// "autoscaling.knative.dev/maxScale" annotation key. To set Cloud SQL
    /// connections for the revision, use the "run.googleapis.com/cloudsql-instances"
    /// annotation key.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<Option<super::super::types::cloudrun::ServiceTemplateMetadata>>,
    /// RevisionSpec holds the desired state of the Revision (from the client).
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "spec")]
    pub r#spec: Box<Option<super::super::types::cloudrun::ServiceTemplateSpec>>,
}
