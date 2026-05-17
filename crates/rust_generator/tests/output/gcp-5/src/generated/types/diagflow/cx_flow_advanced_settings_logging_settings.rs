#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxFlowAdvancedSettingsLoggingSettings {
    /// Enables consent-based end-user input redaction, if true, a pre-defined session parameter **$session.params.conversation-redaction** will be used to determine if the utterance should be redacted.
    #[builder(into)]
    #[serde(rename = "enableConsentBasedRedaction")]
    pub r#enable_consent_based_redaction: Option<bool>,
    /// Enables DF Interaction logging.
    #[builder(into)]
    #[serde(rename = "enableInteractionLogging")]
    pub r#enable_interaction_logging: Option<bool>,
    /// Enables Google Cloud Logging.
    #[builder(into)]
    #[serde(rename = "enableStackdriverLogging")]
    pub r#enable_stackdriver_logging: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxFlowAdvancedSettingsLoggingSettings {
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
                "enable_consent_based_redaction".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_consent_based_redaction,
                )
                .await,
            );
            map.insert(
                "enable_interaction_logging".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_interaction_logging,
                )
                .await,
            );
            map.insert(
                "enable_stackdriver_logging".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_stackdriver_logging,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxFlowAdvancedSettingsLoggingSettings {
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
                    r#enable_consent_based_redaction: {
                        let field_value = match fields_map.get("enable_consent_based_redaction") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_consent_based_redaction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_interaction_logging: {
                        let field_value = match fields_map.get("enable_interaction_logging") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_interaction_logging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_stackdriver_logging: {
                        let field_value = match fields_map.get("enable_stackdriver_logging") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_stackdriver_logging' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
