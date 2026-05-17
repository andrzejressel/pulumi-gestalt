#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceMetadata {
    /// Annotations is a key value map stored with a resource that
    /// may be set by external tools to store and retrieve arbitrary metadata.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations
    /// **Note**: The Cloud Run API may add additional annotations that were not provided in your config.
    /// If the provider plan shows a diff where a server-side annotation is added, you can add it to your config
    /// or apply the lifecycle.ignore_changes rule to the metadata.0.annotations field.
    /// Annotations with `run.googleapis.com/` and `autoscaling.knative.dev` are restricted. Use the following annotation
    /// keys to configure features on a Service:
    /// - `run.googleapis.com/binary-authorization-breakglass` sets the [Binary Authorization breakglass](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--breakglass).
    /// - `run.googleapis.com/binary-authorization` sets the [Binary Authorization](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--binary-authorization).
    /// - `run.googleapis.com/client-name` sets the client name calling the Cloud Run API.
    /// - `run.googleapis.com/custom-audiences` sets the [custom audiences](https://cloud.google.com/sdk/gcloud/reference/alpha/run/deploy#--add-custom-audiences)
    /// that can be used in the audience field of ID token for authenticated requests.
    /// - `run.googleapis.com/description` sets a user defined description for the Service.
    /// - `run.googleapis.com/ingress` sets the [ingress settings](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--ingress)
    /// for the Service. For example, `"run.googleapis.com/ingress" = "all"`.
    /// - `run.googleapis.com/launch-stage` sets the [launch stage](https://cloud.google.com/run/docs/troubleshooting#launch-stage-validation)
    /// when a preview feature is used. For example, `"run.googleapis.com/launch-stage": "BETA"`
    /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
    /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
    #[builder(into)]
    #[serde(rename = "annotations")]
    pub r#annotations: Option<std::collections::HashMap<String, String>>,
    #[builder(into)]
    #[serde(rename = "effectiveAnnotations")]
    pub r#effective_annotations: Option<std::collections::HashMap<String, String>>,
    /// (Output)
    /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
    #[builder(into)]
    #[serde(rename = "effectiveLabels")]
    pub r#effective_labels: Option<std::collections::HashMap<String, String>>,
    /// (Output)
    /// A sequence number representing a specific generation of the desired state.
    #[builder(into)]
    #[serde(rename = "generation")]
    pub r#generation: Option<i32>,
    /// Map of string keys and values that can be used to organize and categorize
    /// (scope and select) objects. May match selectors of replication controllers
    /// and routes.
    /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
    /// Please refer to the field `effective_labels` for all of the labels present on the resource.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// In Cloud Run the namespace must be equal to either the
    /// project ID or project number.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Option<String>,
    /// (Output)
    /// The combination of labels configured directly on the resource
    /// and default labels configured on the provider.
    #[builder(into)]
    #[serde(rename = "pulumiLabels")]
    pub r#pulumi_labels: Option<std::collections::HashMap<String, String>>,
    /// (Output)
    /// An opaque value that represents the internal version of this object that
    /// can be used by clients to determine when objects have changed. May be used
    /// for optimistic concurrency, change detection, and the watch operation on a
    /// resource or set of resources. They may only be valid for a
    /// particular resource or set of resources.
    #[builder(into)]
    #[serde(rename = "resourceVersion")]
    pub r#resource_version: Option<String>,
    /// (Output)
    /// SelfLink is a URL representing this object.
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: Option<String>,
    /// (Output)
    /// UID is a unique id generated by the server on successful creation of a resource and is not
    /// allowed to change on PUT operations.
    #[builder(into)]
    #[serde(rename = "uid")]
    pub r#uid: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceMetadata {
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
                "annotations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#annotations,
                )
                .await,
            );
            map.insert(
                "effective_annotations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#effective_annotations,
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
                "generation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#generation,
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
                "namespace".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#namespace,
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
            map.insert(
                "resource_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_version,
                )
                .await,
            );
            map.insert(
                "self_link".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#self_link,
                )
                .await,
            );
            map.insert(
                "uid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#uid,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceMetadata {
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
                    r#annotations: {
                        let field_value = match fields_map.get("annotations") {
                            Some(value) => value,
                            None => bail!("Missing field 'annotations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#effective_annotations: {
                        let field_value = match fields_map.get("effective_annotations") {
                            Some(value) => value,
                            None => bail!("Missing field 'effective_annotations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#generation: {
                        let field_value = match fields_map.get("generation") {
                            Some(value) => value,
                            None => bail!("Missing field 'generation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#namespace: {
                        let field_value = match fields_map.get("namespace") {
                            Some(value) => value,
                            None => bail!("Missing field 'namespace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#resource_version: {
                        let field_value = match fields_map.get("resource_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#self_link: {
                        let field_value = match fields_map.get("self_link") {
                            Some(value) => value,
                            None => bail!("Missing field 'self_link' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uid: {
                        let field_value = match fields_map.get("uid") {
                            Some(value) => value,
                            None => bail!("Missing field 'uid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
