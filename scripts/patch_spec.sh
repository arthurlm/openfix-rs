#!/bin/bash
set -xe

typeset spec_files=$(find protocol-spec -name '*.xml')

sed -i '/BeginString/d' $spec_files
sed -i '/BodyLength/d' $spec_files
sed -i '/CheckSum/d' $spec_files
