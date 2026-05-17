#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterBrokerNodeGroupInfo {
    #[builder(into)]
    #[serde(rename = "azDistribution")]
    pub r#az_distribution: String,
    #[builder(into)]
    #[serde(rename = "clientSubnets")]
    pub r#client_subnets: Vec<String>,
    #[builder(into)]
    #[serde(rename = "connectivityInfos")]
    pub r#connectivity_infos: Vec<super::super::types::msk::GetClusterBrokerNodeGroupInfoConnectivityInfo>,
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: String,
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Vec<String>,
    #[builder(into)]
    #[serde(rename = "storageInfos")]
    pub r#storage_infos: Vec<super::super::types::msk::GetClusterBrokerNodeGroupInfoStorageInfo>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterBrokerNodeGroupInfo {
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
                "az_distribution".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#az_distribution,
                )
                .await,
            );
            map.insert(
                "client_subnets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_subnets,
                )
                .await,
            );
            map.insert(
                "connectivity_infos".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connectivity_infos,
                )
                .await,
            );
            map.insert(
                "instance_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_type,
                )
                .await,
            );
            map.insert(
                "security_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#security_groups,
                )
                .await,
            );
            map.insert(
                "storage_infos".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_infos,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterBrokerNodeGroupInfo {
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
                    r#az_distribution: {
                        let field_value = match fields_map.get("az_distribution") {
                            Some(value) => value,
                            None => bail!("Missing field 'az_distribution' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_subnets: {
                        let field_value = match fields_map.get("client_subnets") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_subnets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connectivity_infos: {
                        let field_value = match fields_map.get("connectivity_infos") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectivity_infos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_type: {
                        let field_value = match fields_map.get("instance_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_groups: {
                        let field_value = match fields_map.get("security_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_infos: {
                        let field_value = match fields_map.get("storage_infos") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_infos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
