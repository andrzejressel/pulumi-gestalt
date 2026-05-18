#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountBackup {
    /// The interval in minutes between two backups. Possible values are between 60 and 1440. Defaults to `240`.
    #[builder(into)]
    #[serde(rename = "intervalInMinutes")]
    pub r#interval_in_minutes: Option<i32>,
    /// The time in hours that each backup is retained. Possible values are between 8 and 720. Defaults to `8`.
    #[builder(into)]
    #[serde(rename = "retentionInHours")]
    pub r#retention_in_hours: Option<i32>,
    /// The storage redundancy is used to indicate the type of backup residency. Possible values are `Geo`, `Local` and `Zone`. Defaults to `Geo`.
    /// 
    /// > **Note:** You can only configure `interval_in_minutes`, `retention_in_hours` and `storage_redundancy` when the `type` field is set to `Periodic`.
    #[builder(into)]
    #[serde(rename = "storageRedundancy")]
    pub r#storage_redundancy: Option<String>,
    /// The continuous backup tier. Possible values are `Continuous7Days` and `Continuous30Days`.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: Option<String>,
    /// The type of the `backup`. Possible values are `Continuous` and `Periodic`.
    /// 
    /// > **Note:** Migration of `Periodic` to `Continuous` is one-way, changing `Continuous` to `Periodic` forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccountBackup {
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
                    "interval_in_minutes",
                    &self.r#interval_in_minutes,
                ),
                to_pulumi_object_field(
                    "retention_in_hours",
                    &self.r#retention_in_hours,
                ),
                to_pulumi_object_field(
                    "storage_redundancy",
                    &self.r#storage_redundancy,
                ),
                to_pulumi_object_field(
                    "tier",
                    &self.r#tier,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccountBackup {
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
                    r#interval_in_minutes: {
                        let field_value = match fields_map.get("interval_in_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'interval_in_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#retention_in_hours: {
                        let field_value = match fields_map.get("retention_in_hours") {
                            Some(value) => value,
                            None => bail!("Missing field 'retention_in_hours' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_redundancy: {
                        let field_value = match fields_map.get("storage_redundancy") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_redundancy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tier: {
                        let field_value = match fields_map.get("tier") {
                            Some(value) => value,
                            None => bail!("Missing field 'tier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
