#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DimensionsResponse {
    /// Depth of the device.
    #[builder(into)]
    #[serde(rename = "depth")]
    pub r#depth: f64,
    /// Height of the device.
    #[builder(into)]
    #[serde(rename = "height")]
    pub r#height: f64,
    /// Length of the device.
    #[builder(into)]
    #[serde(rename = "length")]
    pub r#length: f64,
    /// Unit for the dimensions of length, height and width.
    #[builder(into)]
    #[serde(rename = "lengthHeightUnit")]
    pub r#length_height_unit: String,
    /// Weight of the device.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: f64,
    /// Unit for the dimensions of weight.
    #[builder(into)]
    #[serde(rename = "weightUnit")]
    pub r#weight_unit: String,
    /// Width of the device.
    #[builder(into)]
    #[serde(rename = "width")]
    pub r#width: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DimensionsResponse {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "depth",
                    &self.r#depth,
                ),
                to_pulumi_object_field(
                    "height",
                    &self.r#height,
                ),
                to_pulumi_object_field(
                    "length",
                    &self.r#length,
                ),
                to_pulumi_object_field(
                    "length_height_unit",
                    &self.r#length_height_unit,
                ),
                to_pulumi_object_field(
                    "weight",
                    &self.r#weight,
                ),
                to_pulumi_object_field(
                    "weight_unit",
                    &self.r#weight_unit,
                ),
                to_pulumi_object_field(
                    "width",
                    &self.r#width,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DimensionsResponse {
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
                    r#depth: {
                        let field_value = match fields_map.get("depth") {
                            Some(value) => value,
                            None => bail!("Missing field 'depth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#height: {
                        let field_value = match fields_map.get("height") {
                            Some(value) => value,
                            None => bail!("Missing field 'height' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#length: {
                        let field_value = match fields_map.get("length") {
                            Some(value) => value,
                            None => bail!("Missing field 'length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#length_height_unit: {
                        let field_value = match fields_map.get("length_height_unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'length_height_unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weight: {
                        let field_value = match fields_map.get("weight") {
                            Some(value) => value,
                            None => bail!("Missing field 'weight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weight_unit: {
                        let field_value = match fields_map.get("weight_unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'weight_unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#width: {
                        let field_value = match fields_map.get("width") {
                            Some(value) => value,
                            None => bail!("Missing field 'width' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
