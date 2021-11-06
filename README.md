# Rust Serverless Lambda Template

![Rust](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)
![Serverless](https://img.shields.io/badge/Serverless-black?style=for-the-badge&logo=serverless&logoColor=#E57324)
![AWS Lambda](https://img.shields.io/badge/AWS_Lambda-232F3E?style=for-the-badge&logo=amazonaws&logoColor=white)
![NPM](https://img.shields.io/badge/npm-CB3837?style=for-the-badge&logo=npm&logoColor=white)

---

![Repo Size](https://img.shields.io/github/repo-size/sex-request/rust-serverless-lambda-template)
![Stars](https://img.shields.io/github/stars/sex-request/rust-serverless-lambda-template?style=social)

This setting used by [This Repository](https://github.com/sex-request/backend)

## Default Setting

- Rust
  - lambda_http
  - tokio
  - serde_json
- Serverless
  - serverless-rust
  - dockerless
    - default docker image's rust version is 1.43.1
    - before 1.50 caused compile error
- Using husky
  - pre-commit : cargo fmt

## Should Required Settings

- [Install aws cli](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)
  - setting `aws configure`
- [serverless-rust](https://www.serverless.com/plugins/serverless-rust) should setting `x86_64-unknown-linux-musl`
  - if you use `dockerless` option

## [ IMPORTANT ] Must Change Here

- `package.json` line 2 - project name
- `serverless.yml` line 1 - service
- `serverless.yml` line 3 - provider
- `serverless.yml` line 21 - functions

## Usage

### Install

```sh
npm ci && npm run postinstall && npm run fmt:install
```

### Test

```sh
npm run test # cargo test
```

### Deploy

```sh
npm run deploy:dev
npm run deploy:qa
npm run deploy:prod
```

### Formatting

```sh
npm run fmt:install # install rustfmt
npm run fmt # formatting
npm run fmt:check # check lint
```
