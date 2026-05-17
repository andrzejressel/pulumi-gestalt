#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecurityDeviceGroupRangeRule {
    /// Specifies the time range. represented in ISO 8601 duration format.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: String,
    /// The maximum threshold in the given time window.
    #[builder(into)]
    #[serde(rename = "max")]
    pub r#max: i32,
    /// The minimum threshold in the given time window.
    #[builder(into)]
    #[serde(rename = "min")]
    pub r#min: i32,
    /// The type of supported rule type. Possible Values are `ActiveConnectionsNotInAllowedRange`, `AmqpC2DMessagesNotInAllowedRange`, `MqttC2DMessagesNotInAllowedRange`, `HttpC2DMessagesNotInAllowedRange`, `AmqpC2DRejectedMessagesNotInAllowedRange`, `MqttC2DRejectedMessagesNotInAllowedRange`, `HttpC2DRejectedMessagesNotInAllowedRange`, `AmqpD2CMessagesNotInAllowedRange`, `MqttD2CMessagesNotInAllowedRange`, `HttpD2CMessagesNotInAllowedRange`, `DirectMethodInvokesNotInAllowedRange`, `FailedLocalLoginsNotInAllowedRange`, `FileUploadsNotInAllowedRange`, `QueuePurgesNotInAllowedRange`, `TwinUpdatesNotInAllowedRange` and `UnauthorizedOperationsNotInAllowedRange`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SecurityDeviceGroupRangeRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#duration,
                )
                .await,
            );
            map.insert(
                "max".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max,
                )
                .await,
            );
            map.insert(
                "min".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SecurityDeviceGroupRangeRule {
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
                    r#duration: {
                        let field_value = match fields_map.get("duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max: {
                        let field_value = match fields_map.get("max") {
                            Some(value) => value,
                            None => bail!("Missing field 'max' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min: {
                        let field_value = match fields_map.get("min") {
                            Some(value) => value,
                            None => bail!("Missing field 'min' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
