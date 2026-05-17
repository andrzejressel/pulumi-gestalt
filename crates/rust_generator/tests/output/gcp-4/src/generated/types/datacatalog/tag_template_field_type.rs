#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TagTemplateFieldType {
    /// Represents an enum type.
    /// Exactly one of `primitive_type` or `enum_type` must be set
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "enumType")]
    pub r#enum_type: Option<Box<super::super::types::datacatalog::TagTemplateFieldTypeEnumType>>,
    /// Represents primitive types - string, bool etc.
    /// Exactly one of `primitive_type` or `enum_type` must be set
    /// Possible values are: `DOUBLE`, `STRING`, `BOOL`, `TIMESTAMP`.
    #[builder(into)]
    #[serde(rename = "primitiveType")]
    pub r#primitive_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TagTemplateFieldType {
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
                "enum_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enum_type,
                )
                .await,
            );
            map.insert(
                "primitive_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#primitive_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TagTemplateFieldType {
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
                    r#enum_type: {
                        let field_value = match fields_map.get("enum_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'enum_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primitive_type: {
                        let field_value = match fields_map.get("primitive_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'primitive_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
