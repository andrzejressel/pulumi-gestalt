#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClassificationJobS3JobDefinitionScopingIncludesAnd {
    /// A property-based condition that defines a property, operator, and one or more values for including or excluding an object from the job. (documented below)
    #[builder(into)]
    #[serde(rename = "simpleScopeTerm")]
    pub r#simple_scope_term: Option<Box<super::super::types::macie2::ClassificationJobS3JobDefinitionScopingIncludesAndSimpleScopeTerm>>,
    /// A tag-based condition that defines the operator and tag keys or tag key and value pairs for including or excluding an object from the job. (documented below)
    #[builder(into)]
    #[serde(rename = "tagScopeTerm")]
    pub r#tag_scope_term: Option<Box<super::super::types::macie2::ClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTerm>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClassificationJobS3JobDefinitionScopingIncludesAnd {
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
                "simple_scope_term".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#simple_scope_term,
                )
                .await,
            );
            map.insert(
                "tag_scope_term".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tag_scope_term,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClassificationJobS3JobDefinitionScopingIncludesAnd {
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
                    r#simple_scope_term: {
                        let field_value = match fields_map.get("simple_scope_term") {
                            Some(value) => value,
                            None => bail!("Missing field 'simple_scope_term' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_scope_term: {
                        let field_value = match fields_map.get("tag_scope_term") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_scope_term' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
