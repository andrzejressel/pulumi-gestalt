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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "disableClipboardRedirection",
                    &self.r#disable_clipboard_redirection,
                ),
                to_pulumi_object_field(
                    "disableCopyPaste",
                    &self.r#disable_copy_paste,
                ),
                to_pulumi_object_field(
                    "disableDownload",
                    &self.r#disable_download,
                ),
                to_pulumi_object_field(
                    "disableKeyboard",
                    &self.r#disable_keyboard,
                ),
                to_pulumi_object_field(
                    "disablePrinting",
                    &self.r#disable_printing,
                ),
                to_pulumi_object_field(
                    "disableUpload",
                    &self.r#disable_upload,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("disableClipboardRedirection") {
                            Some(value) => value,
                            None => bail!("Missing field 'disableClipboardRedirection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_copy_paste: {
                        let field_value = match fields_map.get("disableCopyPaste") {
                            Some(value) => value,
                            None => bail!("Missing field 'disableCopyPaste' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_download: {
                        let field_value = match fields_map.get("disableDownload") {
                            Some(value) => value,
                            None => bail!("Missing field 'disableDownload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_keyboard: {
                        let field_value = match fields_map.get("disableKeyboard") {
                            Some(value) => value,
                            None => bail!("Missing field 'disableKeyboard' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_printing: {
                        let field_value = match fields_map.get("disablePrinting") {
                            Some(value) => value,
                            None => bail!("Missing field 'disablePrinting' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_upload: {
                        let field_value = match fields_map.get("disableUpload") {
                            Some(value) => value,
                            None => bail!("Missing field 'disableUpload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
