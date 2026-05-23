#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ByteMatchSetByteMatchTuple {
    /// Settings for the ByteMatchTuple. FieldToMatch documented below.
    #[builder(into)]
    pub r#field_to_match: Box<super::super::types::wafregional::ByteMatchSetByteMatchTupleFieldToMatch>,
    /// Within the portion of a web request that you want to search.
    #[builder(into)]
    pub r#positional_constraint: String,
    /// The value that you want AWS WAF to search for. The maximum length of the value is 50 bytes.
    #[builder(into)]
    pub r#target_string: Option<String>,
    /// The formatting way for web request.
    /// 
    /// FieldToMatch(field_to_match) support following:
    #[builder(into)]
    pub r#text_transformation: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ByteMatchSetByteMatchTuple {
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
                    "fieldToMatch",
                    &self.r#field_to_match,
                ),
                to_pulumi_object_field(
                    "positionalConstraint",
                    &self.r#positional_constraint,
                ),
                to_pulumi_object_field(
                    "targetString",
                    &self.r#target_string,
                ),
                to_pulumi_object_field(
                    "textTransformation",
                    &self.r#text_transformation,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("fieldToMatch") {
                            Some(value) => value,
                            None => bail!("Missing field 'fieldToMatch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#positional_constraint: {
                        let field_value = match fields_map.get("positionalConstraint") {
                            Some(value) => value,
                            None => bail!("Missing field 'positionalConstraint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_string: {
                        let field_value = match fields_map.get("targetString") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetString' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#text_transformation: {
                        let field_value = match fields_map.get("textTransformation") {
                            Some(value) => value,
                            None => bail!("Missing field 'textTransformation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
