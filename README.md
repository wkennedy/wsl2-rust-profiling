## Configuring perf in WSL2 for profiling with CLion

Having recently switched some of my development to setting up remote development in WSL2 with CLion, I noticed that profiling was not easily available. I tried somethings, but didn't have much success until I came across this gist: https://gist.github.com/abel0b/b1881e41b9e1c4b16d84e5e083c38a13

At the bottom of that thread is a suggestion to run these commands:

In the Windows shell run:

    wsl --update

Then in WSL2 (assuming it's Ubuntu) run:

    sudo apt update
    sudo apt install flex bison
    sudo apt install libdwarf-dev libelf-dev libnuma-dev libunwind-dev \
    libnewt-dev libdwarf++0 libelf++0 libdw-dev libbfb0-dev \
    systemtap-sdt-dev libssl-dev libperl-dev python-dev-is-python3 \
    binutils-dev libiberty-dev libzstd-dev libcap-dev libbabeltrace-dev
    git clone https://github.com/microsoft/WSL2-Linux-Kernel --depth 1
    cd WSL2-Linux-Kernel/tools/perf
    make -j8 # parallel build
    sudo cp perf /usr/local/bin

In the CLion settings you can search for "profiler" and check that perf is configured correctly:
