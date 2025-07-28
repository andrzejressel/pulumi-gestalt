use anyhow::{Context, Result};
use pulumi_gestalt_schema::deserialize_package_file;
use pulumi_gestalt_schema_protobuf::convert_to_protobuf;
use std::fs;
use std::path::{Path, PathBuf};
// DO NOT EDIT - START

#[test]
fn array_of_enum_map() -> Result<()> {
    run_pulumi_generator_test("array-of-enum-map", "array-of-enum-map", None)
}

#[test]
fn azure_native_nested_types() -> Result<()> {
    run_pulumi_generator_test("azure-native-nested-types", "azure-native-nested-types", None)
}

#[test]
fn cloudflare() -> Result<()> {
    run_pulumi_generator_test("cloudflare", "cloudflare", None)
}

#[test]
fn cyclic_types() -> Result<()> {
    run_pulumi_generator_test("cyclic-types", "cyclic-types", None)
}

#[test]
fn different_enum() -> Result<()> {
    run_pulumi_generator_test("different-enum", "different-enum", None)
}

#[test]
fn docker() -> Result<()> {
    run_pulumi_generator_test("docker", "docker", None)
}

#[test]
fn functions_secrets() -> Result<()> {
    run_pulumi_generator_test("functions-secrets", "functions-secrets", None)
}

#[test]
fn mini_awsnative() -> Result<()> {
    run_pulumi_generator_test("mini-awsnative", "mini-awsnative", None)
}

#[test]
fn nested_module() -> Result<()> {
    run_pulumi_generator_test("nested-module", "nested-module", None)
}

#[test]
fn nested_module_thirdparty() -> Result<()> {
    run_pulumi_generator_test("nested-module-thirdparty", "nested-module-thirdparty", None)
}

#[test]
fn output_funcs() -> Result<()> {
    run_pulumi_generator_test("output-funcs", "output-funcs", None)
}

#[test]
fn output_funcs_edgeorder() -> Result<()> {
    run_pulumi_generator_test("output-funcs-edgeorder", "output-funcs-edgeorder", None)
}

#[test]
fn plain_object_defaults() -> Result<()> {
    run_pulumi_generator_test("plain-object-defaults", "plain-object-defaults", None)
}

#[test]
fn plain_object_disable_defaults() -> Result<()> {
    run_pulumi_generator_test("plain-object-disable-defaults", "plain-object-disable-defaults", None)
}

#[test]
fn random() -> Result<()> {
    run_pulumi_generator_test("random", "random", None)
}

#[test]
fn reserved_names() -> Result<()> {
    run_pulumi_generator_test("reserved_names", "reserved_names", None)
}

#[test]
fn unions_inline() -> Result<()> {
    run_pulumi_generator_test("unions-inline", "unions-inline", None)
}

#[test]
fn unions_inside_arrays() -> Result<()> {
    run_pulumi_generator_test("unions-inside-arrays", "unions-inside-arrays", None)
}

#[test]
fn workarounds() -> Result<()> {
    run_pulumi_generator_test("workarounds", "workarounds", None)
}

#[test]
fn aws_0() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-0", Some(&["accessanalyzer","account","acm","acmpca","alb","amp","amplify","apigateway","apigatewayv2","appautoscaling"]))
}

#[test]
fn aws_1() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-1", Some(&["appconfig","appfabric","appflow","appintegrations","applicationinsights","applicationloadbalancing","appmesh","apprunner","appstream","appsync"]))
}

#[test]
fn aws_2() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-2", Some(&["athena","auditmanager","autoscaling","autoscalingplans","backup","batch","bcmdata","bedrock","bedrockfoundation","bedrockmodel"]))
}

#[test]
fn aws_3() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-3", Some(&["budgets","cfg","chatbot","chime","chimesdkmediapipelines","cleanrooms","cloud9","cloudcontrol","cloudformation","cloudfront"]))
}

#[test]
fn aws_4() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-4", Some(&["cloudhsmv2","cloudsearch","cloudtrail","cloudwatch","codeartifact","codebuild","codecatalyst","codecommit","codeconnections","codedeploy"]))
}

#[test]
fn aws_5() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-5", Some(&["codeguruprofiler","codegurureviewer","codepipeline","codestarconnections","codestarnotifications","cognito","comprehend","computeoptimizer","config","connect"]))
}

