#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamProcessorRegionsOfInterestBoundingBox {
    /// Height of the bounding box as a ratio of the overall image height.
    #[builder(into)]
    #[serde(rename = "height")]
    pub r#height: Option<f64>,
    /// Left coordinate of the bounding box as a ratio of overall image width.
    #[builder(into)]
    #[serde(rename = "left")]
    pub r#left: Option<f64>,
    /// Top coordinate of the bounding box as a ratio of overall image height.
    #[builder(into)]
    #[serde(rename = "top")]
    pub r#top: Option<f64>,
    /// Width of the bounding box as a ratio of the overall image width.
    #[builder(into)]
    #[serde(rename = "width")]
    pub r#width: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StreamProcessorRegionsOfInterestBoundingBox {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("height".to_string(), self.r#height.to_pulumi_value().await);
            map.insert("left".to_string(), self.r#left.to_pulumi_value().await);
            map.insert("top".to_string(), self.r#top.to_pulumi_value().await);
            map.insert("width".to_string(), self.r#width.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StreamProcessorRegionsOfInterestBoundingBox {
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
                    r#height: {
                        let field_value = match fields_map.get("height") {
                            Some(value) => value,
                            None => bail!("Missing field 'height' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<f64> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#left: {
                        let field_value = match fields_map.get("left") {
                            Some(value) => value,
                            None => bail!("Missing field 'left' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<f64> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#top: {
                        let field_value = match fields_map.get("top") {
                            Some(value) => value,
                            None => bail!("Missing field 'top' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<f64> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#width: {
                        let field_value = match fields_map.get("width") {
                            Some(value) => value,
                            None => bail!("Missing field 'width' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<f64> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
