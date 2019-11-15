# TinyGW: An Rust Example of a Brigade Gateway

This is a sample [Brigade](https://brigade.sh) gateway written in Rust. It triggers a Brigade `interval` event every 5 minutes. The repository also contains some samples you can use for testing this gateway.

## Using This Repo

Assuming you have Brigade installed and configured on a Kubernetes cluster to which you have access, you can use this demo as follows:

1. Clone this repo
2. Install the project: `kubectl create -f project.yaml`
3. Export your project name for the gateway: `export PROJECT=brigade-3fe1406a8254afd471de2bdd53483501f947004cd3d174e6a60764`
4. Start the gateway: `cargo run`

Every few minutes, this gateway will create a new `interval` event. The project you created in Step 2 will handle that event by printing a log to the console. If you are using Kashti, you'll see it. Otherwise, you can use `brig build list` every few minutes to see the new build, and `brig build logs --last` to see the output of the last run.

## To Modify the Brigade Script

The Brigade script is stored in the `project.yaml`. When you edit it, you will need to edit the project in your cluster.

## To Modify the Gateway

Edit the code in `main.rs` and then re-run `cargo run`

## The TinyGW Helm Chart

If you want to run the gateway inside of your cluster, you can `make build`, which will build and push the Docker image, and then install it via the Helm 3 chart.

```console
$ export TAG=myrepo/tinygw:latest
$ make build
$ helm install tinygw ./charts/tinygw --set image.repository=$TAG
```