#!/bin/bash

protoc --rust_out src/message/gen message/*.proto
protoc --csharp_out=cs/Message --csharp_opt=file_extension=.g.cs message/*.proto
