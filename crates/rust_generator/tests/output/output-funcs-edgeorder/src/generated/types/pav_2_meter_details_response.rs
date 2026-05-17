#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct Pav2MeterDetailsResponse {
    /// Represents billing type.
    /// Expected value is 'Pav2'.
    #[builder(skip)]
    #[serde(rename = "billingType")]
    r#billing_type: super::constants::ConstStringPav2,
    /// Charging type.
    #[builder(into)]
    #[serde(rename = "chargingType")]
    pub r#charging_type: String,
    /// Validation status of requested data center and transport.
    #[builder(into)]
    #[serde(rename = "meterGuid")]
    pub r#meter_guid: String,
    /// Billing unit applicable for Pav2 billing
    #[builder(into)]
    #[serde(rename = "multiplier")]
    pub r#multiplier: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for Pav2MeterDetailsResponse {
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
                    "billing_type",
                    &self.r#billing_type,
                ),
                to_pulumi_object_field(
                    "charging_type",
                    &self.r#charging_type,
                ),
                to_pulumi_object_field(
                    "meter_guid",
                    &self.r#meter_guid,
                ),
                to_pulumi_object_field(
                    "multiplier",
                    &self.r#multiplier,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("billing_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'billing_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#charging_type: {
                        let field_value = match fields_map.get("charging_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'charging_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#meter_guid: {
                        let field_value = match fields_map.get("meter_guid") {
                            Some(value) => value,
                            None => bail!("Missing field 'meter_guid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
