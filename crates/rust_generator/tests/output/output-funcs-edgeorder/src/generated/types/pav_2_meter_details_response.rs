#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct Pav2MeterDetailsResponse {
    /// Represents billing type.
    /// Expected value is 'Pav2'.
    #[builder(skip)]
    r#billing_type: super::constants::ConstStringPav2,
    /// Charging type.
    #[builder(into)]
    pub r#charging_type: String,
    /// Validation status of requested data center and transport.
    #[builder(into)]
    pub r#meter_guid: String,
    /// Billing unit applicable for Pav2 billing
    #[builder(into)]
    pub r#multiplier: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for Pav2MeterDetailsResponse {
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
                    "billingType",
                    &self.r#billing_type,
                ),
                to_pulumi_object_field(
                    "chargingType",
                    &self.r#charging_type,
                ),
                to_pulumi_object_field(
                    "meterGuid",
                    &self.r#meter_guid,
                ),
                to_pulumi_object_field(
                    "multiplier",
                    &self.r#multiplier,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for Pav2MeterDetailsResponse {
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
                    r#billing_type: {
                        let field_value = match fields_map.get("billingType") {
                            Some(value) => value,
                            None => bail!("Missing field 'billingType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#charging_type: {
                        let field_value = match fields_map.get("chargingType") {
                            Some(value) => value,
                            None => bail!("Missing field 'chargingType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#meter_guid: {
                        let field_value = match fields_map.get("meterGuid") {
                            Some(value) => value,
                            None => bail!("Missing field 'meterGuid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#multiplier: {
                        let field_value = match fields_map.get("multiplier") {
                            Some(value) => value,
                            None => bail!("Missing field 'multiplier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
