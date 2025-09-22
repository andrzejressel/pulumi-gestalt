#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceConfigurationWebCrawlerConfigurationUrls {
    /// A block that specifies the configuration of the seed or starting point URLs of the websites you want to crawl. You can choose to crawl only the website host names, or the website host names with subdomains, or the website host names with subdomains and other domains that the webpages link to. You can list up to `100` seed URLs. Detailed below.
    #[builder(into)]
    #[serde(rename = "seedUrlConfiguration")]
    pub r#seed_url_configuration: Option<Box<super::super::types::kendra::DataSourceConfigurationWebCrawlerConfigurationUrlsSeedUrlConfiguration>>,
    /// A block that specifies the configuration of the sitemap URLs of the websites you want to crawl. Only URLs belonging to the same website host names are crawled. You can list up to `3` sitemap URLs. Detailed below.
    #[builder(into)]
    #[serde(rename = "siteMapsConfiguration")]
    pub r#site_maps_configuration: Option<Box<super::super::types::kendra::DataSourceConfigurationWebCrawlerConfigurationUrlsSiteMapsConfiguration>>,
}
