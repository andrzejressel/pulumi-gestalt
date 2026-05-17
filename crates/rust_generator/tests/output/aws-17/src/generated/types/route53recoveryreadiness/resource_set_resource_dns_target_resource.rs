#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResourceSetResourceDnsTargetResource {
    /// DNS Name that acts as the ingress point to a portion of application.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: String,
    /// Hosted Zone ARN that contains the DNS record with the provided name of target resource.
    #[builder(into)]
    #[serde(rename = "hostedZoneArn")]
    pub r#hosted_zone_arn: Option<String>,
    /// Route53 record set id to uniquely identify a record given a `domain_name` and a `record_type`.
    #[builder(into)]
    #[serde(rename = "recordSetId")]
    pub r#record_set_id: Option<String>,
    /// Type of DNS Record of target resource.
    #[builder(into)]
    #[serde(rename = "recordType")]
    pub r#record_type: Option<String>,
    /// Target resource the R53 record specified with the above params points to.
    #[builder(into)]
    #[serde(rename = "targetResource")]
    pub r#target_resource: Option<Box<super::super::types::route53recoveryreadiness::ResourceSetResourceDnsTargetResourceTargetResource>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResourceSetResourceDnsTargetResource {
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
                "domain_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#domain_name,
                )
                .await,
            );
            map.insert(
                "hosted_zone_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hosted_zone_arn,
                )
                .await,
            );
            map.insert(
                "record_set_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#record_set_id,
                )
                .await,
            );
            map.insert(
                "record_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#record_type,
                )
                .await,
            );
            map.insert(
                "target_resource".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_resource,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResourceSetResourceDnsTargetResource {
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
                    r#domain_name: {
                        let field_value = match fields_map.get("domain_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hosted_zone_arn: {
                        let field_value = match fields_map.get("hosted_zone_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'hosted_zone_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_set_id: {
                        let field_value = match fields_map.get("record_set_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_set_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_type: {
                        let field_value = match fields_map.get("record_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_resource: {
                        let field_value = match fields_map.get("target_resource") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_resource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
