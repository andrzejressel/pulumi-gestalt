#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetMetastoreServiceNetworkConfigConsumer {
    /// The URI of the endpoint used to access the metastore service.
    #[builder(into)]
    #[serde(rename = "endpointUri")]
    pub r#endpoint_uri: String,
    /// The subnetwork of the customer project from which an IP address is reserved and used as the Dataproc Metastore service's endpoint.
    /// It is accessible to hosts in the subnet and to all hosts in a subnet in the same region and same network.
    /// There must be at least one IP address available in the subnet's primary range. The subnet is specified in the following form:
    /// 'projects/{projectNumber}/regions/{region_id}/subnetworks/{subnetwork_id}
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetMetastoreServiceNetworkConfigConsumer {
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
                "endpoint_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#endpoint_uri,
                )
                .await,
            );
            map.insert(
                "subnetwork".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnetwork,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetMetastoreServiceNetworkConfigConsumer {
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
                    r#endpoint_uri: {
                        let field_value = match fields_map.get("endpoint_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnetwork: {
                        let field_value = match fields_map.get("subnetwork") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetwork' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
