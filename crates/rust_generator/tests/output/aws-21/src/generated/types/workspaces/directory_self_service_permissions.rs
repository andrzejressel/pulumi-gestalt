#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DirectorySelfServicePermissions {
    /// Whether WorkSpaces directory users can change the compute type (bundle) for their workspace. Default `false`.
    #[builder(into)]
    #[serde(rename = "changeComputeType")]
    pub r#change_compute_type: Option<bool>,
    /// Whether WorkSpaces directory users can increase the volume size of the drives on their workspace. Default `false`.
    #[builder(into)]
    #[serde(rename = "increaseVolumeSize")]
    pub r#increase_volume_size: Option<bool>,
    /// Whether WorkSpaces directory users can rebuild the operating system of a workspace to its original state. Default `false`.
    #[builder(into)]
    #[serde(rename = "rebuildWorkspace")]
    pub r#rebuild_workspace: Option<bool>,
    /// Whether WorkSpaces directory users can restart their workspace. Default `true`.
    #[builder(into)]
    #[serde(rename = "restartWorkspace")]
    pub r#restart_workspace: Option<bool>,
    /// Whether WorkSpaces directory users can switch the running mode of their workspace. Default `false`.
    #[builder(into)]
    #[serde(rename = "switchRunningMode")]
    pub r#switch_running_mode: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DirectorySelfServicePermissions {
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
                    "change_compute_type",
                    &self.r#change_compute_type,
                ),
                to_pulumi_object_field(
                    "increase_volume_size",
                    &self.r#increase_volume_size,
                ),
                to_pulumi_object_field(
                    "rebuild_workspace",
                    &self.r#rebuild_workspace,
                ),
                to_pulumi_object_field(
                    "restart_workspace",
                    &self.r#restart_workspace,
                ),
                to_pulumi_object_field(
                    "switch_running_mode",
                    &self.r#switch_running_mode,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DirectorySelfServicePermissions {
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
                    r#change_compute_type: {
                        let field_value = match fields_map.get("change_compute_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'change_compute_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#increase_volume_size: {
                        let field_value = match fields_map.get("increase_volume_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'increase_volume_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rebuild_workspace: {
                        let field_value = match fields_map.get("rebuild_workspace") {
                            Some(value) => value,
                            None => bail!("Missing field 'rebuild_workspace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restart_workspace: {
                        let field_value = match fields_map.get("restart_workspace") {
                            Some(value) => value,
                            None => bail!("Missing field 'restart_workspace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#switch_running_mode: {
                        let field_value = match fields_map.get("switch_running_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'switch_running_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
