#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceMaintenancePolicy {
    /// (Output)
    /// Output only. The time when the policy was created.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond
    /// resolution and up to nine fractional digits
    #[builder(into)]
    pub r#create_time: Option<String>,
    /// Optional. Description of what this policy is for.
    /// Create/Update methods return INVALID_ARGUMENT if the
    /// length is greater than 512.
    #[builder(into)]
    pub r#description: Option<String>,
    /// (Output)
    /// Output only. The time when the policy was updated.
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond
    /// resolution and up to nine fractional digits.
    #[builder(into)]
    pub r#update_time: Option<String>,
    /// Required. Maintenance window that is applied to resources covered by this policy.
    /// Minimum 1. For the current version, the maximum number of weekly_maintenance_windows
    /// is expected to be one.
    /// Structure is documented below.
    #[builder(into)]
    pub r#weekly_maintenance_windows: Vec<super::super::types::memcache::InstanceMaintenancePolicyWeeklyMaintenanceWindow>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceMaintenancePolicy {
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
                    "createTime",
                    &self.r#create_time,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "updateTime",
                    &self.r#update_time,
                ),
                to_pulumi_object_field(
                    "weeklyMaintenanceWindows",
                    &self.r#weekly_maintenance_windows,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceMaintenancePolicy {
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
                    r#create_time: {
                        let field_value = match fields_map.get("createTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'createTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#update_time: {
                        let field_value = match fields_map.get("updateTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'updateTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weekly_maintenance_windows: {
                        let field_value = match fields_map.get("weeklyMaintenanceWindows") {
                            Some(value) => value,
                            None => bail!("Missing field 'weeklyMaintenanceWindows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
