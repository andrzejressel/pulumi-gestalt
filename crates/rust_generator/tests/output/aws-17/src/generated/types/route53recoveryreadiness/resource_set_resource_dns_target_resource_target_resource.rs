#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResourceSetResourceDnsTargetResourceTargetResource {
    /// NLB resource a DNS Target Resource points to. Required if `r53_resource` is not set.
    #[builder(into)]
    #[serde(rename = "nlbResource")]
    pub r#nlb_resource: Option<Box<super::super::types::route53recoveryreadiness::ResourceSetResourceDnsTargetResourceTargetResourceNlbResource>>,
    /// Route53 resource a DNS Target Resource record points to.
    #[builder(into)]
    #[serde(rename = "r53Resource")]
    pub r#r_53_resource: Option<Box<super::super::types::route53recoveryreadiness::ResourceSetResourceDnsTargetResourceTargetResourceR53Resource>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResourceSetResourceDnsTargetResourceTargetResource {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("nlb_resource".to_string(), self.r#nlb_resource.to_pulumi_value().await);
            map.insert("r_53_resource".to_string(), self.r#r_53_resource.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResourceSetResourceDnsTargetResourceTargetResource {
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
                    r#nlb_resource: {
                        let field_value = match fields_map.get("nlb_resource") {
                            Some(value) => value,
                            None => bail!("Missing field 'nlb_resource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::route53recoveryreadiness::ResourceSetResourceDnsTargetResourceTargetResourceNlbResource>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#r_53_resource: {
                        let field_value = match fields_map.get("r_53_resource") {
                            Some(value) => value,
                            None => bail!("Missing field 'r_53_resource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::route53recoveryreadiness::ResourceSetResourceDnsTargetResourceTargetResourceR53Resource>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
