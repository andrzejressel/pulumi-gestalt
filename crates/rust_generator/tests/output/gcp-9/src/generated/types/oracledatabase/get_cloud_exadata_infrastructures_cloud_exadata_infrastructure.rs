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
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "cloud_exadata_infrastructure_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloud_exadata_infrastructure_id,
                )
                .await,
            );
            map.insert(
                "create_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#create_time,
                )
                .await,
            );
            map.insert(
                "deletion_protection".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#deletion_protection,
                )
                .await,
            );
            map.insert(
                "display_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#display_name,
                )
                .await,
            );
            map.insert(
                "effective_labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#effective_labels,
                )
                .await,
            );
            map.insert(
                "entitlement_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#entitlement_id,
                )
                .await,
            );
            map.insert(
                "gcp_oracle_zone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gcp_oracle_zone,
                )
                .await,
            );
            map.insert(
                "labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#labels,
                )
                .await,
            );
            map.insert(
                "location".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#location,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
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
            map.insert(
                "properties".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#properties,
                )
                .await,
            );
            map.insert(
                "pulumi_labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pulumi_labels,
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
                        let field_value = match fields_map.get("cloud_exadata_infrastructure_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_exadata_infrastructure_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_time: {
                        let field_value = match fields_map.get("create_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'create_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deletion_protection: {
                        let field_value = match fields_map.get("deletion_protection") {
                            Some(value) => value,
                            None => bail!("Missing field 'deletion_protection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("display_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#effective_labels: {
                        let field_value = match fields_map.get("effective_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'effective_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#entitlement_id: {
                        let field_value = match fields_map.get("entitlement_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'entitlement_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcp_oracle_zone: {
                        let field_value = match fields_map.get("gcp_oracle_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcp_oracle_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("pulumi_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'pulumi_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
