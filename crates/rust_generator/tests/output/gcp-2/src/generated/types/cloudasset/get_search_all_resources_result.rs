#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSearchAllResourcesResult {
    /// The type of this resource.
    #[builder(into)]
    #[serde(rename = "assetType")]
    pub r#asset_type: String,
    /// The create timestamp of this resource, at which the resource was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: String,
    /// One or more paragraphs of text description of this resource. Maximum length could be up to 1M bytes.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The display name of this resource.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: String,
    /// The folder(s) that this resource belongs to, in the form of `folders/{FOLDER_NUMBER}`. This field is available when the resource belongs to one or more folders.
    #[builder(into)]
    #[serde(rename = "folders")]
    pub r#folders: Vec<String>,
    /// The Cloud KMS CryptoKey names or CryptoKeyVersion names. This field is available only when the resource's Protobuf contains it.
    #[builder(into)]
    #[serde(rename = "kmsKeys")]
    pub r#kms_keys: Vec<String>,
    /// Labels associated with this resource.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: std::collections::HashMap<String, String>,
    /// Location can be `global`, regional like `us-east1`, or zonal like `us-west1-b`.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// The full resource name of this resource.. See [Resource Names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Network tags associated with this resource.
    #[builder(into)]
    #[serde(rename = "networkTags")]
    pub r#network_tags: Vec<String>,
    /// The organization that this resource belongs to, in the form of `organizations/{ORGANIZATION_NUMBER}`. This field is available when the resource belongs to an organization.
    #[builder(into)]
    #[serde(rename = "organization")]
    pub r#organization: String,
    /// The type of this resource's immediate parent, if there is one.
    #[builder(into)]
    #[serde(rename = "parentAssetType")]
    pub r#parent_asset_type: String,
    /// The full resource name of this resource's parent, if it has one.
    #[builder(into)]
    #[serde(rename = "parentFullResourceName")]
    pub r#parent_full_resource_name: String,
    /// The project that this resource belongs to, in the form of `projects/{project_number}`.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: String,
    /// The state of this resource.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: String,
    /// The last update timestamp of this resource, at which the resource was last modified or deleted.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSearchAllResourcesResult {
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
                    "asset_type",
                    &self.r#asset_type,
                ),
                to_pulumi_object_field(
                    "create_time",
                    &self.r#create_time,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "display_name",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "folders",
                    &self.r#folders,
                ),
                to_pulumi_object_field(
                    "kms_keys",
                    &self.r#kms_keys,
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
                    "network_tags",
                    &self.r#network_tags,
                ),
                to_pulumi_object_field(
                    "organization",
                    &self.r#organization,
                ),
                to_pulumi_object_field(
                    "parent_asset_type",
                    &self.r#parent_asset_type,
                ),
                to_pulumi_object_field(
                    "parent_full_resource_name",
                    &self.r#parent_full_resource_name,
                ),
                to_pulumi_object_field(
                    "project",
                    &self.r#project,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
                to_pulumi_object_field(
                    "update_time",
                    &self.r#update_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSearchAllResourcesResult {
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
                    r#asset_type: {
                        let field_value = match fields_map.get("asset_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'asset_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#folders: {
                        let field_value = match fields_map.get("folders") {
                            Some(value) => value,
                            None => bail!("Missing field 'folders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_keys: {
                        let field_value = match fields_map.get("kms_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#network_tags: {
                        let field_value = match fields_map.get("network_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organization: {
                        let field_value = match fields_map.get("organization") {
                            Some(value) => value,
                            None => bail!("Missing field 'organization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parent_asset_type: {
                        let field_value = match fields_map.get("parent_asset_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'parent_asset_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parent_full_resource_name: {
                        let field_value = match fields_map.get("parent_full_resource_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'parent_full_resource_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#update_time: {
                        let field_value = match fields_map.get("update_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'update_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
