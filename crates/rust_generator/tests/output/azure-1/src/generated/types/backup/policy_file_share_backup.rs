#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyFileShareBackup {
    /// Sets the backup frequency. Possible values are `Daily` and `Hourly`. 
    /// 
    /// > **NOTE:** This argument is made available for consistency with VM backup policies and to allow for potential future support of weekly backups
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: String,
    /// A `hourly` block defined as below. This is required when `frequency` is set to `Hourly`.
    #[builder(into)]
    #[serde(rename = "hourly")]
    pub r#hourly: Option<Box<super::super::types::backup::PolicyFileShareBackupHourly>>,
    /// The time of day to perform the backup in 24-hour format. Times must be either on the hour or half hour (e.g. 12:00, 12:30, 13:00, etc.)
    /// 
    /// > **NOTE:** `time` is required when `frequency` is set to `Daily`.
    #[builder(into)]
    #[serde(rename = "time")]
    pub r#time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyFileShareBackup {
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
                "frequency".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#frequency,
                )
                .await,
            );
            map.insert(
                "hourly".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hourly,
                )
                .await,
            );
            map.insert(
                "time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyFileShareBackup {
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
                    r#frequency: {
                        let field_value = match fields_map.get("frequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'frequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hourly: {
                        let field_value = match fields_map.get("hourly") {
                            Some(value) => value,
                            None => bail!("Missing field 'hourly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time: {
                        let field_value = match fields_map.get("time") {
                            Some(value) => value,
                            None => bail!("Missing field 'time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
