# enso-darknet

A `Stable Diffusion` pipeline implemented on Rust using `libtorch`, and packed to a docker image.

> Based on examples from `tch-rs` crate.

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
  * `wget http://localhost:80/run-job/?prompt=Some%20prompt`

### You can donate my work on this repository
> USDT/TRC20 address `TWwumLM9UXZbZdW8ySqWNNNkoqvQJ8PMdK`
