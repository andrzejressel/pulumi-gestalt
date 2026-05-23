#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TeamsAccountAntivirus {
    /// Scan on file download.
    #[builder(into)]
    pub r#enabled_download_phase: bool,
    /// Scan on file upload.
    #[builder(into)]
    pub r#enabled_upload_phase: bool,
    /// Block requests for files that cannot be scanned.
    #[builder(into)]
    pub r#fail_closed: bool,
    /// Set notifications for antivirus.
    #[builder(into)]
    pub r#notification_settings: Option<Box<super::types::TeamsAccountAntivirusNotificationSettings>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TeamsAccountAntivirus {
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
                    "enabledDownloadPhase",
                    &self.r#enabled_download_phase,
                ),
                to_pulumi_object_field(
                    "enabledUploadPhase",
                    &self.r#enabled_upload_phase,
                ),
                to_pulumi_object_field(
                    "failClosed",
                    &self.r#fail_closed,
                ),
                to_pulumi_object_field(
                    "notificationSettings",
                    &self.r#notification_settings,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TeamsAccountAntivirus {
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
                        let field_value = match fields_map.get("enabledDownloadPhase") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabledDownloadPhase' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled_upload_phase: {
                        let field_value = match fields_map.get("enabledUploadPhase") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabledUploadPhase' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fail_closed: {
                        let field_value = match fields_map.get("failClosed") {
                            Some(value) => value,
                            None => bail!("Missing field 'failClosed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notification_settings: {
                        let field_value = match fields_map.get("notificationSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'notificationSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
