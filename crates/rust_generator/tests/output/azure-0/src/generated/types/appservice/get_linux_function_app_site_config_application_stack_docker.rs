#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLinuxFunctionAppSiteConfigApplicationStackDocker {
    /// The name of the Docker image used.
    #[builder(into)]
    #[serde(rename = "imageName")]
    pub r#image_name: String,
    /// The image tag of the image used.
    #[builder(into)]
    #[serde(rename = "imageTag")]
    pub r#image_tag: String,
    /// The password for the account to use to connect to the registry.
    #[builder(into)]
    #[serde(rename = "registryPassword")]
    pub r#registry_password: String,
    /// The URL of the docker registry.
    #[builder(into)]
    #[serde(rename = "registryUrl")]
    pub r#registry_url: String,
    /// The username used for connections to the registry.
    #[builder(into)]
    #[serde(rename = "registryUsername")]
    pub r#registry_username: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLinuxFunctionAppSiteConfigApplicationStackDocker {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "image_name",
                    &self.r#image_name,
                ),
                to_pulumi_object_field(
                    "image_tag",
                    &self.r#image_tag,
                ),
                to_pulumi_object_field(
                    "registry_password",
                    &self.r#registry_password,
                ),
                to_pulumi_object_field(
                    "registry_url",
                    &self.r#registry_url,
                ),
                to_pulumi_object_field(
                    "registry_username",
                    &self.r#registry_username,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLinuxFunctionAppSiteConfigApplicationStackDocker {
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
                    r#image_name: {
                        let field_value = match fields_map.get("image_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_tag: {
                        let field_value = match fields_map.get("image_tag") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_tag' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#registry_password: {
                        let field_value = match fields_map.get("registry_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'registry_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#registry_url: {
                        let field_value = match fields_map.get("registry_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'registry_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#registry_username: {
                        let field_value = match fields_map.get("registry_username") {
                            Some(value) => value,
                            None => bail!("Missing field 'registry_username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
