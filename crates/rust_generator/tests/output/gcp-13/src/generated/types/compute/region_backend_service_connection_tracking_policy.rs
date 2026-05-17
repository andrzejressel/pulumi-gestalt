#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionBackendServiceConnectionTrackingPolicy {
    /// Specifies connection persistence when backends are unhealthy.
    /// If set to `DEFAULT_FOR_PROTOCOL`, the existing connections persist on
    /// unhealthy backends only for connection-oriented protocols (TCP and SCTP)
    /// and only if the Tracking Mode is PER_CONNECTION (default tracking mode)
    /// or the Session Affinity is configured for 5-tuple. They do not persist
    /// for UDP.
    /// If set to `NEVER_PERSIST`, after a backend becomes unhealthy, the existing
    /// connections on the unhealthy backend are never persisted on the unhealthy
    /// backend. They are always diverted to newly selected healthy backends
    /// (unless all backends are unhealthy).
    /// If set to `ALWAYS_PERSIST`, existing connections always persist on
    /// unhealthy backends regardless of protocol and session affinity. It is
    /// generally not recommended to use this mode overriding the default.
    /// Default value is `DEFAULT_FOR_PROTOCOL`.
    /// Possible values are: `DEFAULT_FOR_PROTOCOL`, `NEVER_PERSIST`, `ALWAYS_PERSIST`.
    #[builder(into)]
    #[serde(rename = "connectionPersistenceOnUnhealthyBackends")]
    pub r#connection_persistence_on_unhealthy_backends: Option<String>,
    /// Enable Strong Session Affinity for Network Load Balancing. This option is not available publicly.
    #[builder(into)]
    #[serde(rename = "enableStrongAffinity")]
    pub r#enable_strong_affinity: Option<bool>,
    /// Specifies how long to keep a Connection Tracking entry while there is
    /// no matching traffic (in seconds).
    /// For L4 ILB the minimum(default) is 10 minutes and maximum is 16 hours.
    /// For NLB the minimum(default) is 60 seconds and the maximum is 16 hours.
    #[builder(into)]
    #[serde(rename = "idleTimeoutSec")]
    pub r#idle_timeout_sec: Option<i32>,
    /// Specifies the key used for connection tracking. There are two options:
    /// `PER_CONNECTION`: The Connection Tracking is performed as per the
    /// Connection Key (default Hash Method) for the specific protocol.
    /// `PER_SESSION`: The Connection Tracking is performed as per the
    /// configured Session Affinity. It matches the configured Session Affinity.
    /// Default value is `PER_CONNECTION`.
    /// Possible values are: `PER_CONNECTION`, `PER_SESSION`.
    #[builder(into)]
    #[serde(rename = "trackingMode")]
    pub r#tracking_mode: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionBackendServiceConnectionTrackingPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "connection_persistence_on_unhealthy_backends",
                    &self.r#connection_persistence_on_unhealthy_backends,
                ),
                to_pulumi_object_field(
                    "enable_strong_affinity",
                    &self.r#enable_strong_affinity,
                ),
                to_pulumi_object_field(
                    "idle_timeout_sec",
                    &self.r#idle_timeout_sec,
                ),
                to_pulumi_object_field(
                    "tracking_mode",
                    &self.r#tracking_mode,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionBackendServiceConnectionTrackingPolicy {
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
                    r#connection_persistence_on_unhealthy_backends: {
                        let field_value = match fields_map.get("connection_persistence_on_unhealthy_backends") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_persistence_on_unhealthy_backends' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_strong_affinity: {
                        let field_value = match fields_map.get("enable_strong_affinity") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_strong_affinity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idle_timeout_sec: {
                        let field_value = match fields_map.get("idle_timeout_sec") {
                            Some(value) => value,
                            None => bail!("Missing field 'idle_timeout_sec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tracking_mode: {
                        let field_value = match fields_map.get("tracking_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'tracking_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
