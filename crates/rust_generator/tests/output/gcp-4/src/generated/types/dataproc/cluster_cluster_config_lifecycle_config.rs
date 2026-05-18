#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterConfigLifecycleConfig {
    /// The time when cluster will be auto-deleted.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
    /// Example: "2014-10-02T15:01:23.045123456Z".
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "autoDeleteTime")]
    pub r#auto_delete_time: Option<String>,
    /// The duration to keep the cluster alive while idling
    /// (no jobs running). After this TTL, the cluster will be deleted. Valid range: [10m, 14d].
    #[builder(into)]
    #[serde(rename = "idleDeleteTtl")]
    pub r#idle_delete_ttl: Option<String>,
    /// Time when the cluster became idle
    /// (most recent job finished) and became eligible for deletion due to idleness.
    #[builder(into)]
    #[serde(rename = "idleStartTime")]
    pub r#idle_start_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterClusterConfigLifecycleConfig {
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
                    "auto_delete_time",
                    &self.r#auto_delete_time,
                ),
                to_pulumi_object_field(
                    "idle_delete_ttl",
                    &self.r#idle_delete_ttl,
                ),
                to_pulumi_object_field(
                    "idle_start_time",
                    &self.r#idle_start_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterClusterConfigLifecycleConfig {
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
                    r#auto_delete_time: {
                        let field_value = match fields_map.get("auto_delete_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_delete_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idle_delete_ttl: {
                        let field_value = match fields_map.get("idle_delete_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'idle_delete_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idle_start_time: {
                        let field_value = match fields_map.get("idle_start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'idle_start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
