#!/bin/bash

# download vocabulary
wget https://github.com/openai/CLIP/raw/main/clip/bpe_simple_vocab_16e6.txt.gz -O ./data/vocab_16e6.txt.gz
gunzip -f ./data/vocab_16e6.txt.gz

# SD 2.1
wget https://huggingface.co/stabilityai/stable-diffusion-2-1/resolve/fp16/text_encoder/pytorch_model.bin -O ./data/clip-2.1.bin
# download VAE
wget https://huggingface.co/stabilityai/stable-diffusion-2-1/resolve/fp16/vae/diffusion_pytorch_model.bin -O ./data/vae-2.1.bin
# download UNET
wget https://huggingface.co/stabilityai/stable-diffusion-2-1/resolve/fp16/unet/diffusion_pytorch_model.bin -O ./data/unet-2.1.bin

# SD 1.5
#wget https://huggingface.co/openai/clip-vit-large-patch14/resolve/main/pytorch_model.bin -O ./data/clip-1.5.bin
# download VAE
#wget https://huggingface.co/runwayml/stable-diffusion-v1-5/blob/main/vae/diffusion_pytorch_model.bin -O ./data/vae-1.5.bin
# download UNET
#wget https://huggingface.co/runwayml/stable-diffusion-v1-5/blob/main/unet/diffusion_pytorch_model.bin -O ./data/unet-1.5.bin