#[test]
fn aws_6() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-6", Some(&["controltower","costexplorer","costoptimizationhub","cur","customerprofiles","dataexchange","datapipeline","datasync","datazone","dax"]))
}

#[test]
fn aws_7() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-7", Some(&["detective","devicefarm","devopsguru","directconnect","directoryservice","dlm","dms","docdb","drs","dynamodb"]))
}

#[test]
fn aws_8() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-8", Some(&["ebs","ec2","ec2clientvpn","ec2transitgateway","ecr","ecrpublic","ecs","efs","eks","elasticache"]))
}

#[test]
fn aws_9() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-9", Some(&["elasticbeanstalk","elasticsearch","elastictranscoder","elb","emr","emrcontainers","emrserverless","evidently","finspace","fis"]))
}

#[test]
fn aws_10() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-10", Some(&["fms","fsx","gamelift","glacier","globalaccelerator","glue","grafana","guardduty","iam","identitystore"]))
}

#[test]
fn aws_11() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-11", Some(&["imagebuilder","index","inspector","inspector2","iot","ivs","ivschat","kendra","keyspaces","kinesis"]))
}

#[test]
fn aws_12() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-12", Some(&["kinesisanalyticsv2","kms","lakeformation","lambda","lb","lex","licensemanager","lightsail","location","m2"]))
}

#[test]
fn aws_13() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-13", Some(&["macie","macie2","mediaconvert","medialive","mediapackage","mediastore","memorydb","mq","msk","mskconnect"]))
}

#[test]
fn aws_14() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-14", Some(&["mwaa","neptune","networkfirewall","networkmanager","networkmonitor","oam","opensearch","opensearchingest","opsworks","organizations"]))
}

#[test]
fn aws_15() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-15", Some(&["outposts","paymentcryptography","pinpoint","pipes","polly","pricing","qldb","quicksight","ram","rbin"]))
}

#[test]
fn aws_16() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-16", Some(&["rds","redshift","redshiftdata","redshiftserverless","rekognition","resiliencehub","resourceexplorer","resourcegroups","resourcegroupstaggingapi","rolesanywhere"]))
}

#[test]
fn aws_17() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-17", Some(&["route53","route53domains","route53recoverycontrol","route53recoveryreadiness","rum","s3","s3control","s3outposts","s3tables","sagemaker"]))
}

#[test]
fn aws_18() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-18", Some(&["scheduler","schemas","secretsmanager","securityhub","securitylake","serverlessrepository","servicecatalog","servicediscovery","servicequotas","ses"]))
}

#[test]
fn aws_19() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-19", Some(&["sesv2","sfn","shield","signer","simpledb","sns","sqs","ssm","ssmcontacts","ssmincidents"]))
}

#[test]
fn aws_20() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-20", Some(&["ssoadmin","storagegateway","swf","synthetics","timestreaminfluxdb","timestreamwrite","transcribe","transfer","verifiedaccess","verifiedpermissions"]))
}

#[test]
fn aws_21() -> Result<()> {
    run_pulumi_generator_test("aws", "aws-21", Some(&["vpc","vpclattice","waf","wafregional","wafv2","worklink","workspaces","xray"]))
}

#[test]
fn azure_0() -> Result<()> {
    run_pulumi_generator_test("azure", "azure-0", Some(&["aadb2c","advisor","analysisservices","apimanagement","appconfiguration","appinsights","appplatform","appservice","arc","arckubernetes"]))
}

#[test]
fn azure_1() -> Result<()> {
    run_pulumi_generator_test("azure", "azure-1", Some(&["arcmachine","armmsi","attestation","authorization","automanage","automation","avs","backup","batch","billing"]))
}

#[test]
fn azure_2() -> Result<()> {
    run_pulumi_generator_test("azure", "azure-2", Some(&["blueprint","bot","cdn","chaosstudio","cognitive","communication","compute","confidentialledger","config","connections"]))
}

#[test]
fn azure_3() -> Result<()> {
    run_pulumi_generator_test("azure", "azure-3", Some(&["consumption","containerapp","containerservice","core","cosmosdb","costmanagement","customip","dashboard","databasemigration","databoxedge"]))
}

#[test]
fn azure_4() -> Result<()> {
    run_pulumi_generator_test("azure", "azure-4", Some(&["databricks","datadog","datafactory","dataprotection","datashare","desktopvirtualization","devcenter","devtest","digitaltwins","dns"]))
}

