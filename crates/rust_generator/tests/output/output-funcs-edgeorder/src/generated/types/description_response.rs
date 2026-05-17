#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DescriptionResponse {
    /// Attributes for the product system.
    #[builder(into)]
    #[serde(rename = "attributes")]
    pub r#attributes: Vec<String>,
    /// Type of description.
    #[builder(into)]
    #[serde(rename = "descriptionType")]
    pub r#description_type: String,
    /// Keywords for the product system.
    #[builder(into)]
    #[serde(rename = "keywords")]
    pub r#keywords: Vec<String>,
    /// Links for the product system.
    #[builder(into)]
    #[serde(rename = "links")]
    pub r#links: Vec<super::types::LinkResponse>,
    /// Long description of the product system.
    #[builder(into)]
    #[serde(rename = "longDescription")]
    pub r#long_description: String,
    /// Short description of the product system.
    #[builder(into)]
    #[serde(rename = "shortDescription")]
    pub r#short_description: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DescriptionResponse {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "attributes",
                    &self.r#attributes,
                ),
                to_pulumi_object_field(
                    "description_type",
                    &self.r#description_type,
                ),
                to_pulumi_object_field(
                    "keywords",
                    &self.r#keywords,
                ),
                to_pulumi_object_field(
                    "links",
                    &self.r#links,
                ),
                to_pulumi_object_field(
                    "long_description",
                    &self.r#long_description,
                ),
                to_pulumi_object_field(
                    "short_description",
                    &self.r#short_description,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DescriptionResponse {
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
                    r#attributes: {
                        let field_value = match fields_map.get("attributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'attributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description_type: {
                        let field_value = match fields_map.get("description_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'description_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#keywords: {
                        let field_value = match fields_map.get("keywords") {
                            Some(value) => value,
                            None => bail!("Missing field 'keywords' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#links: {
                        let field_value = match fields_map.get("links") {
                            Some(value) => value,
                            None => bail!("Missing field 'links' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#long_description: {
                        let field_value = match fields_map.get("long_description") {
                            Some(value) => value,
                            None => bail!("Missing field 'long_description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#short_description: {
                        let field_value = match fields_map.get("short_description") {
                            Some(value) => value,
                            None => bail!("Missing field 'short_description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
