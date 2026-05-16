#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterBrokerNodeGroupInfoConnectivityInfo {
    /// Access control settings for brokers. See below.
    #[builder(into)]
    #[serde(rename = "publicAccess")]
    pub r#public_access: Option<Box<super::super::types::msk::ClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess>>,
    /// VPC connectivity access control for brokers. See below.
    #[builder(into)]
    #[serde(rename = "vpcConnectivity")]
    pub r#vpc_connectivity: Option<Box<super::super::types::msk::ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterBrokerNodeGroupInfoConnectivityInfo {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("public_access".to_string(), self.r#public_access.to_pulumi_value().await);
            map.insert("vpc_connectivity".to_string(), self.r#vpc_connectivity.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterBrokerNodeGroupInfoConnectivityInfo {
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
                    r#public_access: {
                        let field_value = match fields_map.get("public_access") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_access' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::msk::ClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#vpc_connectivity: {
                        let field_value = match fields_map.get("vpc_connectivity") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_connectivity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::msk::ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
