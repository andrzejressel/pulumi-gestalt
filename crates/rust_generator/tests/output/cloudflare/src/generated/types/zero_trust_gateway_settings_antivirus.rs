#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustGatewaySettingsAntivirus {
    /// Scan on file download.
    #[builder(into)]
    #[serde(rename = "enabledDownloadPhase")]
    pub r#enabled_download_phase: bool,
    /// Scan on file upload.
    #[builder(into)]
    #[serde(rename = "enabledUploadPhase")]
    pub r#enabled_upload_phase: bool,
    /// Block requests for files that cannot be scanned.
    #[builder(into)]
    #[serde(rename = "failClosed")]
    pub r#fail_closed: bool,
    /// Set notifications for antivirus.
    #[builder(into)]
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings: Option<Box<super::types::ZeroTrustGatewaySettingsAntivirusNotificationSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ZeroTrustGatewaySettingsAntivirus {
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
                "enabled_download_phase".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled_download_phase,
                )
                .await,
            );
            map.insert(
                "enabled_upload_phase".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled_upload_phase,
                )
                .await,
            );
            map.insert(
                "fail_closed".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fail_closed,
                )
                .await,
            );
            map.insert(
                "notification_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#notification_settings,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ZeroTrustGatewaySettingsAntivirus {
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
                    r#enabled_download_phase: {
                        let field_value = match fields_map.get("enabled_download_phase") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled_download_phase' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled_upload_phase: {
                        let field_value = match fields_map.get("enabled_upload_phase") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled_upload_phase' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fail_closed: {
                        let field_value = match fields_map.get("fail_closed") {
                            Some(value) => value,
                            None => bail!("Missing field 'fail_closed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notification_settings: {
                        let field_value = match fields_map.get("notification_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'notification_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
