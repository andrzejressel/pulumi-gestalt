#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEnvironmentConfigWorkloadsConfigWorker {
    /// CPU request and limit for a single Airflow worker replica.
    #[builder(into)]
    pub r#cpu: f64,
    /// Maximum number of workers for autoscaling.
    #[builder(into)]
    pub r#max_count: i32,
    /// Memory (GB) request and limit for a single Airflow worker replica.
    #[builder(into)]
    pub r#memory_gb: f64,
    /// Minimum number of workers for autoscaling.
    #[builder(into)]
    pub r#min_count: i32,
    /// Storage (GB) request and limit for a single Airflow worker replica.
    #[builder(into)]
    pub r#storage_gb: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetEnvironmentConfigWorkloadsConfigWorker {
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
                    "cpu",
                    &self.r#cpu,
                ),
                to_pulumi_object_field(
                    "maxCount",
                    &self.r#max_count,
                ),
                to_pulumi_object_field(
                    "memoryGb",
                    &self.r#memory_gb,
                ),
                to_pulumi_object_field(
                    "minCount",
                    &self.r#min_count,
                ),
                to_pulumi_object_field(
                    "storageGb",
                    &self.r#storage_gb,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetEnvironmentConfigWorkloadsConfigWorker {
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
                    r#cpu: {
                        let field_value = match fields_map.get("cpu") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_count: {
                        let field_value = match fields_map.get("maxCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_gb: {
                        let field_value = match fields_map.get("memoryGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'memoryGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_count: {
                        let field_value = match fields_map.get("minCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'minCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_gb: {
                        let field_value = match fields_map.get("storageGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
