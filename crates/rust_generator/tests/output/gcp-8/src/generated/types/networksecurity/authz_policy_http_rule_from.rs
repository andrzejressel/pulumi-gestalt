#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthzPolicyHttpRuleFrom {
    /// Describes the properties of a request's sources. At least one of sources or notSources must be specified. Limited to 5 sources. A match occurs when ANY source (in sources or notSources) matches the request. Within a single source, the match follows AND semantics across fields and OR semantics within a single field, i.e. a match occurs when ANY principal matches AND ANY ipBlocks match.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "notSources")]
    pub r#not_sources: Option<Vec<super::super::types::networksecurity::AuthzPolicyHttpRuleFromNotSource>>,
    /// Describes the properties of a request's sources. At least one of sources or notSources must be specified. Limited to 5 sources. A match occurs when ANY source (in sources or notSources) matches the request. Within a single source, the match follows AND semantics across fields and OR semantics within a single field, i.e. a match occurs when ANY principal matches AND ANY ipBlocks match.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sources")]
    pub r#sources: Option<Vec<super::super::types::networksecurity::AuthzPolicyHttpRuleFromSource>>,
}
