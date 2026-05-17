#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterPersistenceConfig {
    /// AOF configuration. This field will be ignored if mode is not AOF.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "aofConfig")]
    pub r#aof_config: Option<Box<super::super::types::redis::ClusterPersistenceConfigAofConfig>>,
    /// Optional. Controls whether Persistence features are enabled. If not provided, the existing value will be used.
    /// - DISABLED: 	Persistence (both backup and restore) is disabled for the cluster.
    /// - RDB: RDB based Persistence is enabled.
    /// - AOF: AOF based Persistence is enabled.
    /// Possible values are: `PERSISTENCE_MODE_UNSPECIFIED`, `DISABLED`, `RDB`, `AOF`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// RDB configuration. This field will be ignored if mode is not RDB.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rdbConfig")]
    pub r#rdb_config: Option<Box<super::super::types::redis::ClusterPersistenceConfigRdbConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterPersistenceConfig {
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
                "aof_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aof_config,
                )
                .await,
            );
            map.insert(
                "mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mode,
                )
                .await,
            );
            map.insert(
                "rdb_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rdb_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterPersistenceConfig {
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
                    r#aof_config: {
                        let field_value = match fields_map.get("aof_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'aof_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rdb_config: {
                        let field_value = match fields_map.get("rdb_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'rdb_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
