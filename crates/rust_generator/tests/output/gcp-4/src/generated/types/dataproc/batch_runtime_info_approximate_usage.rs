#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BatchRuntimeInfoApproximateUsage {
    /// (Output)
    /// Accelerator type being used, if any.
    #[builder(into)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: Option<String>,
    /// (Output)
    /// Accelerator usage in (milliAccelerator x seconds)
    #[builder(into)]
    #[serde(rename = "milliAcceleratorSeconds")]
    pub r#milli_accelerator_seconds: Option<String>,
    /// (Output)
    /// DCU (Dataproc Compute Units) usage in (milliDCU x seconds)
    #[builder(into)]
    #[serde(rename = "milliDcuSeconds")]
    pub r#milli_dcu_seconds: Option<String>,
    /// (Output)
    /// Shuffle storage usage in (GB x seconds)
    #[builder(into)]
    #[serde(rename = "shuffleStorageGbSeconds")]
    pub r#shuffle_storage_gb_seconds: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BatchRuntimeInfoApproximateUsage {
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
                    "accelerator_type",
                    &self.r#accelerator_type,
                ),
                to_pulumi_object_field(
                    "milli_accelerator_seconds",
                    &self.r#milli_accelerator_seconds,
                ),
                to_pulumi_object_field(
                    "milli_dcu_seconds",
                    &self.r#milli_dcu_seconds,
                ),
                to_pulumi_object_field(
                    "shuffle_storage_gb_seconds",
                    &self.r#shuffle_storage_gb_seconds,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BatchRuntimeInfoApproximateUsage {
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
                    r#accelerator_type: {
                        let field_value = match fields_map.get("accelerator_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerator_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#milli_accelerator_seconds: {
                        let field_value = match fields_map.get("milli_accelerator_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'milli_accelerator_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#milli_dcu_seconds: {
                        let field_value = match fields_map.get("milli_dcu_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'milli_dcu_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shuffle_storage_gb_seconds: {
                        let field_value = match fields_map.get("shuffle_storage_gb_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'shuffle_storage_gb_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
