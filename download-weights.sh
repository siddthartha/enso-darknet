#!/bin/bash

# download vocabulary
wget https://huggingface.co/lmz/rust-stable-diffusion-v2-1/raw/main/weights/bpe_simple_vocab_16e6.txt -O ./data/bpe_simple_vocab_16e6.txt

# SD 2.1
wget https://huggingface.co/lmz/rust-stable-diffusion-v2-1/resolve/main/weights/clip_v2.1.ot -O ./data/clip-2.1.ot
wget https://huggingface.co/lmz/rust-stable-diffusion-v2-1/resolve/main/weights/unet_v2.1.ot -O ./data/unet-2.1.ot
wget https://huggingface.co/lmz/rust-stable-diffusion-v2-1/resolve/main/weights/vae_v2.1.ot -O ./data/vae-2.1.ot
