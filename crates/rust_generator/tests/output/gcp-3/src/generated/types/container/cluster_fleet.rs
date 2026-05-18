#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterFleet {
    /// The resource name of the fleet Membership resource associated to this cluster with format `//gkehub.googleapis.com/projects/{{project}}/locations/{{location}}/memberships/{{name}}`. See the official doc for [fleet management](https://cloud.google.com/kubernetes-engine/docs/fleets-overview).
    #[builder(into)]
    #[serde(rename = "membership")]
    pub r#membership: Option<String>,
    /// The short name of the fleet membership, extracted from `fleet.0.membership`. You can use this field to configure `membership_id` under google_gkehub_feature_membership.
    #[builder(into)]
    #[serde(rename = "membershipId")]
    pub r#membership_id: Option<String>,
    /// The location of the fleet membership,  extracted from `fleet.0.membership`. You can use this field to configure `membership_location` under google_gkehub_feature_membership.
    #[builder(into)]
    #[serde(rename = "membershipLocation")]
    pub r#membership_location: Option<String>,
    /// Whether the cluster has been registered via the fleet API.
    #[builder(into)]
    #[serde(rename = "preRegistered")]
    pub r#pre_registered: Option<bool>,
    /// The name of the Fleet host project where this cluster will be registered.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterFleet {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "membership",
                    &self.r#membership,
                ),
                to_pulumi_object_field(
                    "membership_id",
                    &self.r#membership_id,
                ),
                to_pulumi_object_field(
                    "membership_location",
                    &self.r#membership_location,
                ),
                to_pulumi_object_field(
                    "pre_registered",
                    &self.r#pre_registered,
                ),
                to_pulumi_object_field(
                    "project",
                    &self.r#project,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterFleet {
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
