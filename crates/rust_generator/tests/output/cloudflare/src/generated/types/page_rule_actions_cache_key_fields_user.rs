#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PageRuleActionsCacheKeyFieldsUser {
    /// `true` - classifies a request as “mobile”, “desktop”, or “tablet” based on the User Agent; defaults to `false`.
    #[builder(into)]
    #[serde(rename = "deviceType")]
    pub r#device_type: Option<bool>,
    /// `true` - includes the client’s country, derived from the IP address; defaults to `false`.
    #[builder(into)]
    #[serde(rename = "geo")]
    pub r#geo: Option<bool>,
    /// `true` - includes the first language code contained in the `Accept-Language` header sent by the client; defaults to `false`.
    /// 
    /// Example:
    /// 
    /// ```ignore
    /// use pulumi_gestalt_rust::Output;
    /// use pulumi_gestalt_rust::{add_export, pulumi_main};
    /// #[pulumi_main]
    /// fn test_main() -> Result<(), Error> {
    ///     let foobar = page_rule::create(
    ///         "foobar",
    ///         PageRuleArgs::builder()
    ///             .actions(
    ///                 PageRuleActions::builder()
    ///                     .cacheKeyFields(
    ///                         PageRuleActionsCacheKeyFields::builder()
    ///                             .cookie(
    ///                                 PageRuleActionsCacheKeyFieldsCookie::builder()
    ///                                     .checkPresences(vec!["wordpress_test_cookie",])
    ///                                     .build_struct(),
    ///                             )
    ///                             .header(
    ///                                 PageRuleActionsCacheKeyFieldsHeader::builder()
    ///                                     .checkPresences(vec!["header_present",])
    ///                                     .excludes(vec!["origin",])
    ///                                     .includes(vec!["api-key", "dnt",])
    ///                                     .build_struct(),
    ///                             )
    ///                             .host(
    ///                                 PageRuleActionsCacheKeyFieldsHost::builder()
    ///                                     .resolved(true)
    ///                                     .build_struct(),
    ///                             )
    ///                             .queryString(
    ///                                 PageRuleActionsCacheKeyFieldsQueryString::builder()
    ///                                     .ignore(true)
    ///                                     .build_struct(),
    ///                             )
    ///                             .user(
    ///                                 PageRuleActionsCacheKeyFieldsUser::builder()
    ///                                     .deviceType(false)
    ///                                     .geo(true)
    ///                                     .lang(true)
    ///                                     .build_struct(),
    ///                             )
    ///                             .build_struct(),
    ///                     )
    ///                     .build_struct(),
    ///             )
    ///             .priority(1)
    ///             .target("${cloudflareZone}/app/*")
    ///             .zone_id("${cloudflareZoneId}")
    ///             .build_struct(),
    ///     );
    /// }
    /// ```
    #[builder(into)]
    #[serde(rename = "lang")]
    pub r#lang: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PageRuleActionsCacheKeyFieldsUser {
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
                    "device_type",
                    &self.r#device_type,
                ),
                to_pulumi_object_field(
                    "geo",
                    &self.r#geo,
                ),
                to_pulumi_object_field(
                    "lang",
                    &self.r#lang,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PageRuleActionsCacheKeyFieldsUser {
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
                    r#device_type: {
                        let field_value = match fields_map.get("device_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#geo: {
                        let field_value = match fields_map.get("geo") {
                            Some(value) => value,
                            None => bail!("Missing field 'geo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lang: {
                        let field_value = match fields_map.get("lang") {
                            Some(value) => value,
                            None => bail!("Missing field 'lang' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
