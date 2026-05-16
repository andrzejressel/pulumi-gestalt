#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSourceConfigurationWebCrawlerConfigurationUrls {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("seed_url_configuration".to_string(), self.r#seed_url_configuration.to_pulumi_value().await);
            map.insert("site_maps_configuration".to_string(), self.r#site_maps_configuration.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSourceConfigurationWebCrawlerConfigurationUrls {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#seed_url_configuration: {
                        let field_value = match fields_map.get("seed_url_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'seed_url_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kendra::DataSourceConfigurationWebCrawlerConfigurationUrlsSeedUrlConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#site_maps_configuration: {
                        let field_value = match fields_map.get("site_maps_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'site_maps_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kendra::DataSourceConfigurationWebCrawlerConfigurationUrlsSiteMapsConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
