module.exports = {
        rpc: {
            protos: {
                getProtos: {
                    description: "this is the description", type: "String",
                    params: [
                        { name: 'params', type: 'GetProtosParams' },
                        { name: 'at', type: 'BlockHash', isOptional: true }
                    ]
                },
            }
        },

        types: {
            ShardsFormat: {
                _enum: [
                    "edn",
                    "binary"
                ]
            },

            AudioCategories: {
                _enum: [
                    "oggFile",
                    "mp3File",
                ]
            },

            ModelCategories: {
                _enum: [
                    "gltfFile",
                    "sdf",
                    "physicsCollider"
                ]
            },

            TextureCategories: {
                _enum: [
                    "pngFile",
                    "jpgFile"
                ]
            },

            VectorCategories: {
                _enum: [
                    "svgFile",
                    "ttfFile"
                ]
            },

            VideoCategories: {
                _enum: [
                    "mkvFile",
                    "mp4File"
                ]
            },

            TextCategories: {
                _enum: [
                    "plain",
                    "json"
                ]
            },

            BinaryCategories: {
                _enum: [
                    "wasmProgram",
                    "wasmReactor",
                    "blendFile",
                    "onnxModel",
                ]
            },

            ShardsScriptInfo: {
                format: 'ShardsFormat',
                requiring: 'Vec<ShardsTrait>',
                implementing: 'Vec<ShardsTrait>'
            },

            ShardsTrait: "Vec<u16>",

            ShardsFormat: {
                _enum: [
                    "edn",
                    "binary",
                ]
            },

            Categories: {
                _enum: {
                    "text": "TextCategories",
                    "trait": "Option<ShardsTrait>",
                    "shards": "ShardsScriptInfo",
                    "audio": "AudioCategories",
                    "texture": "TextureCategories",
                    "vector": "VectorCategories",
                    "video": "VideoCategories",
                    "model": "ModelCategories",
                    "binary": "BinaryCategories",
                }
            },

            BlockHash: 'Hash',

            GetProtosParams: {
                desc: 'bool',
                from: 'u32',
                limit: 'u32',
                metadata_keys: 'Vec<String>',
                owner: 'Option<AccountId>',
                return_owners: 'bool',
                categories: 'Vec<Categories>',
                tags: 'Vec<String>',
                available: 'Option<bool>',
                exclude_tags: 'bool',
            }
        }
    };
