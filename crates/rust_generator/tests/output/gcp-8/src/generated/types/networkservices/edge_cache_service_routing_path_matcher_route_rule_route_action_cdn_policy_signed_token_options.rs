#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicySignedTokenOptions {
    /// The allowed signature algorithms to use.
    /// Defaults to using only ED25519.
    /// You may specify up to 3 signature algorithms to use.
    /// Each value may be one of: `ED25519`, `HMAC_SHA_256`, `HMAC_SHA1`.
    #[builder(into)]
    #[serde(rename = "allowedSignatureAlgorithms")]
    pub r#allowed_signature_algorithms: Option<Vec<String>>,
    /// The query parameter in which to find the token.
    /// The name must be 1-64 characters long and match the regular expression `a-zA-Z*` which means the first character must be a letter, and all following characters must be a dash, underscore, letter or digit.
    /// Defaults to `edge-cache-token`.
    #[builder(into)]
    #[serde(rename = "tokenQueryParameter")]
    pub r#token_query_parameter: Option<String>,
}
