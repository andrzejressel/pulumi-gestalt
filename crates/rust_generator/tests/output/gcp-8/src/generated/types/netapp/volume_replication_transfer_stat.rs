#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VolumeReplicationTransferStat {
    /// (Output)
    /// The elapsed time since the creation of the snapshot on the source volume that was last replicated
    /// to the destination volume. Lag time represents the difference in age of the destination volume
    /// data in relation to the source volume data.
    #[builder(into)]
    #[serde(rename = "lagDuration")]
    pub r#lag_duration: Option<String>,
    /// (Output)
    /// Size of last completed transfer in bytes.
    #[builder(into)]
    #[serde(rename = "lastTransferBytes")]
    pub r#last_transfer_bytes: Option<String>,
    /// (Output)
    /// Time taken during last completed transfer.
    #[builder(into)]
    #[serde(rename = "lastTransferDuration")]
    pub r#last_transfer_duration: Option<String>,
    /// (Output)
    /// Time when last transfer completed. A timestamp in RFC3339 UTC "Zulu" format. Examples: "2023-06-22T09:13:01.617Z".
    #[builder(into)]
    #[serde(rename = "lastTransferEndTime")]
    pub r#last_transfer_end_time: Option<String>,
    /// (Output)
    /// A message describing the cause of the last transfer failure.
    #[builder(into)]
    #[serde(rename = "lastTransferError")]
    pub r#last_transfer_error: Option<String>,
    /// (Output)
    /// Cumulative time taken across all transfers for the replication relationship.
    #[builder(into)]
    #[serde(rename = "totalTransferDuration")]
    pub r#total_transfer_duration: Option<String>,
    /// (Output)
    /// Cumulative bytes transferred so far for the replication relationship.
    #[builder(into)]
    #[serde(rename = "transferBytes")]
    pub r#transfer_bytes: Option<String>,
    /// (Output)
    /// Time when progress was updated last. A timestamp in RFC3339 UTC "Zulu" format. Examples: "2023-06-22T09:13:01.617Z".
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VolumeReplicationTransferStat {
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
                "lag_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lag_duration,
                )
                .await,
            );
            map.insert(
                "last_transfer_bytes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_transfer_bytes,
                )
                .await,
            );
            map.insert(
                "last_transfer_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_transfer_duration,
                )
                .await,
            );
            map.insert(
                "last_transfer_end_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_transfer_end_time,
                )
                .await,
            );
            map.insert(
                "last_transfer_error".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_transfer_error,
                )
                .await,
            );
            map.insert(
                "total_transfer_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#total_transfer_duration,
                )
                .await,
            );
            map.insert(
                "transfer_bytes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transfer_bytes,
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
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VolumeReplicationTransferStat {
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
                    r#lag_duration: {
                        let field_value = match fields_map.get("lag_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'lag_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_transfer_bytes: {
                        let field_value = match fields_map.get("last_transfer_bytes") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_transfer_bytes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_transfer_duration: {
                        let field_value = match fields_map.get("last_transfer_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_transfer_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_transfer_end_time: {
                        let field_value = match fields_map.get("last_transfer_end_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_transfer_end_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_transfer_error: {
                        let field_value = match fields_map.get("last_transfer_error") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_transfer_error' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#total_transfer_duration: {
                        let field_value = match fields_map.get("total_transfer_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'total_transfer_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transfer_bytes: {
                        let field_value = match fields_map.get("transfer_bytes") {
                            Some(value) => value,
                            None => bail!("Missing field 'transfer_bytes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
