#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterMaintenancePolicy {
    /// Exclusions to automatic maintenance. Non-emergency maintenance should not occur
    /// in these windows. Each exclusion has a unique name and may be active or expired.
    /// The max number of maintenance exclusions allowed at a given time is 3.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "maintenanceExclusions")]
    pub r#maintenance_exclusions: Option<Vec<super::super::types::edgecontainer::ClusterMaintenancePolicyMaintenanceExclusion>>,
    /// Specifies the maintenance window in which maintenance may be performed.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "window")]
    pub r#window: Box<super::super::types::edgecontainer::ClusterMaintenancePolicyWindow>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterMaintenancePolicy {
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
                "maintenance_exclusions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maintenance_exclusions,
                )
                .await,
            );
            map.insert(
                "window".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#window,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterMaintenancePolicy {
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
                    r#maintenance_exclusions: {
                        let field_value = match fields_map.get("maintenance_exclusions") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenance_exclusions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#window: {
                        let field_value = match fields_map.get("window") {
                            Some(value) => value,
                            None => bail!("Missing field 'window' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
