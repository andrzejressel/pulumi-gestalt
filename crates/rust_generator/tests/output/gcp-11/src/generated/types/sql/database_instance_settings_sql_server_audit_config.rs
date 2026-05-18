#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DatabaseInstanceSettingsSqlServerAuditConfig {
    /// The name of the destination bucket (e.g., gs://mybucket).
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Option<String>,
    /// How long to keep generated audit files. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "retentionInterval")]
    pub r#retention_interval: Option<String>,
    /// How often to upload generated audit files. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "uploadInterval")]
    pub r#upload_interval: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DatabaseInstanceSettingsSqlServerAuditConfig {
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
                    "bucket",
                    &self.r#bucket,
                ),
                to_pulumi_object_field(
                    "retention_interval",
                    &self.r#retention_interval,
                ),
                to_pulumi_object_field(
                    "upload_interval",
                    &self.r#upload_interval,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DatabaseInstanceSettingsSqlServerAuditConfig {
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
                    r#bucket: {
                        let field_value = match fields_map.get("bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_interval: {
                        let field_value = match fields_map.get("retention_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'retention_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upload_interval: {
                        let field_value = match fields_map.get("upload_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'upload_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