#[test]
fn azure_5() -> Result<()> {
    run_pulumi_generator_test("azure", "azure-5", Some(&["domainservices","dynatrace","elasticcloud","elasticsan","eventgrid","eventhub","expressroute","extendedlocation","fabric","fluidrelay"]))
}

#[test]
fn azure_6() -> Result<()> {
    run_pulumi_generator_test("azure", "azure-6", Some(&["frontdoor","graph","hdinsight","healthcare","hpc","hsm","index","iot","iotcentral","keyvault"]))
}

#[test]
fn azure_7() -> Result<()> {
    run_pulumi_generator_test("azure", "azure-7", Some(&["kusto","lb","lighthouse","loadtest","loganalytics","logicapps","machinelearning","maintenance","managedapplication","managedlustre"]))
}

#[test]
fn azure_8() -> Result<()> {
    run_pulumi_generator_test("azure", "azure-8", Some(&["management","managementgroups","managementresource","maps","marketplace","mixedreality","mobile","monitoring","msi","mssql"]))
}

#[test]
fn azure_9() -> Result<()> {
    run_pulumi_generator_test("azure", "azure-9", Some(&["mysql","netapp","network","networkfunction","newrelic","nginx","notificationhub","operationalinsights","oracle","orbital"]))
}

#[test]
fn azure_10() -> Result<()> {
    run_pulumi_generator_test("azure", "azure-10", Some(&["paloalto","pim","policy","portal","postgresql","powerbi","privatedns","privatelink","proximity","purview"]))
}

#[test]
fn azure_11() -> Result<()> {
    run_pulumi_generator_test("azure", "azure-11", Some(&["recoveryservices","redhatopenshift","redis","relay","role","search","securitycenter","sentinel","servicebus","servicefabric"]))
}

#[test]
fn azure_12() -> Result<()> {
    run_pulumi_generator_test("azure", "azure-12", Some(&["signalr","siterecovery","stack","storage","streamanalytics","synapse","systemcenter","trafficmanager","trustedsigning","videoindexer"]))
}

#[test]
fn azure_13() -> Result<()> {
    run_pulumi_generator_test("azure", "azure-13", Some(&["voice","waf","webpubsub","workloadssap"]))
}

#[test]
fn filtering_0() -> Result<()> {
    run_pulumi_generator_test("filtering", "filtering-0", Some(&["ns1"]))
}

#[test]
fn filtering_1() -> Result<()> {
    run_pulumi_generator_test("filtering", "filtering-1", Some(&["ns2"]))
}

#[test]
fn filtering_2() -> Result<()> {
    run_pulumi_generator_test("filtering", "filtering-2", Some(&["ns1","ns2"]))
}

#[test]
fn gcp_0() -> Result<()> {
    run_pulumi_generator_test("gcp", "gcp-0", Some(&["accessapproval","accesscontextmanager","activedirectory","alloydb","apigateway","apigee","appengine","apphub","applicationintegration","artifactregistry"]))
}

#[test]
fn gcp_1() -> Result<()> {
    run_pulumi_generator_test("gcp", "gcp-1", Some(&["assuredworkloads","backupdisasterrecovery","beyondcorp","biglake","bigquery","bigqueryanalyticshub","bigquerydatapolicy","bigtable","billing","binaryauthorization"]))
}

#[test]
fn gcp_2() -> Result<()> {
    run_pulumi_generator_test("gcp", "gcp-2", Some(&["blockchainnodeengine","certificateauthority","certificatemanager","cloudasset","cloudbuild","cloudbuildv2","clouddeploy","clouddomains","cloudfunctions","cloudfunctionsv2"]))
}

#[test]
fn gcp_3() -> Result<()> {
    run_pulumi_generator_test("gcp", "gcp-3", Some(&["cloudidentity","cloudids","cloudquota","cloudrun","cloudrunv2","cloudscheduler","cloudtasks","composer","config","container"]))
}

#[test]
fn gcp_4() -> Result<()> {
    run_pulumi_generator_test("gcp", "gcp-4", Some(&["containeranalysis","databasemigrationservice","datacatalog","dataflow","dataform","datafusion","dataloss","dataplex","dataproc","datastream"]))
}

#[test]
fn gcp_5() -> Result<()> {
    run_pulumi_generator_test("gcp", "gcp-5", Some(&["deploymentmanager","developerconnect","diagflow","discoveryengine","dns","edgecontainer","edgenetwork","endpoints","essentialcontacts","eventarc"]))
}

