#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HubApnsCredential {
    /// The Application Mode which defines which server the APNS Messages should be sent to. Possible values are `Production` and `Sandbox`.
    #[builder(into)]
    #[serde(rename = "applicationMode")]
    pub r#application_mode: String,
    /// The Bundle ID of the iOS/macOS application to send push notifications for, such as `com.org.example`.
    #[builder(into)]
    #[serde(rename = "bundleId")]
    pub r#bundle_id: String,
    /// The Apple Push Notifications Service (APNS) Key.
    #[builder(into)]
    #[serde(rename = "keyId")]
    pub r#key_id: String,
    /// The ID of the team the Token.
    #[builder(into)]
    #[serde(rename = "teamId")]
    pub r#team_id: String,
    /// The Push Token associated with the Apple Developer Account. This is the contents of the `key` downloaded from [the Apple Developer Portal](https://developer.apple.com/account/ios/authkey/) between the `-----BEGIN PRIVATE KEY-----` and `-----END PRIVATE KEY-----` blocks.
    #[builder(into)]
    #[serde(rename = "token")]
    pub r#token: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HubApnsCredential {
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
                "application_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#application_mode,
                )
                .await,
            );
            map.insert(
                "bundle_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bundle_id,
                )
                .await,
            );
            map.insert(
                "key_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#key_id,
                )
                .await,
            );
            map.insert(
                "team_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#team_id,
                )
                .await,
            );
            map.insert(
                "token".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#token,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HubApnsCredential {
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
                    r#application_mode: {
                        let field_value = match fields_map.get("application_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bundle_id: {
                        let field_value = match fields_map.get("bundle_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'bundle_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key_id: {
                        let field_value = match fields_map.get("key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#team_id: {
                        let field_value = match fields_map.get("team_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'team_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token: {
                        let field_value = match fields_map.get("token") {
                            Some(value) => value,
                            None => bail!("Missing field 'token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
