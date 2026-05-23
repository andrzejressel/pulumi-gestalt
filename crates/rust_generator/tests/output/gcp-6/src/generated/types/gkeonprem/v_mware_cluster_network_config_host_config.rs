#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareClusterNetworkConfigHostConfig {
    /// DNS search domains.
    /// 
    /// <a name="nested_control_plane_v2_config"></a>The `control_plane_v2_config` block supports:
    #[builder(into)]
    pub r#dns_search_domains: Option<Vec<String>>,
    /// DNS servers.
    #[builder(into)]
    pub r#dns_servers: Option<Vec<String>>,
    /// NTP servers.
    #[builder(into)]
    pub r#ntp_servers: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VMwareClusterNetworkConfigHostConfig {
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
                    "dnsSearchDomains",
                    &self.r#dns_search_domains,
                ),
                to_pulumi_object_field(
                    "dnsServers",
                    &self.r#dns_servers,
                ),
                to_pulumi_object_field(
                    "ntpServers",
                    &self.r#ntp_servers,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VMwareClusterNetworkConfigHostConfig {
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
                    r#dns_search_domains: {
                        let field_value = match fields_map.get("dnsSearchDomains") {
                            Some(value) => value,
                            None => bail!("Missing field 'dnsSearchDomains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dns_servers: {
                        let field_value = match fields_map.get("dnsServers") {
                            Some(value) => value,
                            None => bail!("Missing field 'dnsServers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ntp_servers: {
                        let field_value = match fields_map.get("ntpServers") {
                            Some(value) => value,
                            None => bail!("Missing field 'ntpServers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
