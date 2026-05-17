#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserPoolAdminCreateUserConfig {
    /// - Whether only admins can create users.
    #[builder(into)]
    #[serde(rename = "allowAdminCreateUserOnly")]
    pub r#allow_admin_create_user_only: bool,
    #[builder(into)]
    #[serde(rename = "inviteMessageTemplates")]
    pub r#invite_message_templates: Vec<super::super::types::cognito::GetUserPoolAdminCreateUserConfigInviteMessageTemplate>,
    /// - Number of days an unconfirmed user account remains valid.
    /// * invite_message_template - Templates for invitation messages.
    #[builder(into)]
    #[serde(rename = "unusedAccountValidityDays")]
    pub r#unused_account_validity_days: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetUserPoolAdminCreateUserConfig {
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
                "allow_admin_create_user_only".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_admin_create_user_only,
                )
                .await,
            );
            map.insert(
                "invite_message_templates".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#invite_message_templates,
                )
                .await,
            );
            map.insert(
                "unused_account_validity_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unused_account_validity_days,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetUserPoolAdminCreateUserConfig {
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
                    r#allow_admin_create_user_only: {
                        let field_value = match fields_map.get("allow_admin_create_user_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_admin_create_user_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#invite_message_templates: {
                        let field_value = match fields_map.get("invite_message_templates") {
                            Some(value) => value,
                            None => bail!("Missing field 'invite_message_templates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unused_account_validity_days: {
                        let field_value = match fields_map.get("unused_account_validity_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'unused_account_validity_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
