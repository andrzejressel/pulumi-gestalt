#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionProfileAlloydbSettings {
    /// Required. Input only. Initial user to setup during cluster creation.
    /// Structure is documented below.
    #[builder(into)]
    pub r#initial_user: Box<super::super::types::databasemigrationservice::ConnectionProfileAlloydbSettingsInitialUser>,
    /// Labels for the AlloyDB cluster created by DMS.
    #[builder(into)]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// Settings for the cluster's primary instance
    /// Structure is documented below.
    #[builder(into)]
    pub r#primary_instance_settings: Option<Box<super::super::types::databasemigrationservice::ConnectionProfileAlloydbSettingsPrimaryInstanceSettings>>,
    /// Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster.
    /// It is specified in the form: 'projects/{project_number}/global/networks/{network_id}'. This is required to create a cluster.
    #[builder(into)]
    pub r#vpc_network: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionProfileAlloydbSettings {
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
                    "initialUser",
                    &self.r#initial_user,
                ),
                to_pulumi_object_field(
                    "labels",
                    &self.r#labels,
                ),
                to_pulumi_object_field(
                    "primaryInstanceSettings",
                    &self.r#primary_instance_settings,
                ),
                to_pulumi_object_field(
                    "vpcNetwork",
                    &self.r#vpc_network,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionProfileAlloydbSettings {
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
                    r#initial_user: {
                        let field_value = match fields_map.get("initialUser") {
                            Some(value) => value,
                            None => bail!("Missing field 'initialUser' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary_instance_settings: {
                        let field_value = match fields_map.get("primaryInstanceSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'primaryInstanceSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_network: {
                        let field_value = match fields_map.get("vpcNetwork") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpcNetwork' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
