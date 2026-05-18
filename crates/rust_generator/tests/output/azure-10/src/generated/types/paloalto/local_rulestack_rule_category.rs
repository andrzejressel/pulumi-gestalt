#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LocalRulestackRuleCategory {
    /// Specifies a list of URL categories to match. Possible values include `abortion`, `abused-drugs`, `adult`, `alcohol-and-tobacco`, `auctions`, `business-and-economy`, `command-and-control`, `computer-and-internet-info`, `content-delivery-networks`, `copyright-infringement`, `cryptocurrency`, `dating`, `dynamic-dns`, `educational-institutions`, `entertainment-and-arts`, `extremism`, `financial-services`, `gambling`, `games`, `government`, `grayware`, `hacking`, `health-and-medicine`, `high-risk`, `home-and-garden`, `hunting-and-fishing`, `insufficient-content`, `internet-communications-and-telephony`, `internet-portals`, `job-search`, `legal`, `low-risk`, `malware`, `medium-risk`, `military`, `motor-vehicles`, `music`, `newly-registered-domain`, `news`, `not-resolved`, `nudity`, `online-storage-and-backup`, `parked`, `peer-to-peer`, `personal-sites-and-blogs`, `philosophy-and-political-advocacy`, `phishing`, `private-ip-addresses`, `proxy-avoidance-and-anonymizers`, `questionable`, `real-estate`, `real-time-detection`, `recreation-and-hobbies`, `reference-and-research`, `religion`, `search-engines`, `sex-education`, `shareware-and-freeware`, `shopping`, `social-networking`, `society`, `sports`, `stock-advice-and-tools`, `streaming-media`, `swimsuits-and-intimate-apparel`, `training-and-tools`, `translation`, `travel`, `unknown`, `weapons`, `web-advertisements`, `web-based-email`, and `web-hosting`.
    #[builder(into)]
    #[serde(rename = "customUrls")]
    pub r#custom_urls: Vec<String>,
    /// Specifies a list of feeds to match.
    #[builder(into)]
    #[serde(rename = "feeds")]
    pub r#feeds: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LocalRulestackRuleCategory {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "custom_urls",
                    &self.r#custom_urls,
                ),
                to_pulumi_object_field(
                    "feeds",
                    &self.r#feeds,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LocalRulestackRuleCategory {
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
                    r#custom_urls: {
                        let field_value = match fields_map.get("custom_urls") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_urls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#feeds: {
                        let field_value = match fields_map.get("feeds") {
                            Some(value) => value,
                            None => bail!("Missing field 'feeds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
