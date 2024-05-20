# Function to download a file if it doesn't exist
download_if_not_exist() {
  local url=$1
  local output_path=$2

  if [ ! -f "$output_path" ]; then
    echo "File $output_path not found. Downloading..."
    wget $url -O $output_path
  else
    echo "File $output_path already exists. Skipping download."
  fi
}

# Download vocabulary if not exists
download_if_not_exist "https://huggingface.co/lmz/rust-stable-diffusion-v2-1/raw/main/weights/bpe_simple_vocab_16e6.txt" "./data/vocab_16e6.txt"

# Download SD 2.1 files if not exist
download_if_not_exist "https://huggingface.co/lmz/rust-stable-diffusion-v2-1/resolve/main/weights/clip_v2.1.ot" "./data/clip-2.1.ot"
download_if_not_exist "https://huggingface.co/lmz/rust-stable-diffusion-v2-1/resolve/main/weights/unet_v2.1.ot" "./data/unet-2.1.ot"
download_if_not_exist "https://huggingface.co/lmz/rust-stable-diffusion-v2-1/resolve/main/weights/vae_v2.1.ot" "./data/vae-2.1.ot"

# Download SD 1.5 files if not exist
download_if_not_exist "https://huggingface.co/lmz/rust-stable-diffusion-v1-5/resolve/main/weights/pytorch_model.ot" "./data/clip-1.5.ot"
download_if_not_exist "https://huggingface.co/lmz/rust-stable-diffusion-v1-5/resolve/main/weights/unet.ot" "./data/unet-1.5.ot"
download_if_not_exist "https://huggingface.co/lmz/rust-stable-diffusion-v1-5/resolve/main/weights/vae.ot" "./data/vae-1.5.ot"
