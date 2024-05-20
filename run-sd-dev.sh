#!/bin/bash

docker-compose exec \
    enso-darknet cargo run --bin sd-cli -- \
    --sd-version v1-5 \
    --clip-weights ./data/clip-1.5.ot \
    --vae-weights ./data/vae-1.5.ot \
    --unet-weights ./data/unet-1.5.ot \
    --vocab-file ./data/vocab_16e6.txt \
    --n-steps 5 \
    --final-image ./media/output.jpg