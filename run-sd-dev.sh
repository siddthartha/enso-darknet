#!/bin/bash

docker-compose exec \
    enso-darknet cargo run --bin sd-cli -- \
    --clip-weights ./data/clip-2.1.ot \
    --vae-weights ./data/vae-2.1.ot \
    --unet-weights ./data/unet-2.1.ot \
    --vocab-file ./data/vocab_16e6.txt \
    --n-steps 5 \
    --final-image ./media/output.jpg