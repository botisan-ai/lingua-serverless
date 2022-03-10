## Lingua Serverless

This is a serverless function that provides language detection API on AWS via [lingua-rs](https://github.com/pemistahl/lingua-rs).

## Setup

Make sure you have the [dockerless serverless-rust environment configured](https://github.com/softprops/serverless-rust#-experimental-local-builds).

Also, create a `.env` file to fill in the environment variables in `serverless.yml`.

It also attaches a custom domain to your serverless function. Please make sure to have an ACM certificate for the configured domain.

```shell
# install dependencies
yarn
cargo install --path .

# create domain
yarn serverless create_domain

# deploy
yarn serverless deploy
```
