#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CollaborationDataEncryptionMetadata {
    #[builder(into)]
    #[serde(rename = "allowClearText")]
    pub r#allow_clear_text: bool,
    #[builder(into)]
    #[serde(rename = "allowDuplicates")]
    pub r#allow_duplicates: bool,
    #[builder(into)]
    #[serde(rename = "allowJoinsOnColumnsWithDifferentNames")]
    pub r#allow_joins_on_columns_with_different_names: bool,
    #[builder(into)]
    #[serde(rename = "preserveNulls")]
    pub r#preserve_nulls: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CollaborationDataEncryptionMetadata {
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
                    "allow_clear_text",
                    &self.r#allow_clear_text,
                ),
                to_pulumi_object_field(
                    "allow_duplicates",
                    &self.r#allow_duplicates,
                ),
                to_pulumi_object_field(
                    "allow_joins_on_columns_with_different_names",
                    &self.r#allow_joins_on_columns_with_different_names,
                ),
                to_pulumi_object_field(
                    "preserve_nulls",
                    &self.r#preserve_nulls,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CollaborationDataEncryptionMetadata {
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
                    r#allow_clear_text: {
                        let field_value = match fields_map.get("allow_clear_text") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_clear_text' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_duplicates: {
                        let field_value = match fields_map.get("allow_duplicates") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_duplicates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_joins_on_columns_with_different_names: {
                        let field_value = match fields_map.get("allow_joins_on_columns_with_different_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_joins_on_columns_with_different_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preserve_nulls: {
                        let field_value = match fields_map.get("preserve_nulls") {
                            Some(value) => value,
                            None => bail!("Missing field 'preserve_nulls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
