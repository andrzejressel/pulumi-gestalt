#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RoleManagementPolicyNotificationRulesActiveAssignmentsApproverNotifications {
    /// The additional recipients to notify
    #[builder(into)]
    #[serde(rename = "additionalRecipients")]
    pub r#additional_recipients: Option<Vec<String>>,
    /// Whether the default recipients are notified
    #[builder(into)]
    #[serde(rename = "defaultRecipients")]
    pub r#default_recipients: bool,
    /// What level of notifications are sent
    #[builder(into)]
    #[serde(rename = "notificationLevel")]
    pub r#notification_level: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RoleManagementPolicyNotificationRulesActiveAssignmentsApproverNotifications {
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
                "additional_recipients".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#additional_recipients,
                )
                .await,
            );
            map.insert(
                "default_recipients".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_recipients,
                )
                .await,
            );
            map.insert(
                "notification_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#notification_level,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RoleManagementPolicyNotificationRulesActiveAssignmentsApproverNotifications {
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
                    r#additional_recipients: {
                        let field_value = match fields_map.get("additional_recipients") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_recipients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_recipients: {
                        let field_value = match fields_map.get("default_recipients") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_recipients' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notification_level: {
                        let field_value = match fields_map.get("notification_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'notification_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
