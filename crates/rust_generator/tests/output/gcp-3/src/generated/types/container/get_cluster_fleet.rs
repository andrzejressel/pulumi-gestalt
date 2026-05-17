#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterFleet {
    /// Full resource name of the registered fleet membership of the cluster.
    #[builder(into)]
    #[serde(rename = "membership")]
    pub r#membership: String,
    /// Short name of the fleet membership, for example "member-1".
    #[builder(into)]
    #[serde(rename = "membershipId")]
    pub r#membership_id: String,
    /// Location of the fleet membership, for example "us-central1".
    #[builder(into)]
    #[serde(rename = "membershipLocation")]
    pub r#membership_location: String,
    /// Whether the cluster has been registered via the fleet API.
    #[builder(into)]
    #[serde(rename = "preRegistered")]
    pub r#pre_registered: bool,
    /// The project in which the resource belongs. If it
    /// is not provided, the provider project is used.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterFleet {
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
                "membership".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#membership,
                )
                .await,
            );
            map.insert(
                "membership_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#membership_id,
                )
                .await,
            );
            map.insert(
                "membership_location".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#membership_location,
                )
                .await,
            );
            map.insert(
                "pre_registered".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pre_registered,
                )
                .await,
            );
            map.insert(
                "project".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#project,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterFleet {
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
                    r#membership: {
                        let field_value = match fields_map.get("membership") {
                            Some(value) => value,
                            None => bail!("Missing field 'membership' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#membership_id: {
                        let field_value = match fields_map.get("membership_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'membership_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#membership_location: {
                        let field_value = match fields_map.get("membership_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'membership_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pre_registered: {
                        let field_value = match fields_map.get("pre_registered") {
                            Some(value) => value,
                            None => bail!("Missing field 'pre_registered' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#project: {
                        let field_value = match fields_map.get("project") {
                            Some(value) => value,
                            None => bail!("Missing field 'project' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
