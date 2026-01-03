/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/horsegame.json`.
 */
export type Horsegame = {
  "address": "2S398gEigP71GhLjryxZwL4xTJ5iEE8QH3q5Vmo4r6v8",
  "metadata": {
    "name": "horsegame",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Horse Racing & Staking Game on Solana"
  },
  "instructions": [
    {
      "name": "breedHorsesCommit",
      "docs": [
        "Commit to breeding horses"
      ],
      "discriminator": [
        206,
        25,
        92,
        113,
        190,
        99,
        25,
        198
      ],
      "accounts": [
        {
          "name": "playerWallet",
          "writable": true,
          "signer": true
        },
        {
          "name": "player",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "playerWallet"
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "rewardsVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  115,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "playerTokenAccount",
          "writable": true
        },
        {
          "name": "tokenMint"
        },
        {
          "name": "tokenProgram"
        }
      ],
      "args": [
        {
          "name": "horseIndices",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "breedHorsesSettle",
      "docs": [
        "Settle breeding (reveal offspring)"
      ],
      "discriminator": [
        39,
        105,
        81,
        163,
        125,
        36,
        73,
        160
      ],
      "accounts": [
        {
          "name": "playerWallet",
          "writable": true,
          "signer": true
        },
        {
          "name": "player",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "playerWallet"
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "rewardsVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  115,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "playerTokenAccount",
          "writable": true
        },
        {
          "name": "tokenMint"
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "slotHashes"
        }
      ],
      "args": []
    },
    {
      "name": "cancelPendingAction",
      "docs": [
        "Cancel a pending action"
      ],
      "discriminator": [
        218,
        126,
        76,
        224,
        30,
        9,
        86,
        114
      ],
      "accounts": [
        {
          "name": "playerWallet",
          "writable": true,
          "signer": true
        },
        {
          "name": "player",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "playerWallet"
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "rewardsVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  115,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "playerTokenAccount",
          "writable": true
        },
        {
          "name": "tokenMint"
        },
        {
          "name": "tokenProgram"
        }
      ],
      "args": []
    },
    {
      "name": "claimRewards",
      "docs": [
        "Claim racing rewards"
      ],
      "discriminator": [
        4,
        144,
        132,
        71,
        116,
        23,
        151,
        80
      ],
      "accounts": [
        {
          "name": "playerWallet",
          "writable": true,
          "signer": true
        },
        {
          "name": "player",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "playerWallet"
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "rewardsVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  115,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "playerTokenAccount",
          "writable": true
        },
        {
          "name": "tokenMint"
        },
        {
          "name": "tokenProgram"
        }
      ],
      "args": []
    },
    {
      "name": "enterRace",
      "docs": [
        "Enter a horse into racing (staking)"
      ],
      "discriminator": [
        242,
        118,
        55,
        253,
        83,
        235,
        115,
        31
      ],
      "accounts": [
        {
          "name": "playerWallet",
          "writable": true,
          "signer": true
        },
        {
          "name": "player",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "playerWallet"
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "rewardsVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  115,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "tokenMint"
        },
        {
          "name": "playerTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram"
        }
      ],
      "args": [
        {
          "name": "horseIndex",
          "type": "u8"
        }
      ]
    },
    {
      "name": "initializeProgram",
      "discriminator": [
        176,
        107,
        205,
        168,
        24,
        157,
        175,
        103
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "feesWallet"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "startSlot",
          "type": "u64"
        },
        {
          "name": "totalSupply",
          "type": "u64"
        },
        {
          "name": "initialStablePurchaseFeeLamports",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "horsePackCostMicrotokens",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "gambleFeeLamports",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "stakingLockupSlots",
          "type": "u64"
        },
        {
          "name": "tokenRewardRate",
          "type": "u64"
        }
      ]
    },
    {
      "name": "openHorsePackCommit",
      "docs": [
        "Commit to opening a horse pack"
      ],
      "discriminator": [
        213,
        158,
        161,
        14,
        46,
        131,
        246,
        211
      ],
      "accounts": [
        {
          "name": "playerWallet",
          "writable": true,
          "signer": true
        },
        {
          "name": "player",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "playerWallet"
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "rewardsVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  115,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "playerTokenAccount",
          "writable": true
        },
        {
          "name": "feesTokenAccount",
          "writable": true
        },
        {
          "name": "referrerTokenAccount",
          "writable": true,
          "optional": true
        },
        {
          "name": "tokenMint",
          "writable": true
        },
        {
          "name": "tokenProgram"
        }
      ],
      "args": []
    },
    {
      "name": "purchaseInitialStable",
      "docs": [
        "NON ADMIN FUNCTIONS",
        "Purchase initial stable to start playing"
      ],
      "discriminator": [
        108,
        11,
        228,
        183,
        198,
        140,
        129,
        135
      ],
      "accounts": [
        {
          "name": "playerWallet",
          "writable": true,
          "signer": true
        },
        {
          "name": "tokenMint",
          "docs": [
            "Token mint (validated to match global_state.token_mint)"
          ]
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "player",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "playerWallet"
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "feesWallet",
          "writable": true
        },
        {
          "name": "referrerWallet",
          "writable": true,
          "optional": true
        },
        {
          "name": "playerTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "playerWallet"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "releaseHorse",
      "docs": [
        "Release a horse (remove from stable)"
      ],
      "discriminator": [
        85,
        68,
        13,
        161,
        43,
        16,
        186,
        125
      ],
      "accounts": [
        {
          "name": "playerWallet",
          "writable": true,
          "signer": true
        },
        {
          "name": "player",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "playerWallet"
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "rewardsVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  115,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "playerTokenAccount",
          "writable": true
        },
        {
          "name": "feesTokenAccount",
          "writable": true
        },
        {
          "name": "tokenMint"
        },
        {
          "name": "tokenProgram"
        }
      ],
      "args": [
        {
          "name": "horseIndex",
          "type": "u8"
        }
      ]
    },
    {
      "name": "resetPlayer",
      "docs": [
        "────────────────────────────────────────────────────────────────────────────",
        "ALL ADMIN FUNCTIONS ENFORCED BY AUTHORITY SIGNING IXS",
        "────────────────────────────────────────────────────────────────────────────"
      ],
      "discriminator": [
        173,
        181,
        112,
        108,
        27,
        126,
        225,
        123
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true,
          "relations": [
            "globalState"
          ]
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "player",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "playerWallet"
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "tokenMint"
        },
        {
          "name": "playerWallet"
        }
      ],
      "args": []
    },
    {
      "name": "setTokenMint",
      "docs": [
        "Set or update the token mint (admin only)",
        "Call this after initialize_program to configure the token",
        "Returns the vault address where you need to transfer tokens"
      ],
      "discriminator": [
        204,
        233,
        179,
        83,
        12,
        31,
        139,
        120
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true,
          "relations": [
            "globalState"
          ]
        },
        {
          "name": "tokenMint",
          "docs": [
            "The new token mint (Token-2022 from pump.fun)"
          ]
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "feesWallet"
        },
        {
          "name": "feesTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "feesWallet"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "rewardsVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  115,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "settleOpenHorsePack",
      "docs": [
        "Settle horse pack opening (reveal horses)"
      ],
      "discriminator": [
        122,
        66,
        152,
        226,
        24,
        52,
        130,
        156
      ],
      "accounts": [
        {
          "name": "playerWallet",
          "writable": true,
          "signer": true
        },
        {
          "name": "player",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "playerWallet"
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "rewardsVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  115,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "playerTokenAccount",
          "writable": true
        },
        {
          "name": "tokenMint"
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "slotHashes"
        }
      ],
      "args": []
    },
    {
      "name": "toggleProduction",
      "discriminator": [
        66,
        176,
        34,
        247,
        31,
        189,
        231,
        105
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true,
          "relations": [
            "globalState"
          ]
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "enable",
          "type": "bool"
        }
      ]
    },
    {
      "name": "updateParameter",
      "discriminator": [
        99,
        187,
        162,
        136,
        162,
        32,
        122,
        2
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true,
          "relations": [
            "globalState"
          ]
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "parameterIndex",
          "type": "u8"
        },
        {
          "name": "parameterValue",
          "type": "u64"
        }
      ]
    },
    {
      "name": "updatePoolManual",
      "discriminator": [
        134,
        168,
        45,
        173,
        62,
        196,
        240,
        243
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true,
          "relations": [
            "globalState"
          ]
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "upgradeStable",
      "docs": [
        "Upgrade stable capacity"
      ],
      "discriminator": [
        96,
        54,
        224,
        106,
        186,
        75,
        108,
        170
      ],
      "accounts": [
        {
          "name": "playerWallet",
          "writable": true,
          "signer": true
        },
        {
          "name": "player",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "playerWallet"
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "rewardsVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  115,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "playerTokenAccount",
          "writable": true
        },
        {
          "name": "feesTokenAccount",
          "writable": true
        },
        {
          "name": "tokenMint",
          "writable": true
        },
        {
          "name": "tokenProgram"
        }
      ],
      "args": [
        {
          "name": "stableType",
          "type": "u8"
        }
      ]
    },
    {
      "name": "withdrawFromRace",
      "docs": [
        "Withdraw horse from racing"
      ],
      "discriminator": [
        74,
        226,
        13,
        165,
        13,
        109,
        16,
        93
      ],
      "accounts": [
        {
          "name": "playerWallet",
          "writable": true,
          "signer": true
        },
        {
          "name": "player",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "playerWallet"
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "rewardsVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  115,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "tokenMint"
        },
        {
          "name": "playerTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram"
        }
      ],
      "args": [
        {
          "name": "horseIndex",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "globalState",
      "discriminator": [
        163,
        46,
        74,
        168,
        216,
        123,
        133,
        98
      ]
    },
    {
      "name": "player",
      "discriminator": [
        205,
        222,
        112,
        7,
        165,
        155,
        206,
        218
      ]
    }
  ],
  "events": [
    {
      "name": "horseEnteredRace",
      "discriminator": [
        234,
        77,
        60,
        240,
        35,
        80,
        253,
        1
      ]
    },
    {
      "name": "horsePackOpened",
      "discriminator": [
        46,
        43,
        72,
        74,
        105,
        200,
        2,
        148
      ]
    },
    {
      "name": "horseReleased",
      "discriminator": [
        41,
        145,
        63,
        133,
        111,
        160,
        53,
        123
      ]
    },
    {
      "name": "horseWithdrawnFromRace",
      "discriminator": [
        164,
        58,
        159,
        59,
        94,
        84,
        204,
        250
      ]
    },
    {
      "name": "horsesBred",
      "discriminator": [
        80,
        128,
        114,
        48,
        120,
        173,
        183,
        35
      ]
    },
    {
      "name": "initialStablePurchased",
      "discriminator": [
        49,
        24,
        74,
        56,
        158,
        148,
        247,
        146
      ]
    },
    {
      "name": "programInitialized",
      "discriminator": [
        43,
        70,
        110,
        241,
        199,
        218,
        221,
        245
      ]
    },
    {
      "name": "stableUpgraded",
      "discriminator": [
        147,
        230,
        58,
        185,
        55,
        17,
        254,
        123
      ]
    },
    {
      "name": "tokenMintSet",
      "discriminator": [
        236,
        245,
        220,
        223,
        246,
        139,
        23,
        69
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "unauthorized",
      "msg": "Unauthorized access"
    },
    {
      "code": 6001,
      "name": "productionDisabled",
      "msg": "Production is disabled"
    },
    {
      "code": 6002,
      "name": "initialStableAlreadyPurchased",
      "msg": "Initial stable already purchased"
    },
    {
      "code": 6003,
      "name": "selfReferralNotAllowed",
      "msg": "Self referral not allowed"
    },
    {
      "code": 6004,
      "name": "stableCapacityExceeded",
      "msg": "Stable capacity exceeded"
    },
    {
      "code": 6005,
      "name": "feedCapacityExceeded",
      "msg": "Feed capacity exceeded"
    },
    {
      "code": 6006,
      "name": "horseIndexOutOfBounds",
      "msg": "Horse index out of bounds"
    },
    {
      "code": 6007,
      "name": "horseIsRacing",
      "msg": "Horse is already racing"
    },
    {
      "code": 6008,
      "name": "horseNotRacing",
      "msg": "Horse is not racing"
    },
    {
      "code": 6009,
      "name": "horsePendingBreeding",
      "msg": "Horse is pending breeding"
    },
    {
      "code": 6010,
      "name": "invalidStableType",
      "msg": "Invalid stable type"
    },
    {
      "code": 6011,
      "name": "insufficientTokens",
      "msg": "Insufficient tokens"
    },
    {
      "code": 6012,
      "name": "cooldownNotExpired",
      "msg": "Cooldown not expired"
    },
    {
      "code": 6013,
      "name": "horsePackAlreadyPending",
      "msg": "Horse pack already pending"
    },
    {
      "code": 6014,
      "name": "noHorsePackPending",
      "msg": "No horse pack pending"
    },
    {
      "code": 6015,
      "name": "randomnessNotResolved",
      "msg": "Randomness not resolved"
    },
    {
      "code": 6016,
      "name": "invalidSlotHashes",
      "msg": "Invalid slot hashes"
    },
    {
      "code": 6017,
      "name": "slotNotFound",
      "msg": "Slot not found"
    },
    {
      "code": 6018,
      "name": "invalidTokenAccountOwner",
      "msg": "Invalid token account owner"
    },
    {
      "code": 6019,
      "name": "invalidMintAuthority",
      "msg": "Invalid mint authority"
    },
    {
      "code": 6020,
      "name": "invalidDecimals",
      "msg": "Invalid decimals"
    },
    {
      "code": 6021,
      "name": "arithmeticOverflow",
      "msg": "Arithmetic overflow"
    },
    {
      "code": 6022,
      "name": "invalidReferralFee",
      "msg": "Invalid referral fee"
    },
    {
      "code": 6023,
      "name": "invalidBurnRate",
      "msg": "Invalid burn rate"
    },
    {
      "code": 6024,
      "name": "invalidDustThresholdDivisor",
      "msg": "Invalid dust threshold divisor"
    },
    {
      "code": 6025,
      "name": "invalidParameterIndex",
      "msg": "Invalid parameter index"
    },
    {
      "code": 6026,
      "name": "invalidBreedingHorseCount",
      "msg": "Invalid breeding horse count"
    },
    {
      "code": 6027,
      "name": "duplicateBreedingHorseIndices",
      "msg": "Duplicate breeding horse indices"
    },
    {
      "code": 6028,
      "name": "breedingAlreadyPending",
      "msg": "Breeding already pending"
    },
    {
      "code": 6029,
      "name": "noBreedingPending",
      "msg": "No breeding pending"
    },
    {
      "code": 6030,
      "name": "noPendingAction",
      "msg": "No pending action"
    },
    {
      "code": 6031,
      "name": "cancelTimeoutNotExpired",
      "msg": "Cancel timeout not expired"
    },
    {
      "code": 6032,
      "name": "referrerAccountMissing",
      "msg": "Referrer account missing"
    },
    {
      "code": 6033,
      "name": "tokenNotInitialized",
      "msg": "Token mint not initialized - call set_token_mint first"
    },
    {
      "code": 6034,
      "name": "invalidTokenMint",
      "msg": "Invalid token mint - does not match configured token"
    }
  ],
  "types": [
    {
      "name": "globalState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "tokenMint",
            "type": "pubkey"
          },
          {
            "name": "feesWallet",
            "type": "pubkey"
          },
          {
            "name": "rewardsVault",
            "type": "pubkey"
          },
          {
            "name": "tokenInitialized",
            "type": "bool"
          },
          {
            "name": "totalSupply",
            "type": "u64"
          },
          {
            "name": "burnedTokens",
            "type": "u64"
          },
          {
            "name": "cumulativeRewards",
            "type": "u64"
          },
          {
            "name": "startSlot",
            "type": "u64"
          },
          {
            "name": "rewardRate",
            "type": "u64"
          },
          {
            "name": "accTokensPerSpeed",
            "type": "u128"
          },
          {
            "name": "lastRewardSlot",
            "type": "u64"
          },
          {
            "name": "burnRate",
            "type": "u8"
          },
          {
            "name": "referralFee",
            "type": "u8"
          },
          {
            "name": "productionEnabled",
            "type": "bool"
          },
          {
            "name": "dustThresholdDivisor",
            "type": "u64"
          },
          {
            "name": "initialStablePurchaseFeeLamports",
            "type": "u64"
          },
          {
            "name": "horsePackCostMicrotokens",
            "type": "u64"
          },
          {
            "name": "gambleFeeLamports",
            "type": "u64"
          },
          {
            "name": "totalFeedConsumption",
            "type": "u64"
          },
          {
            "name": "totalSpeed",
            "type": "u64"
          },
          {
            "name": "totalGlobalGambles",
            "type": "u64"
          },
          {
            "name": "totalGlobalGambleWins",
            "type": "u64"
          },
          {
            "name": "totalHorsePacksOpened",
            "type": "u64"
          },
          {
            "name": "totalBreedingAttempts",
            "type": "u64"
          },
          {
            "name": "totalSuccessfulBreeding",
            "type": "u64"
          },
          {
            "name": "totalStakedTokens",
            "type": "u64"
          },
          {
            "name": "stakingLockupSlots",
            "type": "u64"
          },
          {
            "name": "accSolRewardsPerToken",
            "type": "u128"
          },
          {
            "name": "accTokenRewardsPerToken",
            "type": "u128"
          },
          {
            "name": "lastStakingRewardSlot",
            "type": "u64"
          },
          {
            "name": "tokenRewardRate",
            "type": "u64"
          },
          {
            "name": "totalSolDeposited",
            "type": "u64"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          }
        ]
      }
    },
    {
      "name": "horse",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u16"
          },
          {
            "name": "grade",
            "type": "u8"
          },
          {
            "name": "speed",
            "type": "u16"
          },
          {
            "name": "staminaCost",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "horseEnteredRace",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "player",
            "type": "pubkey"
          },
          {
            "name": "horseIndex",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "horsePackOpened",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "player",
            "type": "pubkey"
          },
          {
            "name": "horseIds",
            "type": {
              "array": [
                "u8",
                5
              ]
            }
          }
        ]
      }
    },
    {
      "name": "horseReleased",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "player",
            "type": "pubkey"
          },
          {
            "name": "horseIndex",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "horseWithdrawnFromRace",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "player",
            "type": "pubkey"
          },
          {
            "name": "horseIndex",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "horsesBred",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "player",
            "type": "pubkey"
          },
          {
            "name": "successfulOffspring",
            "type": "u8"
          },
          {
            "name": "totalBred",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "initialStablePurchased",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "playerWallet",
            "type": "pubkey"
          },
          {
            "name": "playerAccount",
            "type": "pubkey"
          },
          {
            "name": "referrer",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "stableType",
            "type": "u8"
          },
          {
            "name": "initialHorses",
            "type": "u8"
          },
          {
            "name": "slot",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "pendingRandomAction",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          },
          {
            "name": "gamble",
            "fields": [
              {
                "name": "amount",
                "type": "u64"
              }
            ]
          },
          {
            "name": "horsePack"
          },
          {
            "name": "breeding",
            "fields": [
              {
                "name": "horseIndices",
                "type": {
                  "array": [
                    "u8",
                    128
                  ]
                }
              },
              {
                "name": "horseCount",
                "type": "u8"
              }
            ]
          }
        ]
      }
    },
    {
      "name": "player",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "stable",
            "type": {
              "defined": {
                "name": "stable"
              }
            }
          },
          {
            "name": "horses",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "horse"
                  }
                },
                128
              ]
            }
          },
          {
            "name": "horseCount",
            "type": "u8"
          },
          {
            "name": "racingHorsesBitset",
            "type": "u128"
          },
          {
            "name": "feedConsumption",
            "type": "u64"
          },
          {
            "name": "totalSpeed",
            "type": "u64"
          },
          {
            "name": "referrer",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "lastAccTokensPerSpeed",
            "type": "u128"
          },
          {
            "name": "lastClaimSlot",
            "type": "u64"
          },
          {
            "name": "lastUpgradeSlot",
            "type": "u64"
          },
          {
            "name": "totalRewards",
            "type": "u64"
          },
          {
            "name": "totalGambles",
            "type": "u64"
          },
          {
            "name": "totalGambleWins",
            "type": "u64"
          },
          {
            "name": "pendingAction",
            "type": {
              "defined": {
                "name": "pendingRandomAction"
              }
            }
          },
          {
            "name": "commitSlot",
            "type": "u64"
          },
          {
            "name": "totalEarningsForReferrer",
            "type": "u64"
          },
          {
            "name": "totalHorsePacksOpened",
            "type": "u64"
          },
          {
            "name": "totalHorsesBred",
            "type": "u64"
          },
          {
            "name": "successfulBreeding",
            "type": "u64"
          },
          {
            "name": "totalSolSpent",
            "type": "u64"
          },
          {
            "name": "totalTokensSpent",
            "type": "u64"
          },
          {
            "name": "stakedTokens",
            "type": "u64"
          },
          {
            "name": "lastStakeSlot",
            "type": "u64"
          },
          {
            "name": "lastAccSolRewardsPerToken",
            "type": "u128"
          },
          {
            "name": "lastAccTokenRewardsPerToken",
            "type": "u128"
          },
          {
            "name": "claimedTokenRewards",
            "type": "u64"
          },
          {
            "name": "totalRacesEntered",
            "type": "u64"
          },
          {
            "name": "totalRaceWins",
            "type": "u64"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                64
              ]
            }
          }
        ]
      }
    },
    {
      "name": "programInitialized",
      "docs": [
        "Event emitted when program is initialized"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "feesWallet",
            "type": "pubkey"
          },
          {
            "name": "startSlot",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "stable",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stableType",
            "type": "u8"
          },
          {
            "name": "racingSlots",
            "type": "u8"
          },
          {
            "name": "feedCapacity",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "stableUpgraded",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "player",
            "type": "pubkey"
          },
          {
            "name": "newStableType",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "tokenMintSet",
      "docs": [
        "Event emitted when token mint is set/updated"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "tokenMint",
            "type": "pubkey"
          },
          {
            "name": "rewardsVault",
            "type": "pubkey"
          },
          {
            "name": "feesWallet",
            "type": "pubkey"
          },
          {
            "name": "isUpdate",
            "type": "bool"
          }
        ]
      }
    }
  ]
};
