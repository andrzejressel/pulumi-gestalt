#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterMaintenanceEvent {
    /// (Output)
    /// The time when the maintenance event request was created.
    #[builder(into)]
    pub r#create_time: Option<String>,
    /// (Output)
    /// The time when the maintenance event ended, either successfully or not. If
    /// the maintenance event is split into multiple maintenance windows,
    /// end_time is only updated when the whole flow ends.
    #[builder(into)]
    pub r#end_time: Option<String>,
    /// (Output)
    /// The operation for running the maintenance event. Specified in the format
    /// projects/*/locations/*/operations/*. If the maintenance event is split
    /// into multiple operations (e.g. due to maintenance windows), the latest
    /// one is recorded.
    #[builder(into)]
    pub r#operation: Option<String>,
    /// (Output)
    /// The schedule of the maintenance event.
    #[builder(into)]
    pub r#schedule: Option<String>,
    /// (Output)
    /// The time when the maintenance event started.
    #[builder(into)]
    pub r#start_time: Option<String>,
    /// (Output)
    /// Indicates the maintenance event state.
    #[builder(into)]
    pub r#state: Option<String>,
    /// (Output)
    /// The target version of the cluster.
    #[builder(into)]
    pub r#target_version: Option<String>,
    /// (Output)
    /// Indicates the maintenance event type.
    #[builder(into)]
    pub r#type_: Option<String>,
    /// (Output)
    /// The time when the maintenance event message was updated.
    #[builder(into)]
    pub r#update_time: Option<String>,
    /// (Output)
    /// UUID of the maintenance event.
    #[builder(into)]
    pub r#uuid: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterMaintenanceEvent {
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
                    "endTime",
                    &self.r#end_time,
                ),
                to_pulumi_object_field(
                    "operation",
                    &self.r#operation,
                ),
                to_pulumi_object_field(
                    "schedule",
                    &self.r#schedule,
                ),
                to_pulumi_object_field(
                    "startTime",
                    &self.r#start_time,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
                to_pulumi_object_field(
                    "targetVersion",
                    &self.r#target_version,
                ),
                to_pulumi_object_field(
                    "type",
                    &self.r#type_,
                ),
                to_pulumi_object_field(
                    "updateTime",
                    &self.r#update_time,
                ),
                to_pulumi_object_field(
                    "uuid",
                    &self.r#uuid,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterMaintenanceEvent {
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
                    r#end_time: {
                        let field_value = match fields_map.get("endTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'endTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operation: {
                        let field_value = match fields_map.get("operation") {
                            Some(value) => value,
                            None => bail!("Missing field 'operation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schedule: {
                        let field_value = match fields_map.get("schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_time: {
                        let field_value = match fields_map.get("startTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'startTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_version: {
                        let field_value = match fields_map.get("targetVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type") {
                            Some(value) => value,
                            None => bail!("Missing field 'type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#uuid: {
                        let field_value = match fields_map.get("uuid") {
                            Some(value) => value,
                            None => bail!("Missing field 'uuid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
