#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterBrokerNodeGroupInfoConnectivityInfo {
    #[builder(into)]
    #[serde(rename = "publicAccesses")]
    pub r#public_accesses: Vec<super::super::types::msk::GetClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess>,
    #[builder(into)]
    #[serde(rename = "vpcConnectivities")]
    pub r#vpc_connectivities: Vec<super::super::types::msk::GetClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterBrokerNodeGroupInfoConnectivityInfo {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("public_accesses".to_string(), self.r#public_accesses.to_pulumi_value().await);
            map.insert("vpc_connectivities".to_string(), self.r#vpc_connectivities.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterBrokerNodeGroupInfoConnectivityInfo {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#public_accesses: {
                        let field_value = match fields_map.get("public_accesses") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_accesses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::msk::GetClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#vpc_connectivities: {
                        let field_value = match fields_map.get("vpc_connectivities") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_connectivities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::msk::GetClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
