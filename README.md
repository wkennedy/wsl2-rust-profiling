
CLion

# windows
wsl --update

# wsl 2
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