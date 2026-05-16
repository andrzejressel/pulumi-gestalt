#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SizeConstraintSetSizeConstraint {
    /// The type of comparison you want to perform.
    /// e.g., `EQ`, `NE`, `LT`, `GT`.
    /// See [docs](https://docs.aws.amazon.com/waf/latest/APIReference/API_wafRegional_SizeConstraint.html) for all supported values.
    #[builder(into)]
    #[serde(rename = "comparisonOperator")]
    pub r#comparison_operator: String,
    /// Specifies where in a web request to look for the size constraint.
    #[builder(into)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Box<super::super::types::wafregional::SizeConstraintSetSizeConstraintFieldToMatch>,
    /// The size in bytes that you want to compare against the size of the specified `field_to_match`.
    /// Valid values are between 0 - 21474836480 bytes (0 - 20 GB).
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: i32,
    /// Text transformations used to eliminate unusual formatting that attackers use in web requests in an effort to bypass AWS WAF.
    /// If you specify a transformation, AWS WAF performs the transformation on `field_to_match` before inspecting a request for a match.
    /// e.g., `CMD_LINE`, `HTML_ENTITY_DECODE` or `NONE`.
    /// See [docs](http://docs.aws.amazon.com/waf/latest/APIReference/API_SizeConstraint.html#WAF-Type-SizeConstraint-TextTransformation)
    /// for all supported values.
    /// **Note:** if you choose `BODY` as `type`, you must choose `NONE` because CloudFront forwards only the first 8192 bytes for inspection.
    #[builder(into)]
    #[serde(rename = "textTransformation")]
    pub r#text_transformation: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SizeConstraintSetSizeConstraint {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("comparison_operator".to_string(), self.r#comparison_operator.to_pulumi_value().await);
            map.insert("field_to_match".to_string(), self.r#field_to_match.to_pulumi_value().await);
            map.insert("size".to_string(), self.r#size.to_pulumi_value().await);
            map.insert("text_transformation".to_string(), self.r#text_transformation.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SizeConstraintSetSizeConstraint {
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
                    r#comparison_operator: {
                        let field_value = match fields_map.get("comparison_operator") {
                            Some(value) => value,
                            None => bail!("Missing field 'comparison_operator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#field_to_match: {
                        let field_value = match fields_map.get("field_to_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_to_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::wafregional::SizeConstraintSetSizeConstraintFieldToMatch> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#size: {
                        let field_value = match fields_map.get("size") {
                            Some(value) => value,
                            None => bail!("Missing field 'size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
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
