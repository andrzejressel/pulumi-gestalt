#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAccountConsistencyPolicy {
    /// The Consistency Level used by this CosmosDB Account.
    #[builder(into)]
    #[serde(rename = "consistencyLevel")]
    pub r#consistency_level: String,
    /// The amount of staleness (in seconds) tolerated when the consistency level is Bounded Staleness.
    #[builder(into)]
    #[serde(rename = "maxIntervalInSeconds")]
    pub r#max_interval_in_seconds: i32,
    /// The number of stale requests tolerated when the consistency level is Bounded Staleness.
    #[builder(into)]
    #[serde(rename = "maxStalenessPrefix")]
    pub r#max_staleness_prefix: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAccountConsistencyPolicy {
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
                "consistency_level".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#consistency_level,
                )
                .await,
            );
            map.insert(
                "max_interval_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_interval_in_seconds,
                )
                .await,
            );
            map.insert(
                "max_staleness_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_staleness_prefix,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAccountConsistencyPolicy {
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
                    r#consistency_level: {
                        let field_value = match fields_map.get("consistency_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'consistency_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_interval_in_seconds: {
                        let field_value = match fields_map.get("max_interval_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_interval_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_staleness_prefix: {
                        let field_value = match fields_map.get("max_staleness_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_staleness_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
