#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkInsightsAnalysisReturnPathComponentSecurityGroupRule {
    #[builder(into)]
    #[serde(rename = "cidr")]
    pub r#cidr: String,
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: String,
    #[builder(into)]
    #[serde(rename = "portRanges")]
    pub r#port_ranges: Vec<super::super::types::ec2::GetNetworkInsightsAnalysisReturnPathComponentSecurityGroupRulePortRange>,
    #[builder(into)]
    #[serde(rename = "prefixListId")]
    pub r#prefix_list_id: String,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    #[builder(into)]
    #[serde(rename = "securityGroupId")]
    pub r#security_group_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNetworkInsightsAnalysisReturnPathComponentSecurityGroupRule {
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
                "cidr".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cidr,
                )
                .await,
            );
            map.insert(
                "direction".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#direction,
                )
                .await,
            );
            map.insert(
                "port_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#port_ranges,
                )
                .await,
            );
            map.insert(
                "prefix_list_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#prefix_list_id,
                )
                .await,
            );
            map.insert(
                "protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protocol,
                )
                .await,
            );
            map.insert(
                "security_group_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_group_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetNetworkInsightsAnalysisReturnPathComponentSecurityGroupRule {
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
                    r#cidr: {
                        let field_value = match fields_map.get("cidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'cidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#direction: {
                        let field_value = match fields_map.get("direction") {
                            Some(value) => value,
                            None => bail!("Missing field 'direction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port_ranges: {
                        let field_value = match fields_map.get("port_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'port_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_list_id: {
                        let field_value = match fields_map.get("prefix_list_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix_list_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_group_id: {
                        let field_value = match fields_map.get("security_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
