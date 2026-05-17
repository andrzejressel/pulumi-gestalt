#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustGatewayPolicyRuleSettingsBisoAdminControls {
    /// Disable clipboard redirection.
    #[builder(into)]
    #[serde(rename = "disableClipboardRedirection")]
    pub r#disable_clipboard_redirection: Option<bool>,
    /// Disable copy-paste.
    #[builder(into)]
    #[serde(rename = "disableCopyPaste")]
    pub r#disable_copy_paste: Option<bool>,
    /// Disable download.
    #[builder(into)]
    #[serde(rename = "disableDownload")]
    pub r#disable_download: Option<bool>,
    /// Disable keyboard usage.
    #[builder(into)]
    #[serde(rename = "disableKeyboard")]
    pub r#disable_keyboard: Option<bool>,
    /// Disable printing.
    #[builder(into)]
    #[serde(rename = "disablePrinting")]
    pub r#disable_printing: Option<bool>,
    /// Disable upload.
    #[builder(into)]
    #[serde(rename = "disableUpload")]
    pub r#disable_upload: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ZeroTrustGatewayPolicyRuleSettingsBisoAdminControls {
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
                "disable_clipboard_redirection".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_clipboard_redirection,
                )
                .await,
            );
            map.insert(
                "disable_copy_paste".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_copy_paste,
                )
                .await,
            );
            map.insert(
                "disable_download".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_download,
                )
                .await,
            );
            map.insert(
                "disable_keyboard".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_keyboard,
                )
                .await,
            );
            map.insert(
                "disable_printing".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_printing,
                )
                .await,
            );
            map.insert(
                "disable_upload".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_upload,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ZeroTrustGatewayPolicyRuleSettingsBisoAdminControls {
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
                    r#disable_clipboard_redirection: {
                        let field_value = match fields_map.get("disable_clipboard_redirection") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_clipboard_redirection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_copy_paste: {
                        let field_value = match fields_map.get("disable_copy_paste") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_copy_paste' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_download: {
                        let field_value = match fields_map.get("disable_download") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_download' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_keyboard: {
                        let field_value = match fields_map.get("disable_keyboard") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_keyboard' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_printing: {
                        let field_value = match fields_map.get("disable_printing") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_printing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_upload: {
                        let field_value = match fields_map.get("disable_upload") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_upload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
