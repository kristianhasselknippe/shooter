@echo off

pushd shooter-server
start cargo run

popd
pushd shooter-client
start cargo run

popd
