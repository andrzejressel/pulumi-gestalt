#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkServicePccRuleServiceDataFlowTemplate {
    /// The direction of this flow. Possible values are `Uplink`, `Downlink` and `Bidirectional`.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: String,
    /// Specifies the name which should be used for this Mobile Network Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The port(s) to which UEs will connect for this flow. You can specify zero or more ports or port ranges.
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Vec<String>,
    /// A list of the allowed protocol(s) for this flow.
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Vec<String>,
    /// The remote IP address(es) to which UEs will connect for this flow.
    #[builder(into)]
    #[serde(rename = "remoteIpLists")]
    pub r#remote_ip_lists: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNetworkServicePccRuleServiceDataFlowTemplate {
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
                "direction".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#direction,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "ports".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ports,
                )
                .await,
            );
            map.insert(
                "protocols".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protocols,
                )
                .await,
            );
            map.insert(
                "remote_ip_lists".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#remote_ip_lists,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetNetworkServicePccRuleServiceDataFlowTemplate {
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
                    r#direction: {
                        let field_value = match fields_map.get("direction") {
                            Some(value) => value,
                            None => bail!("Missing field 'direction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ports: {
                        let field_value = match fields_map.get("ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocols: {
                        let field_value = match fields_map.get("protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_ip_lists: {
                        let field_value = match fields_map.get("remote_ip_lists") {
                            Some(value) => value,
                            None => bail!("Missing field 'remote_ip_lists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