#[test]
fn gcp_6() -> Result<()> {
    run_pulumi_generator_test("gcp", "gcp-6", Some(&["filestore","firebase","firebaserules","firestore","folder","gemini","gkebackup","gkehub","gkeonprem","healthcare"]))
}

#[test]
fn gcp_7() -> Result<()> {
    run_pulumi_generator_test("gcp", "gcp-7", Some(&["iam","iap","identityplatform","index","integrationconnectors","kms","logging","looker","managedkafka","memcache"]))
}

#[test]
fn gcp_8() -> Result<()> {
    run_pulumi_generator_test("gcp", "gcp-8", Some(&["memorystore","migrationcenter","ml","monitoring","netapp","networkconnectivity","networkmanagement","networksecurity","networkservices","notebooks"]))
}

#[test]
fn gcp_9() -> Result<()> {
    run_pulumi_generator_test("gcp", "gcp-9", Some(&["oracledatabase","organizations","orgpolicy","osconfig","oslogin","parallelstore","privilegedaccessmanager","projects","pubsub","recaptcha"]))
}

#[test]
fn gcp_10() -> Result<()> {
    run_pulumi_generator_test("gcp", "gcp-10", Some(&["redis","resourcemanager","runtimeconfig","secretmanager","securesourcemanager","securitycenter","securityposture","serviceaccount","servicedirectory","servicenetworking"]))
}

#[test]
fn gcp_11() -> Result<()> {
    run_pulumi_generator_test("gcp", "gcp-11", Some(&["serviceusage","siteverification","sourcerepo","spanner","sql","storage","tags","tpu","transcoder","vertex"]))
}

#[test]
fn gcp_12() -> Result<()> {
    run_pulumi_generator_test("gcp", "gcp-12", Some(&["vmwareengine","vpcaccess","workbench","workflows","workstations"]))
}

#[test]
fn gcp_13() -> Result<()> {
    run_pulumi_generator_test("gcp", "gcp-13", Some(&["compute"]))
}
// DO NOT EDIT - END

pub fn run_pulumi_generator_test(
    schema_name: &str,
    directory_name: &str,
    modules: Option<&[&str]>,
) -> Result<()> {
    let root_path = format!("tests/output/{directory_name}");
    let root = Path::new(&root_path);

    let schema = find_schema_files(schema_name);
    fs::create_dir_all(root)?;

    create_symlink(&schema, &root.join(schema.file_name().unwrap()))?;

    let package = deserialize_package_file(schema.as_path(), modules)
        .context("Failed to deserialize package")?;

    let protobuf =
        convert_to_protobuf(&package).context("Failed to convert package to protobuf")?;

    let result_file = root.join("package.pb");

    fs::write(result_file, protobuf)?;

    Ok(())
}

pub fn find_schema_files(name: &str) -> PathBuf {
    let possible_paths = vec![
        Path::new("tests/test_cases").join(format!("{name}.json")),
        Path::new("../../providers").join(format!("{name}.json")),
        Path::new("../../external/pulumi/tests/testdata/codegen")
            .join(name)
            .join("schema.yaml"),
        Path::new("../../external/pulumi/tests/testdata/codegen")
            .join(name)
            .join("schema.json"),
        Path::new("../../external/pulumi/tests/testdata/codegen").join(format!("{name}.yaml")),
        Path::new("../../external/pulumi/tests/testdata/codegen").join(format!("{name}.json")),
        Path::new("../../external/pulumi-java/pkg/codegen/testing/test/testdata")
            .join(name)
            .join("schema.yaml"),
        Path::new("../../external/pulumi-java/pkg/codegen/testing/test/testdata")
            .join(name)
            .join("schema.json"),
        Path::new("../../external/pulumi-java/pkg/codegen/testing/test/testdata")
            .join(format!("{name}.yaml")),
        Path::new("../../external/pulumi-java/pkg/codegen/testing/test/testdata")
            .join(format!("{name}.json")),
    ];

    for path in possible_paths {
        if path.exists() {
            return path;
        }
    }

    panic!("No schema file found for provider: {name}");
}

fn create_symlink(src: &Path, dst: &Path) -> std::io::Result<()> {
    if dst.exists() {
        fs::remove_file(dst)?;
    }
    use pathdiff::diff_paths;
    let relative_path = diff_paths(src, dst.parent().unwrap()).unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(&relative_path, dst)?;
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(&relative_path, dst)?;
    Ok(())
}
