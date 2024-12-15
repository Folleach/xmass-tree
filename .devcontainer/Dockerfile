FROM mcr.microsoft.com/devcontainers/base:bullseye

RUN sudo apt -y update && sudo apt -y install gcc-avr binutils-avr avr-libc avrdude clang libudev-dev pkg-config && sudo usermod -a -G dialout vscode && sudo usermod -a -G video vscode
