#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkspaceWorkspaceProperties {
    /// The compute type. For more information, see [Amazon WorkSpaces Bundles](http://aws.amazon.com/workspaces/details/#Amazon_WorkSpaces_Bundles). Valid values are `VALUE`, `STANDARD`, `PERFORMANCE`, `POWER`, `GRAPHICS`, `POWERPRO`, `GRAPHICSPRO`, `GRAPHICS_G4DN`, and `GRAPHICSPRO_G4DN`.
    #[builder(into)]
    #[serde(rename = "computeTypeName")]
    pub r#compute_type_name: Option<String>,
    /// The size of the root volume.
    #[builder(into)]
    #[serde(rename = "rootVolumeSizeGib")]
    pub r#root_volume_size_gib: Option<i32>,
    /// The running mode. For more information, see [Manage the WorkSpace Running Mode](https://docs.aws.amazon.com/workspaces/latest/adminguide/running-mode.html). Valid values are `AUTO_STOP` and `ALWAYS_ON`.
    #[builder(into)]
    #[serde(rename = "runningMode")]
    pub r#running_mode: Option<String>,
    /// The time after a user logs off when WorkSpaces are automatically stopped. Configured in 60-minute intervals.
    #[builder(into)]
    #[serde(rename = "runningModeAutoStopTimeoutInMinutes")]
    pub r#running_mode_auto_stop_timeout_in_minutes: Option<i32>,
    /// The size of the user storage.
    #[builder(into)]
    #[serde(rename = "userVolumeSizeGib")]
    pub r#user_volume_size_gib: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkspaceWorkspaceProperties {
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
                    "compute_type_name",
                    &self.r#compute_type_name,
                ),
                to_pulumi_object_field(
                    "root_volume_size_gib",
                    &self.r#root_volume_size_gib,
                ),
                to_pulumi_object_field(
                    "running_mode",
                    &self.r#running_mode,
                ),
                to_pulumi_object_field(
                    "running_mode_auto_stop_timeout_in_minutes",
                    &self.r#running_mode_auto_stop_timeout_in_minutes,
                ),
                to_pulumi_object_field(
                    "user_volume_size_gib",
                    &self.r#user_volume_size_gib,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkspaceWorkspaceProperties {
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
                    r#compute_type_name: {
                        let field_value = match fields_map.get("compute_type_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'compute_type_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_volume_size_gib: {
                        let field_value = match fields_map.get("root_volume_size_gib") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_volume_size_gib' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#running_mode: {
                        let field_value = match fields_map.get("running_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'running_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#running_mode_auto_stop_timeout_in_minutes: {
                        let field_value = match fields_map.get("running_mode_auto_stop_timeout_in_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'running_mode_auto_stop_timeout_in_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_volume_size_gib: {
                        let field_value = match fields_map.get("user_volume_size_gib") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_volume_size_gib' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
