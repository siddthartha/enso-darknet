# enso-ml-net

A PoC of simple asynchronuous json API for running ML-models tasks via Redis queue.

> Pipelines based on `LaurentMazare/tch-rs` wrapper crate for original PyTorch for CUDA 11.8 (https://pytorch.org/get-started/locally/).


![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/siddthartha/enso-darknet/rust.yml?logo=rust&label=rust)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/siddthartha/enso-darknet/docker-image.yml)

### Usage with RunPod

* Create and run CPU Pod from official Redis image (`redis:latest` for example)
* Create pod with Enso SD-2.1 community template (https://www.runpod.io/console/explore/2105h5uma5)
* Set `ENSO_REDIS_HOST=redis://{REDIS_POD_URL}:{REDIS_POD_EXTERNAL_PORT}` variable in that template
* Now you can put the task to queue:
  * Get `/render/?prompt=some+prompt&steps=25&height=1024&width=768` to start processing
  * Take `uuid` field from response
  * Try to get `/result/{uuid}.jpg` while it becomes ready or try to see intermediatory timesteps like `/result/{uuid}-{step}.jpg`
  * Also any such pod from this template can be tested by hands via simple debug GUI on `https://pod-url/`

### Usage in docker

* `docker pull dogen/enso-darknet:latest`
* Run job via CLI interface:
```
docker run \
    -v ./data:/enso-darknet/data \
    -v ./media:/enso-darknet/media \
    dogen/enso-darknet ./sd-cli \
    --prompt "Some prompt"
    --clip-weights ./data/clip-2.1.ot \
    --vae-weights ./data/vae-2.1.ot \
    --unet-weights ./data/unet-2.1.ot \
    --vocab-file ./data/bpe_simple_vocab_16e6.txt \
    --n-steps 5 \
    --final-image ./media/output.jpg
```
* Run API and server worker with Redis job queue:
  * `docker-compose up -d`
  * `wget http://localhost:80/api/render/?prompt=Some%20prompt`

### TODO:

* upgrade all dependencies (CUDA `12.1`, libtorch `2.3.0`, tch-rs `0.16`)
* refactore API to REST-like
* put results to S3
* multiple GPU devices support
* load balancing
* migrate to candle framework (?)
* add other various pipelines (OpenGPT, Yolo, etc..)

# You can donate my work on this repository

> USDT/TRC20 address `TWwumLM9UXZbZdW8ySqWNNNkoqvQJ8PMdK`
