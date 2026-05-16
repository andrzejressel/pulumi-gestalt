#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionNetworkEndpointGroupPscData {
    /// The PSC producer port to use when consumer PSC NEG connects to a producer. If
    /// this flag isn't specified for a PSC NEG with endpoint type
    /// private-service-connect, then PSC NEG will be connected to a first port in the
    /// available PSC producer port range.
    #[builder(into)]
    #[serde(rename = "producerPort")]
    pub r#producer_port: Option<String>,
}
