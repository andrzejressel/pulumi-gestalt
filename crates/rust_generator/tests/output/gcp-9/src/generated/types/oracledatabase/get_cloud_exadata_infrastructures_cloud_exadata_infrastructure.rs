#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCloudExadataInfrastructuresCloudExadataInfrastructure {
    /// The ID of the Exadata Infrastructure to create. This value is restricted
    /// to (^a-z?$) and must be a maximum of 63
    /// characters in length. The value must start with a letter and end with
    /// a letter or a number.
    #[builder(into)]
    #[serde(rename = "cloudExadataInfrastructureId")]
    pub r#cloud_exadata_infrastructure_id: String,
    /// The date and time that the Exadata Infrastructure was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: String,
    #[builder(into)]
    #[serde(rename = "deletionProtection")]
    pub r#deletion_protection: bool,
    /// User friendly name for this resource.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: String,
    #[builder(into)]
    #[serde(rename = "effectiveLabels")]
    pub r#effective_labels: std::collections::HashMap<String, String>,
    /// Entitlement ID of the private offer against which this infrastructure
    /// resource is provisioned.
    #[builder(into)]
    #[serde(rename = "entitlementId")]
    pub r#entitlement_id: String,
    /// GCP location where Oracle Exadata is hosted.
    #[builder(into)]
    #[serde(rename = "gcpOracleZone")]
    pub r#gcp_oracle_zone: String,
    /// Labels or tags associated with the resource. 
    /// 
    /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
    /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: std::collections::HashMap<String, String>,
    /// The location of the resource.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// Identifier. The name of the Exadata Infrastructure resource with the following format:
    /// projects/{project}/locations/{region}/cloudExadataInfrastructures/{cloud_exadata_infrastructure}
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The project to which the resource belongs. If it
    /// is not provided, the provider project is used.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: String,
    /// Various properties of Exadata Infrastructure.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Vec<super::super::types::oracledatabase::GetCloudExadataInfrastructuresCloudExadataInfrastructureProperty>,
    /// The combination of labels configured directly on the resource
    ///  and default labels configured on the provider.
    #[builder(into)]
    #[serde(rename = "pulumiLabels")]
    pub r#pulumi_labels: std::collections::HashMap<String, String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCloudExadataInfrastructuresCloudExadataInfrastructure {
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
                    "cloudExadataInfrastructureId",
                    &self.r#cloud_exadata_infrastructure_id,
                ),
                to_pulumi_object_field(
                    "createTime",
                    &self.r#create_time,
                ),
                to_pulumi_object_field(
                    "deletionProtection",
                    &self.r#deletion_protection,
                ),
                to_pulumi_object_field(
                    "displayName",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "effectiveLabels",
                    &self.r#effective_labels,
                ),
                to_pulumi_object_field(
                    "entitlementId",
                    &self.r#entitlement_id,
                ),
                to_pulumi_object_field(
                    "gcpOracleZone",
                    &self.r#gcp_oracle_zone,
                ),
                to_pulumi_object_field(
                    "labels",
                    &self.r#labels,
                ),
                to_pulumi_object_field(
                    "location",
                    &self.r#location,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "project",
                    &self.r#project,
                ),
                to_pulumi_object_field(
                    "properties",
                    &self.r#properties,
                ),
                to_pulumi_object_field(
                    "pulumiLabels",
                    &self.r#pulumi_labels,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCloudExadataInfrastructuresCloudExadataInfrastructure {
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
                    r#cloud_exadata_infrastructure_id: {
                        let field_value = match fields_map.get("cloudExadataInfrastructureId") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudExadataInfrastructureId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_time: {
                        let field_value = match fields_map.get("createTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'createTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deletion_protection: {
                        let field_value = match fields_map.get("deletionProtection") {
                            Some(value) => value,
                            None => bail!("Missing field 'deletionProtection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("displayName") {
                            Some(value) => value,
                            None => bail!("Missing field 'displayName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#effective_labels: {
                        let field_value = match fields_map.get("effectiveLabels") {
                            Some(value) => value,
                            None => bail!("Missing field 'effectiveLabels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#entitlement_id: {
                        let field_value = match fields_map.get("entitlementId") {
                            Some(value) => value,
                            None => bail!("Missing field 'entitlementId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcp_oracle_zone: {
                        let field_value = match fields_map.get("gcpOracleZone") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcpOracleZone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#location: {
                        let field_value = match fields_map.get("location") {
                            Some(value) => value,
                            None => bail!("Missing field 'location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#properties: {
                        let field_value = match fields_map.get("properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pulumi_labels: {
                        let field_value = match fields_map.get("pulumiLabels") {
                            Some(value) => value,
                            None => bail!("Missing field 'pulumiLabels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
