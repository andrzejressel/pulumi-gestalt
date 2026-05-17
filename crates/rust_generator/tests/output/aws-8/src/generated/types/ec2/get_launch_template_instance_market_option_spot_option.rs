#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLaunchTemplateInstanceMarketOptionSpotOption {
    #[builder(into)]
    #[serde(rename = "blockDurationMinutes")]
    pub r#block_duration_minutes: i32,
    #[builder(into)]
    #[serde(rename = "instanceInterruptionBehavior")]
    pub r#instance_interruption_behavior: String,
    #[builder(into)]
    #[serde(rename = "maxPrice")]
    pub r#max_price: String,
    #[builder(into)]
    #[serde(rename = "spotInstanceType")]
    pub r#spot_instance_type: String,
    #[builder(into)]
    #[serde(rename = "validUntil")]
    pub r#valid_until: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLaunchTemplateInstanceMarketOptionSpotOption {
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
                    "block_duration_minutes",
                    &self.r#block_duration_minutes,
                ),
                to_pulumi_object_field(
                    "instance_interruption_behavior",
                    &self.r#instance_interruption_behavior,
                ),
                to_pulumi_object_field(
                    "max_price",
                    &self.r#max_price,
                ),
                to_pulumi_object_field(
                    "spot_instance_type",
                    &self.r#spot_instance_type,
                ),
                to_pulumi_object_field(
                    "valid_until",
                    &self.r#valid_until,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLaunchTemplateInstanceMarketOptionSpotOption {
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
                    r#block_duration_minutes: {
                        let field_value = match fields_map.get("block_duration_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'block_duration_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_interruption_behavior: {
                        let field_value = match fields_map.get("instance_interruption_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_interruption_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_price: {
                        let field_value = match fields_map.get("max_price") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_price' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spot_instance_type: {
                        let field_value = match fields_map.get("spot_instance_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'spot_instance_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#valid_until: {
                        let field_value = match fields_map.get("valid_until") {
                            Some(value) => value,
                            None => bail!("Missing field 'valid_until' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
