#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackendServiceLocalityLbPolicyPolicy {
    /// The name of a locality load balancer policy to be used. The value
    /// should be one of the predefined ones as supported by localityLbPolicy,
    /// although at the moment only ROUND_ROBIN is supported.
    /// This field should only be populated when the customPolicy field is not
    /// used.
    /// Note that specifying the same policy more than once for a backend is
    /// not a valid configuration and will be rejected.
    /// The possible values are:
    /// * `ROUND_ROBIN`: This is a simple policy in which each healthy backend
    /// is selected in round robin order.
    /// * `LEAST_REQUEST`: An O(1) algorithm which selects two random healthy
    /// hosts and picks the host which has fewer active requests.
    /// * `RING_HASH`: The ring/modulo hash load balancer implements consistent
    /// hashing to backends. The algorithm has the property that the
    /// addition/removal of a host from a set of N hosts only affects
    /// 1/N of the requests.
    /// * `RANDOM`: The load balancer selects a random healthy host.
    /// * `ORIGINAL_DESTINATION`: Backend host is selected based on the client
    /// connection metadata, i.e., connections are opened
    /// to the same address as the destination address of
    /// the incoming connection before the connection
    /// was redirected to the load balancer.
    /// * `MAGLEV`: used as a drop in replacement for the ring hash load balancer.
    /// Maglev is not as stable as ring hash but has faster table lookup
    /// build times and host selection times. For more information about
    /// Maglev, refer to https://ai.google/research/pubs/pub44824
    /// Possible values are: `ROUND_ROBIN`, `LEAST_REQUEST`, `RING_HASH`, `RANDOM`, `ORIGINAL_DESTINATION`, `MAGLEV`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackendServiceLocalityLbPolicyPolicy {
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
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackendServiceLocalityLbPolicyPolicy {
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
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
