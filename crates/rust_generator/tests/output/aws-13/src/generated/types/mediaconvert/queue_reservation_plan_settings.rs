#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct QueueReservationPlanSettings {
    /// The length of the term of your reserved queue pricing plan commitment. Valid value is `ONE_YEAR`.
    #[builder(into)]
    #[serde(rename = "commitment")]
    pub r#commitment: String,
    /// Specifies whether the term of your reserved queue pricing plan. Valid values are `AUTO_RENEW` or `EXPIRE`.
    #[builder(into)]
    #[serde(rename = "renewalType")]
    pub r#renewal_type: String,
    /// Specifies the number of reserved transcode slots (RTS) for queue.
    #[builder(into)]
    #[serde(rename = "reservedSlots")]
    pub r#reserved_slots: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for QueueReservationPlanSettings {
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
                "commitment".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#commitment,
                )
                .await,
            );
            map.insert(
                "renewal_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#renewal_type,
                )
                .await,
            );
            map.insert(
                "reserved_slots".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reserved_slots,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for QueueReservationPlanSettings {
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
                    r#commitment: {
                        let field_value = match fields_map.get("commitment") {
                            Some(value) => value,
                            None => bail!("Missing field 'commitment' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#renewal_type: {
                        let field_value = match fields_map.get("renewal_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'renewal_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reserved_slots: {
                        let field_value = match fields_map.get("reserved_slots") {
                            Some(value) => value,
                            None => bail!("Missing field 'reserved_slots' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
