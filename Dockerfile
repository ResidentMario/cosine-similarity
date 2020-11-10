FROM ubuntu:bionic
WORKDIR /spell

# Install Python
RUN apt-get update && \
    apt-get install -y wget curl build-essential git && \
    rm -rf /var/lib/apt/lists/*
ENV CONDA_HOME=/root/anaconda/
RUN wget \
    https://repo.anaconda.com/miniconda/Miniconda3-py37_4.8.3-Linux-x86_64.sh \
    && mkdir /root/.conda \
    && bash Miniconda3-py37_4.8.3-Linux-x86_64.sh -fbp $CONDA_HOME \
    && rm -f Miniconda3-py37_4.8.3-Linux-x86_64.sh
ENV PATH=/root/anaconda/bin:$PATH
RUN conda install -c conda-forge beautifulsoup4 jupyterlab

# Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH=/root/.cargo/bin:$PATH