#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebResourceSite {
    /// The site identifier. If the type is set to SITE, the identifier is a URL. If the type is
    /// set to INET_DOMAIN, the identifier is a domain name.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "identifier")]
    pub r#identifier: Box<String>,
    /// The type of resource to be verified.
    /// Possible values are: `INET_DOMAIN`, `SITE`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
