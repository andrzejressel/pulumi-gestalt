#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterScalingConfiguration {
    /// Whether to enable automatic pause. A DB cluster can be paused only when it's idle (it has no connections). If a DB cluster is paused for more than seven days, the DB cluster might be backed up with a snapshot. In this case, the DB cluster is restored when there is a request to connect to it. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "autoPause")]
    pub r#auto_pause: Option<bool>,
    /// Maximum capacity for an Aurora DB cluster in `serverless` DB engine mode. The maximum capacity must be greater than or equal to the minimum capacity. Valid Aurora MySQL capacity values are `1`, `2`, `4`, `8`, `16`, `32`, `64`, `128`, `256`. Valid Aurora PostgreSQL capacity values are (`2`, `4`, `8`, `16`, `32`, `64`, `192`, and `384`). Defaults to `16`.
    #[builder(into)]
    #[serde(rename = "maxCapacity")]
    pub r#max_capacity: Option<i32>,
    /// Minimum capacity for an Aurora DB cluster in `serverless` DB engine mode. The minimum capacity must be lesser than or equal to the maximum capacity. Valid Aurora MySQL capacity values are `1`, `2`, `4`, `8`, `16`, `32`, `64`, `128`, `256`. Valid Aurora PostgreSQL capacity values are (`2`, `4`, `8`, `16`, `32`, `64`, `192`, and `384`). Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "minCapacity")]
    pub r#min_capacity: Option<i32>,
    /// Amount of time, in seconds, that Aurora Serverless v1 tries to find a scaling point to perform seamless scaling before enforcing the timeout action. Valid values are `60` through `600`. Defaults to `300`.
    #[builder(into)]
    #[serde(rename = "secondsBeforeTimeout")]
    pub r#seconds_before_timeout: Option<i32>,
    /// Time, in seconds, before an Aurora DB cluster in serverless mode is paused. Valid values are `300` through `86400`. Defaults to `300`.
    #[builder(into)]
    #[serde(rename = "secondsUntilAutoPause")]
    pub r#seconds_until_auto_pause: Option<i32>,
    /// Action to take when the timeout is reached. Valid values: `ForceApplyCapacityChange`, `RollbackCapacityChange`. Defaults to `RollbackCapacityChange`. See [documentation](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/aurora-serverless-v1.how-it-works.html#aurora-serverless.how-it-works.timeout-action).
    #[builder(into)]
    #[serde(rename = "timeoutAction")]
    pub r#timeout_action: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterScalingConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "auto_pause".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_pause,
                )
                .await,
            );
            map.insert(
                "max_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_capacity,
                )
                .await,
            );
            map.insert(
                "min_capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_capacity,
                )
                .await,
            );
            map.insert(
                "seconds_before_timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#seconds_before_timeout,
                )
                .await,
            );
            map.insert(
                "seconds_until_auto_pause".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#seconds_until_auto_pause,
                )
                .await,
            );
            map.insert(
                "timeout_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeout_action,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterScalingConfiguration {
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
                    r#auto_pause: {
                        let field_value = match fields_map.get("auto_pause") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_pause' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_capacity: {
                        let field_value = match fields_map.get("max_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_capacity: {
                        let field_value = match fields_map.get("min_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#seconds_before_timeout: {
                        let field_value = match fields_map.get("seconds_before_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'seconds_before_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#seconds_until_auto_pause: {
                        let field_value = match fields_map.get("seconds_until_auto_pause") {
                            Some(value) => value,
                            None => bail!("Missing field 'seconds_until_auto_pause' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout_action: {
                        let field_value = match fields_map.get("timeout_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
