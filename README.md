# binary-nn

[![Build Status](https://travis-ci.org/ttakamura/binary-nn-rust.svg?branch=master)](https://travis-ci.org/ttakamura/binary-nn-rust)

This is a Binarized Neural Networks runtime written by Rust.

* [Binarized Neural Networks](https://arxiv.org/abs/1602.02830)

## Description

binary-nn don't provide training features. it can be used only for predict, inference, phase.

You should train your network using your favorite tools. Then, export learned weights as files and import it to this library.
