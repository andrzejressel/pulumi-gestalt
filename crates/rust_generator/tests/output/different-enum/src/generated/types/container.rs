#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct Container {
    #[builder(into)]
    #[serde(rename = "brightness")]
    pub r#brightness: Option<Box<super::types::ContainerBrightness>>,
    #[builder(into)]
    #[serde(rename = "color")]
    pub r#color: Option<pulumi_gestalt_rust::OneOf2<Box<super::types::ContainerColor>, String>>,
    #[builder(into)]
    #[serde(rename = "material")]
    pub r#material: Option<String>,
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<super::types::ContainerSize>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for Container {
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
                    "brightness",
                    &self.r#brightness,
                ),
                to_pulumi_object_field(
                    "color",
                    &self.r#color,
                ),
                to_pulumi_object_field(
                    "material",
                    &self.r#material,
                ),
                to_pulumi_object_field(
                    "size",
                    &self.r#size,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for Container {
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
                    r#brightness: {
                        let field_value = match fields_map.get("brightness") {
                            Some(value) => value,
                            None => bail!("Missing field 'brightness' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#color: {
                        let field_value = match fields_map.get("color") {
                            Some(value) => value,
                            None => bail!("Missing field 'color' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#material: {
                        let field_value = match fields_map.get("material") {
                            Some(value) => value,
                            None => bail!("Missing field 'material' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#size: {
                        let field_value = match fields_map.get("size") {
                            Some(value) => value,
                            None => bail!("Missing field 'size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
