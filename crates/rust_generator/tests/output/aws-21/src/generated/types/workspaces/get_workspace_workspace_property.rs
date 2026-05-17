#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetWorkspaceWorkspaceProperty {
    /// Compute type. For more information, see [Amazon WorkSpaces Bundles](http://aws.amazon.com/workspaces/details/#Amazon_WorkSpaces_Bundles). Valid values are `VALUE`, `STANDARD`, `PERFORMANCE`, `POWER`, `GRAPHICS`, `POWERPRO` and `GRAPHICSPRO`.
    #[builder(into)]
    #[serde(rename = "computeTypeName")]
    pub r#compute_type_name: String,
    /// Size of the root volume.
    #[builder(into)]
    #[serde(rename = "rootVolumeSizeGib")]
    pub r#root_volume_size_gib: i32,
    /// Running mode. For more information, see [Manage the WorkSpace Running Mode](https://docs.aws.amazon.com/workspaces/latest/adminguide/running-mode.html). Valid values are `AUTO_STOP` and `ALWAYS_ON`.
    #[builder(into)]
    #[serde(rename = "runningMode")]
    pub r#running_mode: String,
    /// Time after a user logs off when WorkSpaces are automatically stopped. Configured in 60-minute intervals.
    #[builder(into)]
    #[serde(rename = "runningModeAutoStopTimeoutInMinutes")]
    pub r#running_mode_auto_stop_timeout_in_minutes: i32,
    /// Size of the user storage.
    #[builder(into)]
    #[serde(rename = "userVolumeSizeGib")]
    pub r#user_volume_size_gib: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetWorkspaceWorkspaceProperty {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "compute_type_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#compute_type_name,
                )
                .await,
            );
            map.insert(
                "root_volume_size_gib".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#root_volume_size_gib,
                )
                .await,
            );
            map.insert(
                "running_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#running_mode,
                )
                .await,
            );
            map.insert(
                "running_mode_auto_stop_timeout_in_minutes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#running_mode_auto_stop_timeout_in_minutes,
                )
                .await,
            );
            map.insert(
                "user_volume_size_gib".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_volume_size_gib,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetWorkspaceWorkspaceProperty {
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
