#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LaunchTemplateCapacityReservationSpecification {
    /// Indicates the instance's Capacity Reservation preferences. Can be `open` or `none`. (Default `none`).
    #[builder(into)]
    #[serde(rename = "capacityReservationPreference")]
    pub r#capacity_reservation_preference: Option<String>,
    /// Used to target a specific Capacity Reservation:
    #[builder(into)]
    #[serde(rename = "capacityReservationTarget")]
    pub r#capacity_reservation_target: Option<Box<super::super::types::ec2::LaunchTemplateCapacityReservationSpecificationCapacityReservationTarget>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LaunchTemplateCapacityReservationSpecification {
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
                "capacity_reservation_preference".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#capacity_reservation_preference,
                )
                .await,
            );
            map.insert(
                "capacity_reservation_target".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#capacity_reservation_target,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LaunchTemplateCapacityReservationSpecification {
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
                    r#capacity_reservation_preference: {
                        let field_value = match fields_map.get("capacity_reservation_preference") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacity_reservation_preference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#capacity_reservation_target: {
                        let field_value = match fields_map.get("capacity_reservation_target") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacity_reservation_target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
