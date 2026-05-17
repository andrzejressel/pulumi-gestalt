#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelInputAttachmentAutomaticInputFailoverSettings {
    /// This clear time defines the requirement a recovered input must meet to be considered healthy. The input must have no failover conditions for this length of time. Enter a time in milliseconds. This value is particularly important if the input\_preference for the failover pair is set to PRIMARY\_INPUT\_PREFERRED, because after this time, MediaLive will switch back to the primary input.
    #[builder(into)]
    #[serde(rename = "errorClearTimeMsec")]
    pub r#error_clear_time_msec: Option<i32>,
    /// A list of failover conditions. If any of these conditions occur, MediaLive will perform a failover to the other input. See Failover Condition Block for more details.
    #[builder(into)]
    #[serde(rename = "failoverConditions")]
    pub r#failover_conditions: Option<Vec<super::super::types::medialive::ChannelInputAttachmentAutomaticInputFailoverSettingsFailoverCondition>>,
    /// Input preference when deciding which input to make active when a previously failed input has recovered.
    #[builder(into)]
    #[serde(rename = "inputPreference")]
    pub r#input_preference: Option<String>,
    /// The input ID of the secondary input in the automatic input failover pair.
    #[builder(into)]
    #[serde(rename = "secondaryInputId")]
    pub r#secondary_input_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelInputAttachmentAutomaticInputFailoverSettings {
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
                "error_clear_time_msec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#error_clear_time_msec,
                )
                .await,
            );
            map.insert(
                "failover_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#failover_conditions,
                )
                .await,
            );
            map.insert(
                "input_preference".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_preference,
                )
                .await,
            );
            map.insert(
                "secondary_input_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secondary_input_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelInputAttachmentAutomaticInputFailoverSettings {
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
                    r#error_clear_time_msec: {
                        let field_value = match fields_map.get("error_clear_time_msec") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_clear_time_msec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failover_conditions: {
                        let field_value = match fields_map.get("failover_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'failover_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_preference: {
                        let field_value = match fields_map.get("input_preference") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_preference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secondary_input_id: {
                        let field_value = match fields_map.get("secondary_input_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary_input_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
