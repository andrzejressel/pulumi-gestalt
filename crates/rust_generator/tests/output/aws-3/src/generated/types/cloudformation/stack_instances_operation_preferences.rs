#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StackInstancesOperationPreferences {
    /// How the concurrency level behaves during the operation execution. Valid values are `STRICT_FAILURE_TOLERANCE` and `SOFT_FAILURE_TOLERANCE`.
    #[builder(into)]
    pub r#concurrency_mode: Option<String>,
    /// Number of accounts, per region, for which this operation can fail before CloudFormation stops the operation in that region.
    #[builder(into)]
    pub r#failure_tolerance_count: Option<i32>,
    /// Percentage of accounts, per region, for which this stack operation can fail before CloudFormation stops the operation in that region.
    #[builder(into)]
    pub r#failure_tolerance_percentage: Option<i32>,
    /// Maximum number of accounts in which to perform this operation at one time.
    #[builder(into)]
    pub r#max_concurrent_count: Option<i32>,
    /// Maximum percentage of accounts in which to perform this operation at one time.
    #[builder(into)]
    pub r#max_concurrent_percentage: Option<i32>,
    /// Concurrency type of deploying stack sets operations in regions, could be in parallel or one region at a time. Valid values are `SEQUENTIAL` and `PARALLEL`.
    #[builder(into)]
    pub r#region_concurrency_type: Option<String>,
    /// Order of the regions where you want to perform the stack operation.
    #[builder(into)]
    pub r#region_orders: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StackInstancesOperationPreferences {
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
                    "concurrencyMode",
                    &self.r#concurrency_mode,
                ),
                to_pulumi_object_field(
                    "failureToleranceCount",
                    &self.r#failure_tolerance_count,
                ),
                to_pulumi_object_field(
                    "failureTolerancePercentage",
                    &self.r#failure_tolerance_percentage,
                ),
                to_pulumi_object_field(
                    "maxConcurrentCount",
                    &self.r#max_concurrent_count,
                ),
                to_pulumi_object_field(
                    "maxConcurrentPercentage",
                    &self.r#max_concurrent_percentage,
                ),
                to_pulumi_object_field(
                    "regionConcurrencyType",
                    &self.r#region_concurrency_type,
                ),
                to_pulumi_object_field(
                    "regionOrders",
                    &self.r#region_orders,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StackInstancesOperationPreferences {
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
                    r#concurrency_mode: {
                        let field_value = match fields_map.get("concurrencyMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'concurrencyMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failure_tolerance_count: {
                        let field_value = match fields_map.get("failureToleranceCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'failureToleranceCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#failure_tolerance_percentage: {
                        let field_value = match fields_map.get("failureTolerancePercentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'failureTolerancePercentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_concurrent_count: {
                        let field_value = match fields_map.get("maxConcurrentCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxConcurrentCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_concurrent_percentage: {
                        let field_value = match fields_map.get("maxConcurrentPercentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxConcurrentPercentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#region_concurrency_type: {
                        let field_value = match fields_map.get("regionConcurrencyType") {
                            Some(value) => value,
                            None => bail!("Missing field 'regionConcurrencyType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#region_orders: {
                        let field_value = match fields_map.get("regionOrders") {
                            Some(value) => value,
                            None => bail!("Missing field 'regionOrders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
