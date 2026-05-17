#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleAppVersionAutomaticScalingDiskUtilization {
    /// Target bytes read per second.
    #[builder(into)]
    #[serde(rename = "targetReadBytesPerSecond")]
    pub r#target_read_bytes_per_second: Option<i32>,
    /// Target ops read per seconds.
    #[builder(into)]
    #[serde(rename = "targetReadOpsPerSecond")]
    pub r#target_read_ops_per_second: Option<i32>,
    /// Target bytes written per second.
    #[builder(into)]
    #[serde(rename = "targetWriteBytesPerSecond")]
    pub r#target_write_bytes_per_second: Option<i32>,
    /// Target ops written per second.
    #[builder(into)]
    #[serde(rename = "targetWriteOpsPerSecond")]
    pub r#target_write_ops_per_second: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlexibleAppVersionAutomaticScalingDiskUtilization {
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
                "target_read_bytes_per_second".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_read_bytes_per_second,
                )
                .await,
            );
            map.insert(
                "target_read_ops_per_second".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_read_ops_per_second,
                )
                .await,
            );
            map.insert(
                "target_write_bytes_per_second".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_write_bytes_per_second,
                )
                .await,
            );
            map.insert(
                "target_write_ops_per_second".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_write_ops_per_second,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlexibleAppVersionAutomaticScalingDiskUtilization {
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
                    r#target_read_bytes_per_second: {
                        let field_value = match fields_map.get("target_read_bytes_per_second") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_read_bytes_per_second' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_read_ops_per_second: {
                        let field_value = match fields_map.get("target_read_ops_per_second") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_read_ops_per_second' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_write_bytes_per_second: {
                        let field_value = match fields_map.get("target_write_bytes_per_second") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_write_bytes_per_second' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_write_ops_per_second: {
                        let field_value = match fields_map.get("target_write_ops_per_second") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_write_ops_per_second' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
