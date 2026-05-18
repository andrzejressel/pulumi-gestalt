#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterContinuousBackupInfo {
    /// (Output)
    /// The earliest restorable time that can be restored to. Output only field.
    #[builder(into)]
    #[serde(rename = "earliestRestorableTime")]
    pub r#earliest_restorable_time: Option<String>,
    /// (Output)
    /// When ContinuousBackup was most recently enabled. Set to null if ContinuousBackup is not enabled.
    #[builder(into)]
    #[serde(rename = "enabledTime")]
    pub r#enabled_time: Option<String>,
    /// (Output)
    /// Output only. The encryption information for the WALs and backups required for ContinuousBackup.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "encryptionInfos")]
    pub r#encryption_infos: Option<Vec<super::super::types::alloydb::ClusterContinuousBackupInfoEncryptionInfo>>,
    /// (Output)
    /// Days of the week on which a continuous backup is taken. Output only field. Ignored if passed into the request.
    #[builder(into)]
    #[serde(rename = "schedules")]
    pub r#schedules: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterContinuousBackupInfo {
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
                    "earliest_restorable_time",
                    &self.r#earliest_restorable_time,
                ),
                to_pulumi_object_field(
                    "enabled_time",
                    &self.r#enabled_time,
                ),
                to_pulumi_object_field(
                    "encryption_infos",
                    &self.r#encryption_infos,
                ),
                to_pulumi_object_field(
                    "schedules",
                    &self.r#schedules,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterContinuousBackupInfo {
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
                    r#earliest_restorable_time: {
                        let field_value = match fields_map.get("earliest_restorable_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'earliest_restorable_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled_time: {
                        let field_value = match fields_map.get("enabled_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_infos: {
                        let field_value = match fields_map.get("encryption_infos") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_infos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schedules: {
                        let field_value = match fields_map.get("schedules") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
