#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorCapacity {
    /// Information about the auto scaling parameters for the connector. See `autoscaling` Block for details.
    #[builder(into)]
    #[serde(rename = "autoscaling")]
    pub r#autoscaling: Option<Box<super::super::types::mskconnect::ConnectorCapacityAutoscaling>>,
    /// Details about a fixed capacity allocated to a connector. See `provisioned_capacity` Block for details.
    #[builder(into)]
    #[serde(rename = "provisionedCapacity")]
    pub r#provisioned_capacity: Option<Box<super::super::types::mskconnect::ConnectorCapacityProvisionedCapacity>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectorCapacity {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("autoscaling".to_string(), self.r#autoscaling.to_pulumi_value().await);
            map.insert("provisioned_capacity".to_string(), self.r#provisioned_capacity.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectorCapacity {
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
                    r#autoscaling: {
                        let field_value = match fields_map.get("autoscaling") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoscaling' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::mskconnect::ConnectorCapacityAutoscaling>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#provisioned_capacity: {
                        let field_value = match fields_map.get("provisioned_capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioned_capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::mskconnect::ConnectorCapacityProvisionedCapacity>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
