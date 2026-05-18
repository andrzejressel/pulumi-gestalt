#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VolumeSnapshotPolicyWeeklySchedule {
    /// Set the day or days of the week to make a snapshot. Accepts a comma separated days of the week. Defaults to 'Sunday'.
    #[builder(into)]
    #[serde(rename = "day")]
    pub r#day: Option<String>,
    /// Set the hour to create the snapshot (0-23), defaults to midnight (0).
    #[builder(into)]
    #[serde(rename = "hour")]
    pub r#hour: Option<i32>,
    /// Set the minute of the hour to create the snapshot (0-59), defaults to the top of the hour (0).
    #[builder(into)]
    #[serde(rename = "minute")]
    pub r#minute: Option<i32>,
    /// The maximum number of snapshots to keep for the weekly schedule.
    #[builder(into)]
    #[serde(rename = "snapshotsToKeep")]
    pub r#snapshots_to_keep: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VolumeSnapshotPolicyWeeklySchedule {
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
                    "day",
                    &self.r#day,
                ),
                to_pulumi_object_field(
                    "hour",
                    &self.r#hour,
                ),
                to_pulumi_object_field(
                    "minute",
                    &self.r#minute,
                ),
                to_pulumi_object_field(
                    "snapshots_to_keep",
                    &self.r#snapshots_to_keep,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VolumeSnapshotPolicyWeeklySchedule {
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
                    r#day: {
                        let field_value = match fields_map.get("day") {
                            Some(value) => value,
                            None => bail!("Missing field 'day' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hour: {
                        let field_value = match fields_map.get("hour") {
                            Some(value) => value,
                            None => bail!("Missing field 'hour' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minute: {
                        let field_value = match fields_map.get("minute") {
                            Some(value) => value,
                            None => bail!("Missing field 'minute' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshots_to_keep: {
                        let field_value = match fields_map.get("snapshots_to_keep") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshots_to_keep' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
