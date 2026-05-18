#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryReleaseConfigRecentScheduledReleaseRecord {
    /// (Output)
    /// The name of the created compilation result, if one was successfully created. Must be in the format projects/*/locations/*/repositories/*/compilationResults/*.
    #[builder(into)]
    #[serde(rename = "compilationResult")]
    pub r#compilation_result: Option<String>,
    /// (Output)
    /// The error status encountered upon this attempt to create the compilation result, if the attempt was unsuccessful.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "errorStatuses")]
    pub r#error_statuses: Option<Vec<super::super::types::dataform::RepositoryReleaseConfigRecentScheduledReleaseRecordErrorStatus>>,
    /// (Output)
    /// The timestamp of this release attempt.
    #[builder(into)]
    #[serde(rename = "releaseTime")]
    pub r#release_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RepositoryReleaseConfigRecentScheduledReleaseRecord {
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
                    "compilation_result",
                    &self.r#compilation_result,
                ),
                to_pulumi_object_field(
                    "error_statuses",
                    &self.r#error_statuses,
                ),
                to_pulumi_object_field(
                    "release_time",
                    &self.r#release_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RepositoryReleaseConfigRecentScheduledReleaseRecord {
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
                    r#compilation_result: {
                        let field_value = match fields_map.get("compilation_result") {
                            Some(value) => value,
                            None => bail!("Missing field 'compilation_result' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#error_statuses: {
                        let field_value = match fields_map.get("error_statuses") {
                            Some(value) => value,
                            None => bail!("Missing field 'error_statuses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#release_time: {
                        let field_value = match fields_map.get("release_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'release_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
