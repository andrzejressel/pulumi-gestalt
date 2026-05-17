#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFlexibleServerStorage {
    /// Is Storage Auto Grow enabled?
    #[builder(into)]
    #[serde(rename = "autoGrowEnabled")]
    pub r#auto_grow_enabled: bool,
    /// Should IOPS be scaled automatically?
    #[builder(into)]
    #[serde(rename = "ioScalingEnabled")]
    pub r#io_scaling_enabled: bool,
    /// The storage IOPS of the MySQL Flexible Server.
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: i32,
    /// The max storage allowed for the MySQL Flexible Server.
    #[builder(into)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetFlexibleServerStorage {
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
                "auto_grow_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_grow_enabled,
                )
                .await,
            );
            map.insert(
                "io_scaling_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#io_scaling_enabled,
                )
                .await,
            );
            map.insert(
                "iops".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#iops,
                )
                .await,
            );
            map.insert(
                "size_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#size_gb,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetFlexibleServerStorage {
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
                    r#auto_grow_enabled: {
                        let field_value = match fields_map.get("auto_grow_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_grow_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#io_scaling_enabled: {
                        let field_value = match fields_map.get("io_scaling_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'io_scaling_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iops: {
                        let field_value = match fields_map.get("iops") {
                            Some(value) => value,
                            None => bail!("Missing field 'iops' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#size_gb: {
                        let field_value = match fields_map.get("size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
