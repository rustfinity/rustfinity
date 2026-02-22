#!/bin/bash

TYPE=${TYPE:-patch}

cargo set-version -p rustfinity --bump $TYPE