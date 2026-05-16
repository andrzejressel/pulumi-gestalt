#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomLayerLoadBasedAutoScaling {
    /// The downscaling settings, as defined below, used for load-based autoscaling
    #[builder(into)]
    #[serde(rename = "downscaling")]
    pub r#downscaling: Option<Box<super::super::types::opsworks::CustomLayerLoadBasedAutoScalingDownscaling>>,
    /// Whether load-based auto scaling is enabled for the layer.
    #[builder(into)]
    #[serde(rename = "enable")]
    pub r#enable: Option<bool>,
    /// The upscaling settings, as defined below, used for load-based autoscaling
    #[builder(into)]
    #[serde(rename = "upscaling")]
    pub r#upscaling: Option<Box<super::super::types::opsworks::CustomLayerLoadBasedAutoScalingUpscaling>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CustomLayerLoadBasedAutoScaling {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("downscaling".to_string(), self.r#downscaling.to_pulumi_value().await);
            map.insert("enable".to_string(), self.r#enable.to_pulumi_value().await);
            map.insert("upscaling".to_string(), self.r#upscaling.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CustomLayerLoadBasedAutoScaling {
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
                    r#downscaling: {
                        let field_value = match fields_map.get("downscaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'downscaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::opsworks::CustomLayerLoadBasedAutoScalingDownscaling>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enable: {
                        let field_value = match fields_map.get("enable") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#upscaling: {
                        let field_value = match fields_map.get("upscaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'upscaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::opsworks::CustomLayerLoadBasedAutoScalingUpscaling>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
