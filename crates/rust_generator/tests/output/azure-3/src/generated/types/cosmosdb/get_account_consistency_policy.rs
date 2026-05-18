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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "consistency_level",
                    &self.r#consistency_level,
                ),
                to_pulumi_object_field(
                    "max_interval_in_seconds",
                    &self.r#max_interval_in_seconds,
                ),
                to_pulumi_object_field(
                    "max_staleness_prefix",
                    &self.r#max_staleness_prefix,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
