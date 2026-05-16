#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNodeSpecServiceDiscovery {
    /// Any AWS Cloud Map information for the virtual node.
    #[builder(into)]
    #[serde(rename = "awsCloudMap")]
    pub r#aws_cloud_map: Option<Box<super::super::types::appmesh::VirtualNodeSpecServiceDiscoveryAwsCloudMap>>,
    /// DNS service name for the virtual node.
    #[builder(into)]
    #[serde(rename = "dns")]
    pub r#dns: Option<Box<super::super::types::appmesh::VirtualNodeSpecServiceDiscoveryDns>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualNodeSpecServiceDiscovery {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("aws_cloud_map".to_string(), self.r#aws_cloud_map.to_pulumi_value().await);
            map.insert("dns".to_string(), self.r#dns.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualNodeSpecServiceDiscovery {
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
                    r#aws_cloud_map: {
                        let field_value = match fields_map.get("aws_cloud_map") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_cloud_map' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualNodeSpecServiceDiscoveryAwsCloudMap>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dns: {
                        let field_value = match fields_map.get("dns") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::appmesh::VirtualNodeSpecServiceDiscoveryDns>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
