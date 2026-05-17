#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetUpdateRunManagedClusterUpdate {
    /// A `node_image_selection` block as defined below.
    #[builder(into)]
    #[serde(rename = "nodeImageSelection")]
    pub r#node_image_selection: Option<Box<super::super::types::containerservice::FleetUpdateRunManagedClusterUpdateNodeImageSelection>>,
    /// A `upgrade` block as defined below.
    #[builder(into)]
    #[serde(rename = "upgrade")]
    pub r#upgrade: Box<super::super::types::containerservice::FleetUpdateRunManagedClusterUpdateUpgrade>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FleetUpdateRunManagedClusterUpdate {
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
                "node_image_selection".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_image_selection,
                )
                .await,
            );
            map.insert(
                "upgrade".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#upgrade,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FleetUpdateRunManagedClusterUpdate {
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
                    r#node_image_selection: {
                        let field_value = match fields_map.get("node_image_selection") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_image_selection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upgrade: {
                        let field_value = match fields_map.get("upgrade") {
                            Some(value) => value,
                            None => bail!("Missing field 'upgrade' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
