#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetScheduledQueryRulesAlertAction {
    /// List of action group reference resource IDs.
    #[builder(into)]
    #[serde(rename = "actionGroups")]
    pub r#action_groups: Vec<String>,
    /// Custom payload to be sent for all webhook URI in Azure action group.
    #[builder(into)]
    #[serde(rename = "customWebhookPayload")]
    pub r#custom_webhook_payload: String,
    /// Custom subject override for all email IDs in Azure action group.
    #[builder(into)]
    #[serde(rename = "emailSubject")]
    pub r#email_subject: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetScheduledQueryRulesAlertAction {
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
                "action_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#action_groups,
                )
                .await,
            );
            map.insert(
                "custom_webhook_payload".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_webhook_payload,
                )
                .await,
            );
            map.insert(
                "email_subject".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_subject,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetScheduledQueryRulesAlertAction {
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
                    r#action_groups: {
                        let field_value = match fields_map.get("action_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'action_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_webhook_payload: {
                        let field_value = match fields_map.get("custom_webhook_payload") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_webhook_payload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_subject: {
                        let field_value = match fields_map.get("email_subject") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_subject' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
