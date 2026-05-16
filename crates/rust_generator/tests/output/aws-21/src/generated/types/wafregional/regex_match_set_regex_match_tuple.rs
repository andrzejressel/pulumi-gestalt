#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegexMatchSetRegexMatchTuple {
    /// The part of a web request that you want to search, such as a specified header or a query string.
    #[builder(into)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Box<super::super::types::wafregional::RegexMatchSetRegexMatchTupleFieldToMatch>,
    /// The ID of a Regex Pattern Set.
    #[builder(into)]
    #[serde(rename = "regexPatternSetId")]
    pub r#regex_pattern_set_id: String,
    /// Text transformations used to eliminate unusual formatting that attackers use in web requests in an effort to bypass AWS WAF.
    /// e.g., `CMD_LINE`, `HTML_ENTITY_DECODE` or `NONE`.
    /// See [docs](http://docs.aws.amazon.com/waf/latest/APIReference/API_ByteMatchTuple.html#WAF-Type-ByteMatchTuple-TextTransformation)
    /// for all supported values.
    #[builder(into)]
    #[serde(rename = "textTransformation")]
    pub r#text_transformation: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegexMatchSetRegexMatchTuple {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("field_to_match".to_string(), self.r#field_to_match.to_pulumi_value().await);
            map.insert("regex_pattern_set_id".to_string(), self.r#regex_pattern_set_id.to_pulumi_value().await);
            map.insert("text_transformation".to_string(), self.r#text_transformation.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegexMatchSetRegexMatchTuple {
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
                    r#field_to_match: {
                        let field_value = match fields_map.get("field_to_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_to_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::wafregional::RegexMatchSetRegexMatchTupleFieldToMatch> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#regex_pattern_set_id: {
                        let field_value = match fields_map.get("regex_pattern_set_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'regex_pattern_set_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#text_transformation: {
                        let field_value = match fields_map.get("text_transformation") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_transformation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
