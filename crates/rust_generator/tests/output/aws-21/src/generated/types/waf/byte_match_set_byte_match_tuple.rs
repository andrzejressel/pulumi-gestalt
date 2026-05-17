#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ByteMatchSetByteMatchTuple {
    /// The part of a web request that you want to search, such as a specified header or a query string.
    #[builder(into)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Box<super::super::types::waf::ByteMatchSetByteMatchTupleFieldToMatch>,
    /// Within the portion of a web request that you want to search
    /// (for example, in the query string, if any), specify where you want to search.
    /// e.g., `CONTAINS`, `CONTAINS_WORD` or `EXACTLY`.
    /// See [docs](http://docs.aws.amazon.com/waf/latest/APIReference/API_ByteMatchTuple.html#WAF-Type-ByteMatchTuple-PositionalConstraint)
    /// for all supported values.
    #[builder(into)]
    #[serde(rename = "positionalConstraint")]
    pub r#positional_constraint: String,
    /// The value that you want to search for within the field specified by `field_to_match`, e.g., `badrefer1`.
    /// See [docs](https://docs.aws.amazon.com/waf/latest/APIReference/API_waf_ByteMatchTuple.html)
    /// for all supported values.
    #[builder(into)]
    #[serde(rename = "targetString")]
    pub r#target_string: Option<String>,
    /// Text transformations used to eliminate unusual formatting that attackers use in web requests in an effort to bypass AWS WAF.
    /// If you specify a transformation, AWS WAF performs the transformation on `target_string` before inspecting a request for a match.
    /// e.g., `CMD_LINE`, `HTML_ENTITY_DECODE` or `NONE`.
    /// See [docs](http://docs.aws.amazon.com/waf/latest/APIReference/API_ByteMatchTuple.html#WAF-Type-ByteMatchTuple-TextTransformation)
    /// for all supported values.
    #[builder(into)]
    #[serde(rename = "textTransformation")]
    pub r#text_transformation: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ByteMatchSetByteMatchTuple {
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
                "field_to_match".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#field_to_match,
                )
                .await,
            );
            map.insert(
                "positional_constraint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#positional_constraint,
                )
                .await,
            );
            map.insert(
                "target_string".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_string,
                )
                .await,
            );
            map.insert(
                "text_transformation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#text_transformation,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ByteMatchSetByteMatchTuple {
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
                    r#field_to_match: {
                        let field_value = match fields_map.get("field_to_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_to_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#positional_constraint: {
                        let field_value = match fields_map.get("positional_constraint") {
                            Some(value) => value,
                            None => bail!("Missing field 'positional_constraint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_string: {
                        let field_value = match fields_map.get("target_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#text_transformation: {
                        let field_value = match fields_map.get("text_transformation") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_transformation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
