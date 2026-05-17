#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AvailabilityInformationResponse {
    /// Current availability stage of the product. Availability stage
    #[builder(into)]
    #[serde(rename = "availabilityStage")]
    pub r#availability_stage: String,
    /// Reason why the product is disabled.
    #[builder(into)]
    #[serde(rename = "disabledReason")]
    pub r#disabled_reason: String,
    /// Message for why the product is disabled.
    #[builder(into)]
    #[serde(rename = "disabledReasonMessage")]
    pub r#disabled_reason_message: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AvailabilityInformationResponse {
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
                "availability_stage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#availability_stage,
                )
                .await,
            );
            map.insert(
                "disabled_reason".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disabled_reason,
                )
                .await,
            );
            map.insert(
                "disabled_reason_message".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disabled_reason_message,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AvailabilityInformationResponse {
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
                    r#availability_stage: {
                        let field_value = match fields_map.get("availability_stage") {
                            Some(value) => value,
                            None => bail!("Missing field 'availability_stage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disabled_reason: {
                        let field_value = match fields_map.get("disabled_reason") {
                            Some(value) => value,
                            None => bail!("Missing field 'disabled_reason' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disabled_reason_message: {
                        let field_value = match fields_map.get("disabled_reason_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'disabled_reason_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
