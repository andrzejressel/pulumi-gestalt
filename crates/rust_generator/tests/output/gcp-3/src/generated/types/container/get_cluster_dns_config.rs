#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterDnsConfig {
    /// Enable additive VPC scope DNS in a GKE cluster.
    #[builder(into)]
    #[serde(rename = "additiveVpcScopeDnsDomain")]
    pub r#additive_vpc_scope_dns_domain: String,
    /// Which in-cluster DNS provider should be used.
    #[builder(into)]
    #[serde(rename = "clusterDns")]
    pub r#cluster_dns: String,
    /// The suffix used for all cluster service records.
    #[builder(into)]
    #[serde(rename = "clusterDnsDomain")]
    pub r#cluster_dns_domain: String,
    /// The scope of access to cluster DNS records.
    #[builder(into)]
    #[serde(rename = "clusterDnsScope")]
    pub r#cluster_dns_scope: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterDnsConfig {
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
                "additive_vpc_scope_dns_domain".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#additive_vpc_scope_dns_domain,
                )
                .await,
            );
            map.insert(
                "cluster_dns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_dns,
                )
                .await,
            );
            map.insert(
                "cluster_dns_domain".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_dns_domain,
                )
                .await,
            );
            map.insert(
                "cluster_dns_scope".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_dns_scope,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterDnsConfig {
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
                    r#additive_vpc_scope_dns_domain: {
                        let field_value = match fields_map.get("additive_vpc_scope_dns_domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'additive_vpc_scope_dns_domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_dns: {
                        let field_value = match fields_map.get("cluster_dns") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_dns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_dns_domain: {
                        let field_value = match fields_map.get("cluster_dns_domain") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_dns_domain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_dns_scope: {
                        let field_value = match fields_map.get("cluster_dns_scope") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_dns_scope' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
