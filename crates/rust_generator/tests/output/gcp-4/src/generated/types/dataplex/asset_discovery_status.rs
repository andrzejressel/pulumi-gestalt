#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AssetDiscoveryStatus {
    /// The duration of the last discovery run.
    #[builder(into)]
    #[serde(rename = "lastRunDuration")]
    pub r#last_run_duration: Option<String>,
    /// The start time of the last discovery run.
    #[builder(into)]
    #[serde(rename = "lastRunTime")]
    pub r#last_run_time: Option<String>,
    /// Additional information about the current state.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// Output only. Current state of the asset. Possible values: STATE_UNSPECIFIED, ACTIVE, CREATING, DELETING, ACTION_REQUIRED
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// Data Stats of the asset reported by discovery.
    #[builder(into)]
    #[serde(rename = "stats")]
    pub r#stats: Option<Vec<super::super::types::dataplex::AssetDiscoveryStatusStat>>,
    /// Output only. The time when the asset was last updated.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AssetDiscoveryStatus {
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
                "last_run_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_run_duration,
                )
                .await,
            );
            map.insert(
                "last_run_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_run_time,
                )
                .await,
            );
            map.insert(
                "message".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#message,
                )
                .await,
            );
            map.insert(
                "state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#state,
                )
                .await,
            );
            map.insert(
                "stats".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#stats,
                )
                .await,
            );
            map.insert(
                "update_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#update_time,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AssetDiscoveryStatus {
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
                    r#last_run_duration: {
                        let field_value = match fields_map.get("last_run_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_run_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_run_time: {
                        let field_value = match fields_map.get("last_run_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_run_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#message: {
                        let field_value = match fields_map.get("message") {
                            Some(value) => value,
                            None => bail!("Missing field 'message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#stats: {
                        let field_value = match fields_map.get("stats") {
                            Some(value) => value,
                            None => bail!("Missing field 'stats' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#update_time: {
                        let field_value = match fields_map.get("update_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'update_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
