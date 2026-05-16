#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionOriginGroup {
    /// The failover criteria for when to failover to the secondary origin.
    #[builder(into)]
    #[serde(rename = "failoverCriteria")]
    pub r#failover_criteria: Box<super::super::types::cloudfront::DistributionOriginGroupFailoverCriteria>,
    /// Ordered member configuration blocks assigned to the origin group, where the first member is the primary origin. You must specify two members.
    #[builder(into)]
    #[serde(rename = "members")]
    pub r#members: Vec<super::super::types::cloudfront::DistributionOriginGroupMember>,
    #[builder(into)]
    #[serde(rename = "originId")]
    pub r#origin_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionOriginGroup {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("failover_criteria".to_string(), self.r#failover_criteria.to_pulumi_value().await);
            map.insert("members".to_string(), self.r#members.to_pulumi_value().await);
            map.insert("origin_id".to_string(), self.r#origin_id.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionOriginGroup {
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
                    r#failover_criteria: {
                        let field_value = match fields_map.get("failover_criteria") {
                            Some(value) => value,
                            None => bail!("Missing field 'failover_criteria' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::cloudfront::DistributionOriginGroupFailoverCriteria> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#members: {
                        let field_value = match fields_map.get("members") {
                            Some(value) => value,
                            None => bail!("Missing field 'members' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::DistributionOriginGroupMember> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#origin_id: {
                        let field_value = match fields_map.get("origin_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
