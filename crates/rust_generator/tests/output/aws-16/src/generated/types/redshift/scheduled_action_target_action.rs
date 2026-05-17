#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScheduledActionTargetAction {
    /// An action that runs a `PauseCluster` API operation. Documented below.
    #[builder(into)]
    #[serde(rename = "pauseCluster")]
    pub r#pause_cluster: Option<Box<super::super::types::redshift::ScheduledActionTargetActionPauseCluster>>,
    /// An action that runs a `ResizeCluster` API operation. Documented below.
    #[builder(into)]
    #[serde(rename = "resizeCluster")]
    pub r#resize_cluster: Option<Box<super::super::types::redshift::ScheduledActionTargetActionResizeCluster>>,
    /// An action that runs a `ResumeCluster` API operation. Documented below.
    #[builder(into)]
    #[serde(rename = "resumeCluster")]
    pub r#resume_cluster: Option<Box<super::super::types::redshift::ScheduledActionTargetActionResumeCluster>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScheduledActionTargetAction {
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
                    "pause_cluster",
                    &self.r#pause_cluster,
                ),
                to_pulumi_object_field(
                    "resize_cluster",
                    &self.r#resize_cluster,
                ),
                to_pulumi_object_field(
                    "resume_cluster",
                    &self.r#resume_cluster,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScheduledActionTargetAction {
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
                    r#pause_cluster: {
                        let field_value = match fields_map.get("pause_cluster") {
                            Some(value) => value,
                            None => bail!("Missing field 'pause_cluster' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resize_cluster: {
                        let field_value = match fields_map.get("resize_cluster") {
                            Some(value) => value,
                            None => bail!("Missing field 'resize_cluster' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resume_cluster: {
                        let field_value = match fields_map.get("resume_cluster") {
                            Some(value) => value,
                            None => bail!("Missing field 'resume_cluster' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
