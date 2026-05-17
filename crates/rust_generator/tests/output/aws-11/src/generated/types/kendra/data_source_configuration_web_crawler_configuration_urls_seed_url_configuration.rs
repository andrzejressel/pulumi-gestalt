#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceConfigurationWebCrawlerConfigurationUrlsSeedUrlConfiguration {
    /// The list of seed or starting point URLs of the websites you want to crawl. The list can include a maximum of `100` seed URLs. Array Members: Minimum number of `0` items. Maximum number of `100` items. Length Constraints: Minimum length of `1`. Maximum length of `2048`.
    #[builder(into)]
    #[serde(rename = "seedUrls")]
    pub r#seed_urls: Vec<String>,
    /// The default mode is set to `HOST_ONLY`. You can choose one of the following modes:
    /// * `HOST_ONLY` – crawl only the website host names. For example, if the seed URL is `"abc.example.com"`, then only URLs with host name `"abc.example.com"` are crawled.
    /// * `SUBDOMAINS` – crawl the website host names with subdomains. For example, if the seed URL is `"abc.example.com"`, then `"a.abc.example.com"` and `"b.abc.example.com"` are also crawled.
    /// * `EVERYTHING` – crawl the website host names with subdomains and other domains that the webpages link to.
    #[builder(into)]
    #[serde(rename = "webCrawlerMode")]
    pub r#web_crawler_mode: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSourceConfigurationWebCrawlerConfigurationUrlsSeedUrlConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "seed_urls".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#seed_urls,
                )
                .await,
            );
            map.insert(
                "web_crawler_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#web_crawler_mode,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSourceConfigurationWebCrawlerConfigurationUrlsSeedUrlConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#seed_urls: {
                        let field_value = match fields_map.get("seed_urls") {
                            Some(value) => value,
                            None => bail!("Missing field 'seed_urls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_crawler_mode: {
                        let field_value = match fields_map.get("web_crawler_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'web_crawler_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
