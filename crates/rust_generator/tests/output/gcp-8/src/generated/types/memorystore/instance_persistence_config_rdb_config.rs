#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstancePersistenceConfigRdbConfig {
    /// Optional. Period between RDB snapshots.
    /// Possible values:
    /// ONE_HOUR
    /// SIX_HOURS
    /// TWELVE_HOURS
    /// TWENTY_FOUR_HOURS
    #[builder(into)]
    #[serde(rename = "rdbSnapshotPeriod")]
    pub r#rdb_snapshot_period: Option<String>,
    /// Optional. Time that the first snapshot was/will be attempted, and to which future
    /// snapshots will be aligned. If not provided, the current time will be
    /// used.
    #[builder(into)]
    #[serde(rename = "rdbSnapshotStartTime")]
    pub r#rdb_snapshot_start_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstancePersistenceConfigRdbConfig {
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
                    "rdb_snapshot_period",
                    &self.r#rdb_snapshot_period,
                ),
                to_pulumi_object_field(
                    "rdb_snapshot_start_time",
                    &self.r#rdb_snapshot_start_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstancePersistenceConfigRdbConfig {
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
                    r#rdb_snapshot_period: {
                        let field_value = match fields_map.get("rdb_snapshot_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'rdb_snapshot_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rdb_snapshot_start_time: {
                        let field_value = match fields_map.get("rdb_snapshot_start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'rdb_snapshot_start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
