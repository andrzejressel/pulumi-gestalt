#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineWorkloadDataflowFlexTemplateRequestLaunchParameterEnvironment {
    /// Additional experiment flags for the job.
    #[builder(into)]
    #[serde(rename = "additionalExperiments")]
    pub r#additional_experiments: Option<Vec<String>>,
    /// Additional user labels to be specified for the job. Keys and values should follow the restrictions specified in the labeling restrictions page. An object containing a list of key/value pairs.
    /// 'Example: { "name": "wrench", "mass": "1kg", "count": "3" }.'
    /// 'An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.'
    #[builder(into)]
    #[serde(rename = "additionalUserLabels")]
    pub r#additional_user_labels: Option<std::collections::HashMap<String, String>>,
    /// Whether to enable Streaming Engine for the job.
    #[builder(into)]
    #[serde(rename = "enableStreamingEngine")]
    pub r#enable_streaming_engine: Option<bool>,
    /// Set FlexRS goal for the job. https://cloud.google.com/dataflow/docs/guides/flexrs
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#FlexResourceSchedulingGoal
    /// Possible values are: `FLEXRS_UNSPECIFIED`, `FLEXRS_SPEED_OPTIMIZED`, `FLEXRS_COST_OPTIMIZED`.
    #[builder(into)]
    #[serde(rename = "flexrsGoal")]
    pub r#flexrs_goal: Option<String>,
    /// Configuration for VM IPs.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#WorkerIPAddressConfiguration
    /// Possible values are: `WORKER_IP_UNSPECIFIED`, `WORKER_IP_PUBLIC`, `WORKER_IP_PRIVATE`.
    #[builder(into)]
    #[serde(rename = "ipConfiguration")]
    pub r#ip_configuration: Option<String>,
    /// 'Name for the Cloud KMS key for the job. The key format is: projects//locations//keyRings//cryptoKeys/'
    #[builder(into)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Option<String>,
    /// The machine type to use for the job. Defaults to the value from the template if not specified.
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Option<String>,
    /// The maximum number of Compute Engine instances to be made available to your pipeline during execution, from 1 to 1000.
    #[builder(into)]
    #[serde(rename = "maxWorkers")]
    pub r#max_workers: Option<i32>,
    /// Network to which VMs will be assigned. If empty or unspecified, the service will use the network "default".
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// The initial number of Compute Engine instances for the job.
    #[builder(into)]
    #[serde(rename = "numWorkers")]
    pub r#num_workers: Option<i32>,
    /// The email address of the service account to run the job as.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Option<String>,
    /// Subnetwork to which VMs will be assigned, if desired. You can specify a subnetwork using either a complete URL or an abbreviated path. Expected to be of the form "https://www.googleapis.com/compute/v1/projects/HOST_PROJECT_ID/regions/REGION/subnetworks/SUBNETWORK" or "regions/REGION/subnetworks/SUBNETWORK". If the subnetwork is located in a Shared VPC network, you must use the complete URL.
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Option<String>,
    /// The Cloud Storage path to use for temporary files. Must be a valid Cloud Storage URL, beginning with gs://.
    #[builder(into)]
    #[serde(rename = "tempLocation")]
    pub r#temp_location: Option<String>,
    /// The Compute Engine region (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. "us-west1". Mutually exclusive with workerZone. If neither workerRegion nor workerZone is specified, default to the control plane's region.
    #[builder(into)]
    #[serde(rename = "workerRegion")]
    pub r#worker_region: Option<String>,
    /// The Compute Engine zone (https://cloud.google.com/compute/docs/regions-zones/regions-zones) in which worker processing should occur, e.g. "us-west1-a". Mutually exclusive with workerRegion. If neither workerRegion nor workerZone is specified, a zone in the control plane's region is chosen based on available capacity. If both workerZone and zone are set, workerZone takes precedence.
    #[builder(into)]
    #[serde(rename = "workerZone")]
    pub r#worker_zone: Option<String>,
    /// The Compute Engine availability zone for launching worker instances to run your pipeline. In the future, workerZone will take precedence.
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipelineWorkloadDataflowFlexTemplateRequestLaunchParameterEnvironment {
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
                    "additional_experiments",
                    &self.r#additional_experiments,
                ),
                to_pulumi_object_field(
                    "additional_user_labels",
                    &self.r#additional_user_labels,
                ),
                to_pulumi_object_field(
                    "enable_streaming_engine",
                    &self.r#enable_streaming_engine,
                ),
                to_pulumi_object_field(
                    "flexrs_goal",
                    &self.r#flexrs_goal,
                ),
                to_pulumi_object_field(
                    "ip_configuration",
                    &self.r#ip_configuration,
                ),
                to_pulumi_object_field(
                    "kms_key_name",
                    &self.r#kms_key_name,
                ),
                to_pulumi_object_field(
                    "machine_type",
                    &self.r#machine_type,
                ),
                to_pulumi_object_field(
                    "max_workers",
                    &self.r#max_workers,
                ),
                to_pulumi_object_field(
                    "network",
                    &self.r#network,
                ),
                to_pulumi_object_field(
                    "num_workers",
                    &self.r#num_workers,
                ),
                to_pulumi_object_field(
                    "service_account_email",
                    &self.r#service_account_email,
                ),
                to_pulumi_object_field(
                    "subnetwork",
                    &self.r#subnetwork,
                ),
                to_pulumi_object_field(
                    "temp_location",
                    &self.r#temp_location,
                ),
                to_pulumi_object_field(
                    "worker_region",
                    &self.r#worker_region,
                ),
                to_pulumi_object_field(
                    "worker_zone",
                    &self.r#worker_zone,
                ),
                to_pulumi_object_field(
                    "zone",
                    &self.r#zone,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipelineWorkloadDataflowFlexTemplateRequestLaunchParameterEnvironment {
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
                        let field_value = match fields_map.get("additional_experiments") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_experiments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#additional_user_labels: {
                        let field_value = match fields_map.get("additional_user_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_user_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_streaming_engine: {
                        let field_value = match fields_map.get("enable_streaming_engine") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_streaming_engine' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#flexrs_goal: {
                        let field_value = match fields_map.get("flexrs_goal") {
                            Some(value) => value,
                            None => bail!("Missing field 'flexrs_goal' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_configuration: {
                        let field_value = match fields_map.get("ip_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_name: {
                        let field_value = match fields_map.get("kms_key_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_type: {
                        let field_value = match fields_map.get("machine_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'machine_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_workers: {
                        let field_value = match fields_map.get("max_workers") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_workers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("num_workers") {
                            Some(value) => value,
                            None => bail!("Missing field 'num_workers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_email: {
                        let field_value = match fields_map.get("service_account_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("temp_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'temp_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#worker_region: {
                        let field_value = match fields_map.get("worker_region") {
                            Some(value) => value,
                            None => bail!("Missing field 'worker_region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#worker_zone: {
                        let field_value = match fields_map.get("worker_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'worker_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
