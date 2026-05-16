#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResourceSetResource {
    #[builder(into)]
    #[serde(rename = "componentId")]
    pub r#component_id: Option<String>,
    /// Component for DNS/Routing Control Readiness Checks.
    #[builder(into)]
    #[serde(rename = "dnsTargetResource")]
    pub r#dns_target_resource: Option<Box<super::super::types::route53recoveryreadiness::ResourceSetResourceDnsTargetResource>>,
    /// Recovery group ARN or cell ARN that contains this resource set.
    #[builder(into)]
    #[serde(rename = "readinessScopes")]
    pub r#readiness_scopes: Option<Vec<String>>,
    /// ARN of the resource.
    #[builder(into)]
    #[serde(rename = "resourceArn")]
    pub r#resource_arn: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResourceSetResource {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("component_id".to_string(), self.r#component_id.to_pulumi_value().await);
            map.insert("dns_target_resource".to_string(), self.r#dns_target_resource.to_pulumi_value().await);
            map.insert("readiness_scopes".to_string(), self.r#readiness_scopes.to_pulumi_value().await);
            map.insert("resource_arn".to_string(), self.r#resource_arn.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResourceSetResource {
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
                    r#component_id: {
                        let field_value = match fields_map.get("component_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'component_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dns_target_resource: {
                        let field_value = match fields_map.get("dns_target_resource") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_target_resource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::route53recoveryreadiness::ResourceSetResourceDnsTargetResource>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#readiness_scopes: {
                        let field_value = match fields_map.get("readiness_scopes") {
                            Some(value) => value,
                            None => bail!("Missing field 'readiness_scopes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#resource_arn: {
                        let field_value = match fields_map.get("resource_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
