#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IndexField {
    /// Indicates that this field supports operations on arrayValues. Only one of `order`, `arrayConfig`, and
    /// `vectorConfig` can be specified.
    /// Possible values are: `CONTAINS`.
    #[builder(into)]
    #[serde(rename = "arrayConfig")]
    pub r#array_config: Option<String>,
    /// Name of the field.
    #[builder(into)]
    #[serde(rename = "fieldPath")]
    pub r#field_path: Option<String>,
    /// Indicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=.
    /// Only one of `order`, `arrayConfig`, and `vectorConfig` can be specified.
    /// Possible values are: `ASCENDING`, `DESCENDING`.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Option<String>,
    /// Indicates that this field supports vector search operations. Only one of `order`, `arrayConfig`, and
    /// `vectorConfig` can be specified. Vector Fields should come after the field path `__name__`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vectorConfig")]
    pub r#vector_config: Option<Box<super::super::types::firestore::IndexFieldVectorConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IndexField {
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
                    "array_config",
                    &self.r#array_config,
                ),
                to_pulumi_object_field(
                    "field_path",
                    &self.r#field_path,
                ),
                to_pulumi_object_field(
                    "order",
                    &self.r#order,
                ),
                to_pulumi_object_field(
                    "vector_config",
                    &self.r#vector_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IndexField {
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
                    r#array_config: {
                        let field_value = match fields_map.get("array_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'array_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#field_path: {
                        let field_value = match fields_map.get("field_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#order: {
                        let field_value = match fields_map.get("order") {
                            Some(value) => value,
                            None => bail!("Missing field 'order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vector_config: {
                        let field_value = match fields_map.get("vector_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'vector_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
