#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineNotifications {
    /// The topic ARN for the Amazon SNS topic that you want to notify when Elastic Transcoder has finished processing a job in this pipeline.
    #[builder(into)]
    #[serde(rename = "completed")]
    pub r#completed: Option<String>,
    /// The topic ARN for the Amazon SNS topic that you want to notify when Elastic Transcoder encounters an error condition while processing a job in this pipeline.
    #[builder(into)]
    #[serde(rename = "error")]
    pub r#error: Option<String>,
    /// The topic ARN for the Amazon Simple Notification Service (Amazon SNS) topic that you want to notify when Elastic Transcoder has started to process a job in this pipeline.
    #[builder(into)]
    #[serde(rename = "progressing")]
    pub r#progressing: Option<String>,
    /// The topic ARN for the Amazon SNS topic that you want to notify when Elastic Transcoder encounters a warning condition while processing a job in this pipeline.
    /// 
    /// The `thumbnail_config` object specifies information about the Amazon S3 bucket in
    /// which you want Elastic Transcoder to save thumbnail files: which bucket to use,
    /// which users you want to have access to the files, the type of access you want
    /// users to have, and the storage class that you want to assign to the files. If
    /// you specify values for `content_config`, you must also specify values for
    /// `thumbnail_config` even if you don't want to create thumbnails. (You control
    /// whether to create thumbnails when you create a job. For more information, see
    /// ThumbnailPattern in the topic Create Job.) If you specify values for
    /// `content_config` and `thumbnail_config`, omit the OutputBucket object.
    #[builder(into)]
    #[serde(rename = "warning")]
    pub r#warning: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipelineNotifications {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "completed",
                    &self.r#completed,
                ),
                to_pulumi_object_field(
                    "error",
                    &self.r#error,
                ),
                to_pulumi_object_field(
                    "progressing",
                    &self.r#progressing,
                ),
                to_pulumi_object_field(
                    "warning",
                    &self.r#warning,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipelineNotifications {
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
                    r#completed: {
                        let field_value = match fields_map.get("completed") {
                            Some(value) => value,
                            None => bail!("Missing field 'completed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#error: {
                        let field_value = match fields_map.get("error") {
                            Some(value) => value,
                            None => bail!("Missing field 'error' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#progressing: {
                        let field_value = match fields_map.get("progressing") {
                            Some(value) => value,
                            None => bail!("Missing field 'progressing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#warning: {
                        let field_value = match fields_map.get("warning") {
                            Some(value) => value,
                            None => bail!("Missing field 'warning' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
