#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineWorkloadDataflowLaunchTemplateRequestLaunchParametersEnvironment {
    /// Additional experiment flags for the job.
    #[builder(into)]
    pub r#additional_experiments: Option<Vec<String>>,
    /// Additional user labels to be specified for the job. Keys and values should follow the restrictions specified in the labeling restrictions page. An object containing a list of key/value pairs.
    /// 'Example: { "name": "wrench", "mass": "1kg", "count": "3" }.'
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into)]
    pub r#additional_user_labels: Option<std::collections::BTreeMap<String, String>>,
    /// Whether to bypass the safety checks for the job's temporary directory. Use with caution.
    #[builder(into)]
    pub r#bypass_temp_dir_validation: Option<bool>,
    /// Whether to enable Streaming Engine for the job.
    #[builder(into)]
    pub r#enable_streaming_engine: Option<bool>,
    /// Configuration for VM IPs.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#WorkerIPAddressConfiguration
    /// Possible values are: `WORKER_IP_UNSPECIFIED`, `WORKER_IP_PUBLIC`, `WORKER_IP_PRIVATE`.
    #[builder(into)]
    pub r#ip_configuration: Option<String>,
    /// 'Name for the Cloud KMS key for the job. The key format is: projects//locations//keyRings//cryptoKeys/'
    #[builder(into)]
    pub r#kms_key_name: Option<String>,
    /// The machine type to use for the job. Defaults to the value from the template if not specified.
    #[builder(into)]
    pub r#machine_type: Option<String>,
    /// The maximum number of Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000.
    #[builder(into)]
    pub r#max_workers: Option<i32>,
    /// Network to which VMs will be assigned. If empty or unspecified, the service will use the network "default".
    #[builder(into)]
    pub r#network: Option<String>,
    /// The initial number of Compute Engine instances for the job.
    #[builder(into)]
    pub r#num_workers: Option<i32>,
    /// The email address of the service account to run the job as.
    #[builder(into)]
    pub r#service_account_email: Option<String>,
    /// Subnetwork to which VMs will be assigned, if desired. You can specify a subnetwork using either a complete URL or an abbreviated path. Expected to be of the form "https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK" or "regions/REGION/subnetworks/SUBNETWORK". If the subnetwork is located in a Shared VPC network, you must use the complete URL.
    #[builder(into)]
    pub r#subnetwork: Option<String>,
    /// The Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with gs://.
    #[builder(into)]
    pub r#temp_location: Option<String>,
    /// The Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. "us-west1". Mutually exclusive with workerZone. If neither workerRegion nor workerZone is specified, default to the control plane's region.
    #[builder(into)]
    pub r#worker_region: Option<String>,
    /// The Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. "us-west1-a". Mutually exclusive with workerRegion. If neither workerRegion nor workerZone is specified, a zone in the control plane's region is chosen based on available capacity. If both workerZone and zone are set, workerZone takes precedence.
    #[builder(into)]
    pub r#worker_zone: Option<String>,
    /// The Compute Engine availability zone for launching worker instances to run your pipeline. In the future, workerZone will take precedence.
    #[builder(into)]
    pub r#zone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipelineWorkloadDataflowLaunchTemplateRequestLaunchParametersEnvironment {
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
                    "additionalExperiments",
                    &self.r#additional_experiments,
                ),
                to_pulumi_object_field(
                    "additionalUserLabels",
                    &self.r#additional_user_labels,
                ),
                to_pulumi_object_field(
                    "bypassTempDirValidation",
                    &self.r#bypass_temp_dir_validation,
                ),
                to_pulumi_object_field(
                    "enableStreamingEngine",
                    &self.r#enable_streaming_engine,
                ),
                to_pulumi_object_field(
                    "ipConfiguration",
                    &self.r#ip_configuration,
                ),
                to_pulumi_object_field(
                    "kmsKeyName",
                    &self.r#kms_key_name,
                ),
                to_pulumi_object_field(
                    "machineType",
                    &self.r#machine_type,
                ),
                to_pulumi_object_field(
                    "maxWorkers",
                    &self.r#max_workers,
                ),
                to_pulumi_object_field(
                    "network",
                    &self.r#network,
                ),
                to_pulumi_object_field(
                    "numWorkers",
                    &self.r#num_workers,
                ),
                to_pulumi_object_field(
                    "serviceAccountEmail",
                    &self.r#service_account_email,
                ),
                to_pulumi_object_field(
                    "subnetwork",
                    &self.r#subnetwork,
                ),
                to_pulumi_object_field(
                    "tempLocation",
                    &self.r#temp_location,
                ),
                to_pulumi_object_field(
                    "workerRegion",
                    &self.r#worker_region,
                ),
                to_pulumi_object_field(
                    "workerZone",
                    &self.r#worker_zone,
                ),
                to_pulumi_object_field(
                    "zone",
                    &self.r#zone,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipelineWorkloadDataflowLaunchTemplateRequestLaunchParametersEnvironment {
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
                    r#additional_experiments: {
                        let field_value = match fields_map.get("additionalExperiments") {
                            Some(value) => value,
                            None => bail!("Missing field 'additionalExperiments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#additional_user_labels: {
                        let field_value = match fields_map.get("additionalUserLabels") {
                            Some(value) => value,
                            None => bail!("Missing field 'additionalUserLabels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bypass_temp_dir_validation: {
                        let field_value = match fields_map.get("bypassTempDirValidation") {
                            Some(value) => value,
                            None => bail!("Missing field 'bypassTempDirValidation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_streaming_engine: {
                        let field_value = match fields_map.get("enableStreamingEngine") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableStreamingEngine' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_configuration: {
                        let field_value = match fields_map.get("ipConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_name: {
                        let field_value = match fields_map.get("kmsKeyName") {
                            Some(value) => value,
                            None => bail!("Missing field 'kmsKeyName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_type: {
                        let field_value = match fields_map.get("machineType") {
                            Some(value) => value,
                            None => bail!("Missing field 'machineType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_workers: {
                        let field_value = match fields_map.get("maxWorkers") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxWorkers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network: {
                        let field_value = match fields_map.get("network") {
                            Some(value) => value,
                            None => bail!("Missing field 'network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#num_workers: {
                        let field_value = match fields_map.get("numWorkers") {
                            Some(value) => value,
                            None => bail!("Missing field 'numWorkers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_email: {
                        let field_value = match fields_map.get("serviceAccountEmail") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAccountEmail' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnetwork: {
                        let field_value = match fields_map.get("subnetwork") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetwork' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#temp_location: {
                        let field_value = match fields_map.get("tempLocation") {
                            Some(value) => value,
                            None => bail!("Missing field 'tempLocation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#worker_region: {
                        let field_value = match fields_map.get("workerRegion") {
                            Some(value) => value,
                            None => bail!("Missing field 'workerRegion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#worker_zone: {
                        let field_value = match fields_map.get("workerZone") {
                            Some(value) => value,
                            None => bail!("Missing field 'workerZone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zone: {
                        let field_value = match fields_map.get("zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
