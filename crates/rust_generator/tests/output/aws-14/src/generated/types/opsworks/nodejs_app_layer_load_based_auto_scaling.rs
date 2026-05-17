#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NodejsAppLayerLoadBasedAutoScaling {
    #[builder(into)]
    #[serde(rename = "downscaling")]
    pub r#downscaling: Option<Box<super::super::types::opsworks::NodejsAppLayerLoadBasedAutoScalingDownscaling>>,
    #[builder(into)]
    #[serde(rename = "enable")]
    pub r#enable: Option<bool>,
    #[builder(into)]
    #[serde(rename = "upscaling")]
    pub r#upscaling: Option<Box<super::super::types::opsworks::NodejsAppLayerLoadBasedAutoScalingUpscaling>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NodejsAppLayerLoadBasedAutoScaling {
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
                "downscaling".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#downscaling,
                )
                .await,
            );
            map.insert(
                "enable".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable,
                )
                .await,
            );
            map.insert(
                "upscaling".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#upscaling,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NodejsAppLayerLoadBasedAutoScaling {
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
                    r#downscaling: {
                        let field_value = match fields_map.get("downscaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'downscaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable: {
                        let field_value = match fields_map.get("enable") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upscaling: {
                        let field_value = match fields_map.get("upscaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'upscaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
