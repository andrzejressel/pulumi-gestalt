#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTemplateSpecVolumeSecret {
    /// Mode bits to use on created files by default. Must be a value between 0000
    /// and 0777. Defaults to 0644. Directories within the path are not affected by
    /// this setting. This might be in conflict with other options that affect the
    /// file mode, like fsGroup, and the result can be other mode bits set.
    #[builder(into)]
    #[serde(rename = "defaultMode")]
    pub r#default_mode: Option<i32>,
    /// If unspecified, the volume will expose a file whose name is the
    /// secret_name.
    /// If specified, the key will be used as the version to fetch from Cloud
    /// Secret Manager and the path will be the name of the file exposed in the
    /// volume. When items are defined, they must specify a key and a path.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "items")]
    pub r#items: Option<Vec<super::super::types::cloudrun::ServiceTemplateSpecVolumeSecretItem>>,
    /// The name of the secret in Cloud Secret Manager. By default, the secret
    /// is assumed to be in the same project.
    /// If the secret is in another project, you must define an alias.
    /// An alias definition has the form:
    /// {alias}:projects/{project-id|project-number}/secrets/{secret-name}.
    /// If multiple alias definitions are needed, they must be separated by
    /// commas.
    /// The alias definitions must be set on the run.googleapis.com/secrets
    /// annotation.
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceTemplateSpecVolumeSecret {
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
                    "default_mode",
                    &self.r#default_mode,
                ),
                to_pulumi_object_field(
                    "items",
                    &self.r#items,
                ),
                to_pulumi_object_field(
                    "secret_name",
                    &self.r#secret_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceTemplateSpecVolumeSecret {
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
                    r#default_mode: {
                        let field_value = match fields_map.get("default_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#items: {
                        let field_value = match fields_map.get("items") {
                            Some(value) => value,
                            None => bail!("Missing field 'items' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_name: {
                        let field_value = match fields_map.get("secret_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
