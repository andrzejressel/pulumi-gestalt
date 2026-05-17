#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TaskTaskReportConfig {
    /// Specifies the type of task report you'd like. Valid values: `SUMMARY_ONLY` and `STANDARD`.
    #[builder(into)]
    #[serde(rename = "outputType")]
    pub r#output_type: Option<String>,
    /// Specifies whether you want your task report to include only what went wrong with your transfer or a list of what succeeded and didn't. Valid values: `ERRORS_ONLY` and `SUCCESSES_AND_ERRORS`.
    #[builder(into)]
    #[serde(rename = "reportLevel")]
    pub r#report_level: Option<String>,
    /// Configuration block containing the configuration of the reporting level for aspects of your task report. See `report_overrides` below.
    #[builder(into)]
    #[serde(rename = "reportOverrides")]
    pub r#report_overrides: Option<Box<super::super::types::datasync::TaskTaskReportConfigReportOverrides>>,
    /// Configuration block containing the configuration for the Amazon S3 bucket where DataSync uploads your task report. See `s3_destination` below.
    #[builder(into)]
    #[serde(rename = "s3Destination")]
    pub r#s_3_destination: Box<super::super::types::datasync::TaskTaskReportConfigS3Destination>,
    /// Specifies whether your task report includes the new version of each object transferred into an S3 bucket. This only applies if you enable versioning on your bucket. Keep in mind that setting this to INCLUDE can increase the duration of your task execution. Valid values: `INCLUDE` and `NONE`.
    #[builder(into)]
    #[serde(rename = "s3ObjectVersioning")]
    pub r#s_3_object_versioning: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TaskTaskReportConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "output_type",
                    &self.r#output_type,
                ),
                to_pulumi_object_field(
                    "report_level",
                    &self.r#report_level,
                ),
                to_pulumi_object_field(
                    "report_overrides",
                    &self.r#report_overrides,
                ),
                to_pulumi_object_field(
                    "s_3_destination",
                    &self.r#s_3_destination,
                ),
                to_pulumi_object_field(
                    "s_3_object_versioning",
                    &self.r#s_3_object_versioning,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TaskTaskReportConfig {
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
                    r#output_type: {
                        let field_value = match fields_map.get("output_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#report_level: {
                        let field_value = match fields_map.get("report_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'report_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#report_overrides: {
                        let field_value = match fields_map.get("report_overrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'report_overrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_destination: {
                        let field_value = match fields_map.get("s_3_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_object_versioning: {
                        let field_value = match fields_map.get("s_3_object_versioning") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_object_versioning' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
