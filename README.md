# Image Recognition based Security System Project in Rust

![Hawk](https://s3.ap-south-1.amazonaws.com/uploadbucket1234/Screenshot+from+2019-04-18+17-15-35.png)

This is an Image Recognition project that uses Rust as a primary language and Java language for writing AWS Lambda function that further uses the AWS Rekognition service to send back the level of image similarity. The project can be run on the Raspberry Pi by cross compiling the existing project, details of which are given in the Readme.

We thrive for the best and want you to contribute towards a better Project. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for giving your valuable feedbacks and contributions.

## Setting up your environment

### Rustup.rs

Building this project requires [rustup](https://rustup.rs/), version 1.8.0 or more recent.
If you have an older version, run `rustup self update`.

To install on Windows, download and run [`rustup-init.exe`](https://win.rustup.rs/)
then follow the onscreen instructions.

To install on other systems, run:

```
curl https://sh.rustup.rs -sSf | sh
```

This will also download the current stable version of Rust, which this project won’t use.
To skip that step, run instead:

```
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain none
```
### Kafka

#### Download Kafka

```
tar -xzf kafka_2.12-2.2.0.tgz
cd kafka_2.12-2.2.0
```

#### Start the server

```
bin/zookeeper-server-start.sh config/zookeeper.properties
```

#### Start the Kafka server

```
bin/kafka-server-start.sh config/server.properties
```

#### Create a topic

This creates a topic named `my-topic`

```
bin/kafka-topics.sh --create --bootstrap-server localhost:9092 --replication-factor 1 --partitions 1 --topic my-topic
```

The Kafka server should be up and running when you want to run the application.

For more information on Kafka, refer to the Kafka Quickstart [`KAFKA QUICKSTART`](https://kafka.apache.org/quickstart)

### Exporting variables

`REGION` is required by AWS which should match your Bucket's region.
You are required to add the same region in which you have your Bucket.

```
export Region=$YOUR_REGION
```

`BUCKET_NAME` is the bucket that stores all the clicked images.

```
export Clicked_Image_Bucket=$BUCKET_NAME
```

`CLICKED_IMAGE_PATH` is the path to the clicked image that is taken by the camera of Raspberry Pi.

```
export Clicked_Image_Path=$CLICKED_IMAGE_PATH
```

`API_URL` contains the API Gateway URL that is hit from this application and gets the result from AWS Lambda.

```
export Lambda_End_Point=$API_URL
```

`my-group` and `my-topic` are the two variables required by Kafka.
```
export Consumers=my-group
export Topic_Name=my-topic
```

`IP_ADDRESS:PORT_NO` refers to the socket where the Kafka runs.

```
export Host=$IP_ADDRESS:PORT_NO
```

`RUST_LOG` is required for loggers.

```
export RUST_LOG=hawk=info
```


## Building

### Normal Build

```
git clone https://github.com/KnoldusLabs/Hawk
cd Hawk
cargo build
```

The binary would be saved in `/target/debug/hawk`

### Cross-compilation build for Raspberry Pi

Run `cargo build --target=armv7-unknown-linux-gnueabihf` to get a cross compiled binary in `/target/armv7-unknown-linux-gnueabihf/debug/hawk`

## Running the binary

```
./hawk
```

Then follow the instructions in the application.
