# Iris
Automating all the things with my webcam

## Table of Contents
+ [About](#about)
+ [Getting Started](#getting_started)
+ [Usage](#usage)

## About <a name = "about"></a>

Automates toggling a pair of Elgato Key Light Airs when a webcam is activated.

## Getting Started <a name = "getting_started"></a>


1. Find the IP address of your lights
1. Set the environemnt variables `$LIGHT_1` and `$LIGHT_2` to the IP address of your lights
    * `export LIGHT_1=192.168.1.10; export LIGHT_2=192.168.1.11`

## Usage

```bash
git clone git@github.com:kendallm/Iris.git
cd ./Iris
.bin/iris.sh
```

Instructions for starting this automatically at login will be written later
