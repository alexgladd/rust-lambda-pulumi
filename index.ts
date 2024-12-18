import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
// import * as awsx from "@pulumi/awsx";

// The following resources appear to be the bare minimum required to set up an HTTP APIGWv2 and get
// it integrated with a Lambda function

// The policy that allows a lambda function to assume a role
const assumeRole = aws.iam.getPolicyDocument({
  statements: [{
      effect: "Allow",
      actions: ["sts:AssumeRole"],
      principals: [{
          type: "Service",
          identifiers: ["lambda.amazonaws.com"],
      }],
  }],
});

// Use for building inline policy for resources the lambda function needs to access
// const policy = aws.iam.getPolicyDocument({
//   statements: []
// });

// The role for our function. Uses our assumeRole policy and our inline policy
const role = new aws.iam.Role("RustLambdaRole", {
  assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
  managedPolicyArns: [aws.iam.ManagedPolicy.AWSLambdaBasicExecutionRole],
  // inlinePolicies: [{
  //   policy: policy.then(inlinePolicy => inlinePolicy.json),
  // }],
});

// The lambda function itself
const func = new aws.lambda.Function("RustLambda", {
  architectures: ["x86_64"],
  runtime: aws.lambda.Runtime.CustomAL2023,
  role: role.arn,
  handler: "bootstrap",
  code: new pulumi.asset.FileArchive("./target/lambda/rust-lambda/bootstrap.zip"),
});

// The APIGWv2 for HTTP
const api = new aws.apigatewayv2.Api("RustLambdaApi", {
  protocolType: "HTTP",
});

// A lambda function permission that allows our API gateway to invoke the function
const funcExec = new aws.lambda.Permission("RustLambdaExec", {
  function: func.name,
  action: "lambda:InvokeFunction",
  principal: "apigateway.amazonaws.com",
  sourceArn: pulumi.interpolate `${api.executionArn}/*`
});

// All API gateways need at least one stage; the "$default" name is special and used as a fallback
const stage = new aws.apigatewayv2.Stage("RustLambdaApiStage", {
  apiId: api.id,
  name: "$default",
  autoDeploy: true,
});

// The integration that actually tells our API to invoke our function
const handler = new aws.apigatewayv2.Integration("RustLambdaApiIntegration", {
  apiId: api.id,
  integrationType: "AWS_PROXY",
  integrationMethod: "POST",
  integrationUri: func.invokeArn,
  payloadFormatVersion: "2.0",
});

// The route configuration that uses our integration
const route = new aws.apigatewayv2.Route("RustLambdaProxyRoute", {
  apiId: api.id,
  routeKey: "ANY /{proxy+}",
  target: pulumi.interpolate `integrations/${handler.id}`,
});

// exports
export const endpoint = api.apiEndpoint;
