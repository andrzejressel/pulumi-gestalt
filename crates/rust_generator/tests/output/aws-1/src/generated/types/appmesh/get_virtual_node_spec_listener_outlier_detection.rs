#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualNodeSpecListenerOutlierDetection {
    #[builder(into)]
    pub r#base_ejection_durations: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerOutlierDetectionBaseEjectionDuration>,
    #[builder(into)]
    pub r#intervals: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerOutlierDetectionInterval>,
    #[builder(into)]
    pub r#max_ejection_percent: i32,
    #[builder(into)]
    pub r#max_server_errors: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualNodeSpecListenerOutlierDetection {
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
                    "baseEjectionDurations",
                    &self.r#base_ejection_durations,
                ),
                to_pulumi_object_field(
                    "intervals",
                    &self.r#intervals,
                ),
                to_pulumi_object_field(
                    "maxEjectionPercent",
                    &self.r#max_ejection_percent,
                ),
                to_pulumi_object_field(
                    "maxServerErrors",
                    &self.r#max_server_errors,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualNodeSpecListenerOutlierDetection {
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
                    r#base_ejection_durations: {
                        let field_value = match fields_map.get("baseEjectionDurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'baseEjectionDurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#intervals: {
                        let field_value = match fields_map.get("intervals") {
                            Some(value) => value,
                            None => bail!("Missing field 'intervals' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_ejection_percent: {
                        let field_value = match fields_map.get("maxEjectionPercent") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxEjectionPercent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_server_errors: {
                        let field_value = match fields_map.get("maxServerErrors") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxServerErrors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
