#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventEndpointRoutingConfigFailoverConfig {
    /// Parameters used for the primary Region. Documented below.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Box<super::super::types::cloudwatch::EventEndpointRoutingConfigFailoverConfigPrimary>,
    /// Parameters used for the secondary Region, the Region that events are routed to when failover is triggered or event replication is enabled. Documented below.
    #[builder(into)]
    #[serde(rename = "secondary")]
    pub r#secondary: Box<super::super::types::cloudwatch::EventEndpointRoutingConfigFailoverConfigSecondary>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EventEndpointRoutingConfigFailoverConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("primary".to_string(), self.r#primary.to_pulumi_value().await);
            map.insert("secondary".to_string(), self.r#secondary.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EventEndpointRoutingConfigFailoverConfig {
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
                    r#primary: {
                        let field_value = match fields_map.get("primary") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::cloudwatch::EventEndpointRoutingConfigFailoverConfigPrimary> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#secondary: {
                        let field_value = match fields_map.get("secondary") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::cloudwatch::EventEndpointRoutingConfigFailoverConfigSecondary> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
