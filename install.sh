#!/bin/sh

# create environment 
echo "Creating virtual environment..."
python3 -m virtualenv venv
. ./venv/bin/activate

# install levenshtein
echo "Installing dependencies..."
pip install -U pip python-Levenshtein

# install fontconfig manually
echo "Installing fontconfig..."
git clone https://github.com/Vayn/python-fontconfig
cd python-fontconfig
rm -f fontconfig.c
cython fontconfig.pyx
python3 setup.py install
cd ..
rm -rf python-fontconfig

echo "Installation done."