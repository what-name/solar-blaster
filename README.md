## Solar Blaster
This code is used to (hypothetically) blast the Solana blockchain with transactions. It's very early stage and there's really no point to this other than "can you do it".

**Again, there is absolutely no point to this, and you should not use this for anything.**
It's mainly a learning project for me especially on how to interact with Solana using the Rust Solana SDK, plus some fun Docker & Terraform on how you'd execute it at scale. Do not overload the devnet if you may.

## Usage
Create a `keys` directory at the root with keys named `0.json - 9.json` using the following command:
```
mkdir keys
solana-keygen new --outfile keys/0.json
```
Airdrop some SOL to each of those keys
```
solana address -k keys/0.json
solana airdrop 2 <public_key_here>
```


One of these keys will be randomly selected at the container's launch to be used as a source, while the others will be used as target, changing at every 10th transfer. For now the Dockerfile builds in the keys, which is terrible practice but there are no real funds at stake here so.

To run locally, execute `cargo run` from the root of the project.

## Terraform
To deploy the Terraform code, you will need to have AWS credentials in your env, or modify the code to use a profile.

**If you deploy this, please destroy it after it's not needed. You'd be spamming the network for no reason other than forgetfulness.**

```
$ terraform --version == 1.0.11

cd terraform
terraform init
terraform plan
terraform apply
terraform destroy
```
