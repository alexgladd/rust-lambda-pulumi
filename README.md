# Rust / Lambda / Pulumi

*rust-lambda-pulumi* is a Rust project that implements an AWS Lambda function in Rust using the Axum
web application framework. It uses Pulumi IaC to automate deployment to AWS.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo Lambda](https://www.cargo-lambda.info/guide/installation.html)
- [Pulumi IaC](https://www.pulumi.com/docs/iac/download-install/)
- [pnpm](https://pnpm.io/installation)

## Building

Run `pnpm build` to build the project.

This will use Cargo Lambda to build a Lambda-compatible `bootstrap` binary and package it into a
ZIP file that's ready for deployment.

## Deploying

Run `pnpm run deploy` to deploy the project to AWS.

You'll need to have AWS credentials [set up](https://www.pulumi.com/registry/packages/aws/installation-configuration/#credentials).
This will use Pulumi to create a Lambda function using the built Rust binary and link it to an API
Gateway. Pulumi will output the API Gateway's public endpoint once everything is deployed.

Don't worry, you can run `pnpm run destroy` to tear down all of the deployed resources!

## Testing

Once deployed, hit the root path on the API Gateway's public endpoint to get a list of available
paths (or just cheat and check out the [source](./src/app.rs)!).
