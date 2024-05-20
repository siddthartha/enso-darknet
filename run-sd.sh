#!/bin/bash

docker run \
    -v ./data:/enso-darknet/data \
    -v ./media:/enso-darknet/media \
    dogen/enso-darknet ./sd-cli --prompt "Some beautiful girls runnig on the beach" \
    --clip-weights ./data/clip-1.5.ot \
    --vae-weights ./data/vae-1.5.ot \
    --unet-weights ./data/unet-1.5.ot \
    --vocab-file ./data/vocab_16e6.txt \
    --n-steps 5 \
    --final-image ./media/output.jpg