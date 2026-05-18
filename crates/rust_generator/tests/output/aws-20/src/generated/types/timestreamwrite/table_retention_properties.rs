#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableRetentionProperties {
    /// The duration for which data must be stored in the magnetic store. Minimum value of 1. Maximum value of 73000.
    #[builder(into)]
    #[serde(rename = "magneticStoreRetentionPeriodInDays")]
    pub r#magnetic_store_retention_period_in_days: i32,
    /// The duration for which data must be stored in the memory store. Minimum value of 1. Maximum value of 8766.
    #[builder(into)]
    #[serde(rename = "memoryStoreRetentionPeriodInHours")]
    pub r#memory_store_retention_period_in_hours: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableRetentionProperties {
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
                    "magnetic_store_retention_period_in_days",
                    &self.r#magnetic_store_retention_period_in_days,
                ),
                to_pulumi_object_field(
                    "memory_store_retention_period_in_hours",
                    &self.r#memory_store_retention_period_in_hours,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableRetentionProperties {
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
                    r#magnetic_store_retention_period_in_days: {
                        let field_value = match fields_map.get("magnetic_store_retention_period_in_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'magnetic_store_retention_period_in_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_store_retention_period_in_hours: {
                        let field_value = match fields_map.get("memory_store_retention_period_in_hours") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_store_retention_period_in_hours' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
