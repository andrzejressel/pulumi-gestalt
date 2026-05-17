#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubscriptionPushConfigOidcToken {
    /// Audience to be used when generating OIDC token. The audience claim
    /// identifies the recipients that the JWT is intended for. The audience
    /// value is a single case-sensitive string. Having multiple values (array)
    /// for the audience field is not supported. More info about the OIDC JWT
    /// token audience here: https://tools.ietf.org/html/rfc7519#section-4.1.3
    /// Note: if not specified, the Push endpoint URL will be used.
    #[builder(into)]
    #[serde(rename = "audience")]
    pub r#audience: Option<String>,
    /// Service account email to be used for generating the OIDC token.
    /// The caller (for subscriptions.create, subscriptions.patch, and
    /// subscriptions.modifyPushConfig RPCs) must have the
    /// iam.serviceAccounts.actAs permission for the service account.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SubscriptionPushConfigOidcToken {
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
                "audience".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#audience,
                )
                .await,
            );
            map.insert(
                "service_account_email".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_account_email,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SubscriptionPushConfigOidcToken {
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
                    r#audience: {
                        let field_value = match fields_map.get("audience") {
                            Some(value) => value,
                            None => bail!("Missing field 'audience' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_email: {
                        let field_value = match fields_map.get("service_account_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
