#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HttpRouteRuleMatchHeader {
    /// The value of the header should match exactly the content of exactMatch.
    #[builder(into)]
    #[serde(rename = "exactMatch")]
    pub r#exact_match: Option<String>,
    /// The name of the HTTP header to match against.
    #[builder(into)]
    #[serde(rename = "header")]
    pub r#header: Option<String>,
    /// If specified, the match result will be inverted before checking. Default value is set to false.
    #[builder(into)]
    #[serde(rename = "invertMatch")]
    pub r#invert_match: Option<bool>,
    /// The value of the header must start with the contents of prefixMatch.
    #[builder(into)]
    #[serde(rename = "prefixMatch")]
    pub r#prefix_match: Option<String>,
    /// A header with headerName must exist. The match takes place whether or not the header has a value.
    #[builder(into)]
    #[serde(rename = "presentMatch")]
    pub r#present_match: Option<bool>,
    /// If specified, the rule will match if the request header value is within the range.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rangeMatch")]
    pub r#range_match: Option<Box<super::super::types::networkservices::HttpRouteRuleMatchHeaderRangeMatch>>,
    /// The value of the header must match the regular expression specified in regexMatch.
    #[builder(into)]
    #[serde(rename = "regexMatch")]
    pub r#regex_match: Option<String>,
    /// The value of the header must end with the contents of suffixMatch.
    #[builder(into)]
    #[serde(rename = "suffixMatch")]
    pub r#suffix_match: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HttpRouteRuleMatchHeader {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "exact_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exact_match,
                )
                .await,
            );
            map.insert(
                "header".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#header,
                )
                .await,
            );
            map.insert(
                "invert_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#invert_match,
                )
                .await,
            );
            map.insert(
                "prefix_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#prefix_match,
                )
                .await,
            );
            map.insert(
                "present_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#present_match,
                )
                .await,
            );
            map.insert(
                "range_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#range_match,
                )
                .await,
            );
            map.insert(
                "regex_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#regex_match,
                )
                .await,
            );
            map.insert(
                "suffix_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#suffix_match,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HttpRouteRuleMatchHeader {
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
                    r#exact_match: {
                        let field_value = match fields_map.get("exact_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'exact_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#header: {
                        let field_value = match fields_map.get("header") {
                            Some(value) => value,
                            None => bail!("Missing field 'header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#invert_match: {
                        let field_value = match fields_map.get("invert_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'invert_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_match: {
                        let field_value = match fields_map.get("prefix_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#present_match: {
                        let field_value = match fields_map.get("present_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'present_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#range_match: {
                        let field_value = match fields_map.get("range_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'range_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regex_match: {
                        let field_value = match fields_map.get("regex_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'regex_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#suffix_match: {
                        let field_value = match fields_map.get("suffix_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'suffix_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
