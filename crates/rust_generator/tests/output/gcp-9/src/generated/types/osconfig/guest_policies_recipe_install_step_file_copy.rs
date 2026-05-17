#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuestPoliciesRecipeInstallStepFileCopy {
    /// The id of the relevant artifact in the recipe.
    #[builder(into)]
    #[serde(rename = "artifactId")]
    pub r#artifact_id: String,
    /// The absolute path on the instance to put the file.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: String,
    /// Whether to allow this step to overwrite existing files.If this is false and the file already exists the file
    /// is not overwritten and the step is considered a success. Defaults to false.
    #[builder(into)]
    #[serde(rename = "overwrite")]
    pub r#overwrite: Option<bool>,
    /// Consists of three octal digits which represent, in order, the permissions of the owner, group, and other users
    /// for the file (similarly to the numeric mode used in the linux chmod utility). Each digit represents a three bit
    /// number with the 4 bit corresponding to the read permissions, the 2 bit corresponds to the write bit, and the one
    /// bit corresponds to the execute permission. Default behavior is 755.
    /// Below are some examples of permissions and their associated values:
    /// read, write, and execute: 7 read and execute: 5 read and write: 6 read only: 4
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GuestPoliciesRecipeInstallStepFileCopy {
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
                "artifact_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#artifact_id,
                )
                .await,
            );
            map.insert(
                "destination".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination,
                )
                .await,
            );
            map.insert(
                "overwrite".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#overwrite,
                )
                .await,
            );
            map.insert(
                "permissions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#permissions,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GuestPoliciesRecipeInstallStepFileCopy {
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
                    r#artifact_id: {
                        let field_value = match fields_map.get("artifact_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'artifact_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination: {
                        let field_value = match fields_map.get("destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#overwrite: {
                        let field_value = match fields_map.get("overwrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'overwrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#permissions: {
                        let field_value = match fields_map.get("permissions") {
                            Some(value) => value,
                            None => bail!("Missing field 'permissions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
