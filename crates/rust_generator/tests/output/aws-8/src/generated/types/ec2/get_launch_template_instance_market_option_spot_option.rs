#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLaunchTemplateInstanceMarketOptionSpotOption {
    #[builder(into)]
    pub r#block_duration_minutes: i32,
    #[builder(into)]
    pub r#instance_interruption_behavior: String,
    #[builder(into)]
    pub r#max_price: String,
    #[builder(into)]
    pub r#spot_instance_type: String,
    #[builder(into)]
    pub r#valid_until: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLaunchTemplateInstanceMarketOptionSpotOption {
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
                    "blockDurationMinutes",
                    &self.r#block_duration_minutes,
                ),
                to_pulumi_object_field(
                    "instanceInterruptionBehavior",
                    &self.r#instance_interruption_behavior,
                ),
                to_pulumi_object_field(
                    "maxPrice",
                    &self.r#max_price,
                ),
                to_pulumi_object_field(
                    "spotInstanceType",
                    &self.r#spot_instance_type,
                ),
                to_pulumi_object_field(
                    "validUntil",
                    &self.r#valid_until,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("blockDurationMinutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'blockDurationMinutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_interruption_behavior: {
                        let field_value = match fields_map.get("instanceInterruptionBehavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceInterruptionBehavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_price: {
                        let field_value = match fields_map.get("maxPrice") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxPrice' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spot_instance_type: {
                        let field_value = match fields_map.get("spotInstanceType") {
                            Some(value) => value,
                            None => bail!("Missing field 'spotInstanceType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#valid_until: {
                        let field_value = match fields_map.get("validUntil") {
                            Some(value) => value,
                            None => bail!("Missing field 'validUntil' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
