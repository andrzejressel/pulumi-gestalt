#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SelectionCondition {
    #[builder(into)]
    #[serde(rename = "stringEquals")]
    pub r#string_equals: Option<Vec<super::super::types::backup::SelectionConditionStringEqual>>,
    #[builder(into)]
    #[serde(rename = "stringLikes")]
    pub r#string_likes: Option<Vec<super::super::types::backup::SelectionConditionStringLike>>,
    #[builder(into)]
    #[serde(rename = "stringNotEquals")]
    pub r#string_not_equals: Option<Vec<super::super::types::backup::SelectionConditionStringNotEqual>>,
    #[builder(into)]
    #[serde(rename = "stringNotLikes")]
    pub r#string_not_likes: Option<Vec<super::super::types::backup::SelectionConditionStringNotLike>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SelectionCondition {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("string_equals".to_string(), self.r#string_equals.to_pulumi_value().await);
            map.insert("string_likes".to_string(), self.r#string_likes.to_pulumi_value().await);
            map.insert("string_not_equals".to_string(), self.r#string_not_equals.to_pulumi_value().await);
            map.insert("string_not_likes".to_string(), self.r#string_not_likes.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SelectionCondition {
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
                    r#string_equals: {
                        let field_value = match fields_map.get("string_equals") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_equals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::backup::SelectionConditionStringEqual>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#string_likes: {
                        let field_value = match fields_map.get("string_likes") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_likes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::backup::SelectionConditionStringLike>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#string_not_equals: {
                        let field_value = match fields_map.get("string_not_equals") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_not_equals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::backup::SelectionConditionStringNotEqual>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#string_not_likes: {
                        let field_value = match fields_map.get("string_not_likes") {
                            Some(value) => value,
                            None => bail!("Missing field 'string_not_likes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::backup::SelectionConditionStringNotLike>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
