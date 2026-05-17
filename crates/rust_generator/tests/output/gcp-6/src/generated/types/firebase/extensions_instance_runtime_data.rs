#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExtensionsInstanceRuntimeData {
    /// The fatal error state for the extension instance
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fatalError")]
    pub r#fatal_error: Option<Box<super::super::types::firebase::ExtensionsInstanceRuntimeDataFatalError>>,
    /// The processing state for the extension instance
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "processingState")]
    pub r#processing_state: Option<Box<super::super::types::firebase::ExtensionsInstanceRuntimeDataProcessingState>>,
    /// The time of the last state update.
    #[builder(into)]
    #[serde(rename = "stateUpdateTime")]
    pub r#state_update_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ExtensionsInstanceRuntimeData {
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
                "fatal_error".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fatal_error,
                )
                .await,
            );
            map.insert(
                "processing_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#processing_state,
                )
                .await,
            );
            map.insert(
                "state_update_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#state_update_time,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ExtensionsInstanceRuntimeData {
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
                    r#fatal_error: {
                        let field_value = match fields_map.get("fatal_error") {
                            Some(value) => value,
                            None => bail!("Missing field 'fatal_error' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#processing_state: {
                        let field_value = match fields_map.get("processing_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'processing_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state_update_time: {
                        let field_value = match fields_map.get("state_update_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'state_update_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
