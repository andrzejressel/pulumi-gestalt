#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualNodeSpecServiceDiscovery {
    #[builder(into)]
    #[serde(rename = "awsCloudMaps")]
    pub r#aws_cloud_maps: Vec<super::super::types::appmesh::GetVirtualNodeSpecServiceDiscoveryAwsCloudMap>,
    #[builder(into)]
    #[serde(rename = "dns")]
    pub r#dns: Vec<super::super::types::appmesh::GetVirtualNodeSpecServiceDiscoveryDn>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualNodeSpecServiceDiscovery {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("aws_cloud_maps".to_string(), self.r#aws_cloud_maps.to_pulumi_value().await);
            map.insert("dns".to_string(), self.r#dns.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualNodeSpecServiceDiscovery {
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
                    r#aws_cloud_maps: {
                        let field_value = match fields_map.get("aws_cloud_maps") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_cloud_maps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetVirtualNodeSpecServiceDiscoveryAwsCloudMap> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dns: {
                        let field_value = match fields_map.get("dns") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::appmesh::GetVirtualNodeSpecServiceDiscoveryDn> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
