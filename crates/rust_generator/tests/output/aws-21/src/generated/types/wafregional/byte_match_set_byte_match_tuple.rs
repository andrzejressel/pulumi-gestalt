#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ByteMatchSetByteMatchTuple {
    /// Settings for the ByteMatchTuple. FieldToMatch documented below.
    #[builder(into)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Box<super::super::types::wafregional::ByteMatchSetByteMatchTupleFieldToMatch>,
    /// Within the portion of a web request that you want to search.
    #[builder(into)]
    #[serde(rename = "positionalConstraint")]
    pub r#positional_constraint: String,
    /// The value that you want AWS WAF to search for. The maximum length of the value is 50 bytes.
    #[builder(into)]
    #[serde(rename = "targetString")]
    pub r#target_string: Option<String>,
    /// The formatting way for web request.
    /// 
    /// FieldToMatch(field_to_match) support following:
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

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("field_to_match".to_string(), self.r#field_to_match.to_pulumi_value().await);
            map.insert("positional_constraint".to_string(), self.r#positional_constraint.to_pulumi_value().await);
            map.insert("target_string".to_string(), self.r#target_string.to_pulumi_value().await);
            map.insert("text_transformation".to_string(), self.r#text_transformation.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ByteMatchSetByteMatchTuple {
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
                        <Box<super::super::types::wafregional::ByteMatchSetByteMatchTupleFieldToMatch> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#positional_constraint: {
                        let field_value = match fields_map.get("positional_constraint") {
                            Some(value) => value,
                            None => bail!("Missing field 'positional_constraint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#target_string: {
                        let field_value = match fields_map.get("target_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
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
