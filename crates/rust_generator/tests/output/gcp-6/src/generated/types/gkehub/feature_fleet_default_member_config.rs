#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureFleetDefaultMemberConfig {
    /// Config Management spec
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "configmanagement")]
    pub r#configmanagement: Option<Box<super::super::types::gkehub::FeatureFleetDefaultMemberConfigConfigmanagement>>,
    /// Service Mesh spec
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "mesh")]
    pub r#mesh: Option<Box<super::super::types::gkehub::FeatureFleetDefaultMemberConfigMesh>>,
    /// Policy Controller spec
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "policycontroller")]
    pub r#policycontroller: Option<Box<super::super::types::gkehub::FeatureFleetDefaultMemberConfigPolicycontroller>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureFleetDefaultMemberConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "configmanagement",
                    &self.r#configmanagement,
                ),
                to_pulumi_object_field(
                    "mesh",
                    &self.r#mesh,
                ),
                to_pulumi_object_field(
                    "policycontroller",
                    &self.r#policycontroller,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureFleetDefaultMemberConfig {
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
                    r#configmanagement: {
                        let field_value = match fields_map.get("configmanagement") {
                            Some(value) => value,
                            None => bail!("Missing field 'configmanagement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mesh: {
                        let field_value = match fields_map.get("mesh") {
                            Some(value) => value,
                            None => bail!("Missing field 'mesh' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policycontroller: {
                        let field_value = match fields_map.get("policycontroller") {
                            Some(value) => value,
                            None => bail!("Missing field 'policycontroller' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
