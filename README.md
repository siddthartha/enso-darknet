# enso-darknet

A simple asynchronuous json API for running `Stable Diffusion` generation tasks via Redis queue.

> Pipeline based on examples from `LaurentMazare/diffusers-rs` crate.

## TODO:

* put results to S3
* add other various pipelines (OpenGPT etc.)


### Usage

* `docker pull dogen/enso-darknet:latest`
* `./download-weights.sh`
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


# You can donate my work on this repository

> USDT/TRC20 address `TWwumLM9UXZbZdW8ySqWNNNkoqvQJ8PMdK`
