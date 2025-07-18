#!/usr/bin/env bash

# This script is intended to update LOCAL assets (i.e. from git submodules) inside `csaf-rs`.
# It SHOULD be run after each checkout or similar operation to make sure that assets within `csaf-rs`
# are in sync with the submodules. Failing to run this script before the build might result in unexpected
# behavior due to mismatches between embedded assets and the library version!

rsync -c csaf/csaf_2.0/json_schema/csaf_json_schema.json csaf-rs/assets/csaf_2.0_json_schema.json
rsync -c csaf/csaf_2.1/json_schema/csaf.json csaf-rs/assets/csaf_2.1_json_schema.json
rsync -c ssvc/data/schema/v1/Decision_Point-1-0-1.schema.json csaf-rs/assets/decision_point_1.0.1_json_schema.json
rsync -cr --delete ssvc/data/json/decision_points/ csaf-rs/assets/ssvc_decision_points/
