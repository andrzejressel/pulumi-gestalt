#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StackSetInstanceOperationPreferences {
    /// Specifies how the concurrency level behaves during the operation execution. Valid values are `STRICT_FAILURE_TOLERANCE` and `SOFT_FAILURE_TOLERANCE`.
    #[builder(into)]
    #[serde(rename = "concurrencyMode")]
    pub r#concurrency_mode: Option<String>,
    /// Number of accounts, per Region, for which this operation can fail before AWS CloudFormation stops the operation in that Region.
    #[builder(into)]
    #[serde(rename = "failureToleranceCount")]
    pub r#failure_tolerance_count: Option<i32>,
    /// Percentage of accounts, per Region, for which this stack operation can fail before AWS CloudFormation stops the operation in that Region.
    #[builder(into)]
    #[serde(rename = "failureTolerancePercentage")]
    pub r#failure_tolerance_percentage: Option<i32>,
    /// Maximum number of accounts in which to perform this operation at one time.
    #[builder(into)]
    #[serde(rename = "maxConcurrentCount")]
    pub r#max_concurrent_count: Option<i32>,
    /// Maximum percentage of accounts in which to perform this operation at one time.
    #[builder(into)]
    #[serde(rename = "maxConcurrentPercentage")]
    pub r#max_concurrent_percentage: Option<i32>,
    /// Concurrency type of deploying StackSets operations in Regions, could be in parallel or one Region at a time. Valid values are `SEQUENTIAL` and `PARALLEL`.
    #[builder(into)]
    #[serde(rename = "regionConcurrencyType")]
    pub r#region_concurrency_type: Option<String>,
    /// Order of the Regions in where you want to perform the stack operation.
    #[builder(into)]
    #[serde(rename = "regionOrders")]
    pub r#region_orders: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StackSetInstanceOperationPreferences {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("concurrency_mode".to_string(), self.r#concurrency_mode.to_pulumi_value().await);
            map.insert("failure_tolerance_count".to_string(), self.r#failure_tolerance_count.to_pulumi_value().await);
            map.insert("failure_tolerance_percentage".to_string(), self.r#failure_tolerance_percentage.to_pulumi_value().await);
            map.insert("max_concurrent_count".to_string(), self.r#max_concurrent_count.to_pulumi_value().await);
            map.insert("max_concurrent_percentage".to_string(), self.r#max_concurrent_percentage.to_pulumi_value().await);
            map.insert("region_concurrency_type".to_string(), self.r#region_concurrency_type.to_pulumi_value().await);
            map.insert("region_orders".to_string(), self.r#region_orders.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StackSetInstanceOperationPreferences {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#concurrency_mode: {
                        let field_value = match fields_map.get("concurrency_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'concurrency_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#failure_tolerance_count: {
                        let field_value = match fields_map.get("failure_tolerance_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'failure_tolerance_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#failure_tolerance_percentage: {
                        let field_value = match fields_map.get("failure_tolerance_percentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'failure_tolerance_percentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#max_concurrent_count: {
                        let field_value = match fields_map.get("max_concurrent_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_concurrent_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#max_concurrent_percentage: {
                        let field_value = match fields_map.get("max_concurrent_percentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_concurrent_percentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#region_concurrency_type: {
                        let field_value = match fields_map.get("region_concurrency_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'region_concurrency_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#region_orders: {
                        let field_value = match fields_map.get("region_orders") {
                            Some(value) => value,
                            None => bail!("Missing field 'region_orders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
