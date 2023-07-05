# **Fatal Errors**
These errors can be returned on the Cold Reset flow. They are considered fatal when returned from the Cold Reset flow. They are reported via the cptra_fw_error_fatal register.
Component | Error | Code |
---|---|---|
Sha256 Driver | DRIVER_SHA256_INVALID_STATE | 0x00020001
Sha256 Driver | DRIVER_SHA256_MAX_DATA      | 0x00020002
Sha256 Driver | DRIVER_SHA256_INVALID_STATE | 0x00020003
Sha256 Driver | DRIVER_SHA256_INVALID_STATE | 0x00020004
<br>
Sha384 Driver | DRIVER_SHA384_READ_DATA_KV_READ    | 0x00030001
Sha384 Driver | DRIVER_SHA384_READ_DATA_KV_WRITE   | 0x00030002
Sha384 Driver | DRIVER_SHA384_READ_DATA_KV_UNKNOWN | 0x00030003
Sha384 Driver | DRIVER_SHA384_INVALID_STATE_ERR    | 0x00030007
Sha384 Driver | DRIVER_SHA384_MAX_DATA_ERR         | 0x00030008
Sha384 Driver | DRIVER_SHA384_INVALID_KEY_SIZE     | 0x00030009
Sha384 Driver | DRIVER_SHA384_INVALID_SLICE        | 0x0003000A
Sha384 Driver | DRIVER_SHA384_INDEX_OUT_OF_BOUNDS  | 0x0003000B
<br>
Hmac384 Driver | DRIVER_HMAC384_READ_KEY_KV_READ	 | 0x00040001
Hmac384 Driver | DRIVER_HMAC384_READ_KEY_KV_WRITE    | 0x00040002
Hmac384 Driver | DRIVER_HMAC384_READ_KEY_KV_UNKNOWN  | 0x00040003
Hmac384 Driver | DRIVER_HMAC384_READ_DATA_KV_READ    | 0x00040004
Hmac384 Driver | DRIVER_HMAC384_READ_DATA_KV_WRITE   | 0x00040005
Hmac384 Driver | DRIVER_HMAC384_READ_DATA_KV_UNKNOWN | 0x00040006
Hmac384 Driver | DRIVER_HMAC384_WRITE_TAG_KV_READ    | 0x00040007
Hmac384 Driver | DRIVER_HMAC384_WRITE_TAG_KV_WRITE   | 0x00040008
Hmac384 Driver | DRIVER_HMAC384_WRITE_TAG_KV_UNKNOWN | 0x00040009
Hmac384 Driver | DRIVER_HMAC384_INVALID_STATE        | 0x0004000b
Hmac384 Driver | DRIVER_HMAC384_MAX_DATA             | 0x0004000c
Hmac384 Driver | DRIVER_HMAC384_INVALID_SLICE        | 0x0004000d
Hmac384 Driver | DRIVER_HMAC384_INDEX_OUT_OF_BOUNDS  | 0x0004000e
<br>
Ecc384 Driver | DRIVER_ECC384_READ_SEED_KV_READ			| 0x00050001
Ecc384 Driver | DRIVER_ECC384_READ_SEED_KV_WRITE        | 0x00050002
Ecc384 Driver | DRIVER_ECC384_READ_SEED_KV_UNKNOWN      | 0x00050003
Ecc384 Driver | DRIVER_ECC384_WRITE_PRIV_KEY_KV_READ    | 0x00050004
Ecc384 Driver | DRIVER_ECC384_WRITE_PRIV_KEY_KV_WRITE   | 0x00050005
Ecc384 Driver | DRIVER_ECC384_WRITE_PRIV_KEY_KV_UNKNOWN | 0x00050006
Ecc384 Driver | DRIVER_ECC384_READ_PRIV_KEY_KV_READ     | 0x00050007
Ecc384 Driver | DRIVER_ECC384_READ_PRIV_KEY_KV_WRITE    | 0x00050008
Ecc384 Driver | DRIVER_ECC384_READ_PRIV_KEY_KV_UNKNOWN  | 0x00050009
Ecc384 Driver | DRIVER_ECC384_READ_DATA_KV_READ         | 0x0005000a
Ecc384 Driver | DRIVER_ECC384_READ_DATA_KV_WRITE        | 0x0005000b
Ecc384 Driver | DRIVER_ECC384_READ_DATA_KV_UNKNOWN      | 0x0005000c
<br>
KeyVault Driver | DRIVER_KV_ERASE_USE_LOCK_SET_FAILURE   | 0x00060001
KeyVault Driver | DRIVER_KV_ERASE_WRITE_LOCK_SET_FAILURE | 0x00060002
<br>
PCR Bank Driver | DRIVER_PCR_BANK_ERASE_WRITE_LOCK_SET_FAILURE | 0x00070001
<br>
Mailbox Driver | DRIVER_MAILBOX_INVALID_STATE	 | 0x00080001
Mailbox Driver | DRIVER_MAILBOX_INVALID_DATA_LEN | 0x00080002
Mailbox Driver | DRIVER_MAILBOX_ENQUEUE_ERR      | 0x00080004
<br>
Sha384 Accelerator Driver | DRIVER_SHA384ACC_INDEX_OUT_OF_BOUNDS | 0x00090003
<br>
Sha1 Driver | DRIVER_SHA1_INVALID_STATE		  | 0x000a0001
Sha1 Driver | DRIVER_SHA1_MAX_DATA            | 0x000a0002
Sha1 Driver | DRIVER_SHA1_INVALID_SLICE       | 0x000a0003
Sha1 Driver | DRIVER_SHA1_INDEX_OUT_OF_BOUNDS | 0x000a0004
<br>
Verifier Library | IMAGE_VERIFIER_ERR_MANIFEST_MARKER_MISMATCH                         | 0x000b0001
Verifier Library | IMAGE_VERIFIER_ERR_MANIFEST_SIZE_MISMATCH                           | 0x000b0002
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_PUB_KEY_DIGEST_INVALID                    | 0x000b0003
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_PUB_KEY_DIGEST_FAILURE                    | 0x000b0004
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_PUB_KEY_DIGEST_MISMATCH                   | 0x000b0005
Verifier Library | IMAGE_VERIFIER_ERR_OWNER_PUB_KEY_DIGEST_FAILURE                     | 0x000b0006
Verifier Library | IMAGE_VERIFIER_ERR_OWNER_PUB_KEY_DIGEST_MISMATCH                    | 0x000b0007
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_ECC_PUB_KEY_INDEX_OUT_OF_BOUNDS           | 0x000b0008
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_ECC_PUB_KEY_REVOKED                       | 0x000b0009
Verifier Library | IMAGE_VERIFIER_ERR_HEADER_DIGEST_FAILURE                            | 0x000b000a
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_ECC_VERIFY_FAILURE                        | 0x000b000b
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_ECC_SIGNATURE_INVALID                     | 0x000b000c
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_ECC_PUB_KEY_INDEX_MISMATCH                | 0x000b000d
Verifier Library | IMAGE_VERIFIER_ERR_OWNER_ECC_VERIFY_FAILURE                         | 0x000b000e
Verifier Library | IMAGE_VERIFIER_ERR_OWNER_ECC_SIGNATURE_INVALID                      | 0x000b000f
Verifier Library | IMAGE_VERIFIER_ERR_TOC_ENTRY_COUNT_INVALID                          | 0x000b0010
Verifier Library | IMAGE_VERIFIER_ERR_TOC_DIGEST_FAILURES                              | 0x000b0011
Verifier Library | IMAGE_VERIFIER_ERR_TOC_DIGEST_MISMATCH                              | 0x000b0012
Verifier Library | IMAGE_VERIFIER_ERR_FMC_DIGEST_FAILURE                               | 0x000b0013
Verifier Library | IMAGE_VERIFIER_ERR_FMC_DIGEST_MISMATCH                              | 0x000b0014
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_DIGEST_FAILURE                           | 0x000b0015
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_DIGEST_MISMATCH                          | 0x000b0016
Verifier Library | IMAGE_VERIFIER_ERR_FMC_RUNTIME_OVERLAP                              | 0x000b0017
Verifier Library | IMAGE_VERIFIER_ERR_FMC_RUNTIME_INCORRECT_ORDER                      | 0x000b0018
Verifier Library | IMAGE_VERIFIER_ERR_OWNER_PUB_KEY_DIGEST_INVALID_ARG                 | 0x000b0019
Verifier Library | IMAGE_VERIFIER_ERR_OWNER_ECC_SIGNATURE_INVALID_ARG                  | 0x000b001a
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_PUB_KEY_DIGEST_INVALID_ARG                | 0x000b001b
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_ECC_SIGNATURE_INVALID_ARG                 | 0x000b001c
Verifier Library | IMAGE_VERIFIER_ERR_FMC_LOAD_ADDR_INVALID                            | 0x000b0021
Verifier Library | IMAGE_VERIFIER_ERR_FMC_LOAD_ADDR_UNALIGNED                          | 0x000b0022
Verifier Library | IMAGE_VERIFIER_ERR_FMC_ENTRY_POINT_INVALID                          | 0x000b0023
Verifier Library | IMAGE_VERIFIER_ERR_FMC_ENTRY_POINT_UNALIGNED                        | 0x000b0024
Verifier Library | IMAGE_VERIFIER_ERR_FMC_SVN_GREATER_THAN_MAX_SUPPORTED               | 0x000b0025
Verifier Library | IMAGE_VERIFIER_ERR_FMC_SVN_LESS_THAN_MIN_SUPPORTED                  | 0x000b0026
Verifier Library | IMAGE_VERIFIER_ERR_FMC_SVN_LESS_THAN_FUSE                           | 0x000b0027
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_LOAD_ADDR_INVALID                        | 0x000b0028
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_LOAD_ADDR_UNALIGNED                      | 0x000b0029
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_ENTRY_POINT_INVALID                      | 0x000b002a
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_ENTRY_POINT_UNALIGNED                    | 0x000b002b
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_SVN_GREATER_THAN_MAX_SUPPORTED           | 0x000b002c
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_SVN_LESS_THAN_MIN_SUPPORTED              | 0x000b002d
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_SVN_LESS_THAN_FUSE                       | 0x000b002e
Verifier Library | IMAGE_VERIFIER_ERR_IMAGE_LEN_MORE_THAN_BUNDLE_SIZE                  | 0x000b002f
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_LMS_PUB_KEY_INDEX_MISMATCH                | 0x000b0030
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_LMS_VERIFY_FAILURE                        | 0x000b0031
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_LMS_PUBKEY_INDEX_OUT_OF_BOUNDS            | 0x000b0032
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_LMS_SIGNATURE_INVALID                     | 0x000b0033
Verifier Library | IMAGE_VERIFIER_ERR_VEN_LMS_PUB_KEY_INDEX_OUT_OF_BOUNDS              | 0x000b0034
Verifier Library | IMAGE_VERIFIER_ERR_FMC_RUNTIME_LOAD_ADDR_OVERLAP                    | 0x000b0035
<br>
LMS Driver | DRIVER_LMS_INVALID_LMS_ALGO_TYPE                     | 0x000c0001
LMS Driver | DRIVER_LMS_INVALID_LMOTS_ALGO_TYPE                   | 0x000c0002
LMS Driver | DRIVER_LMS_INVALID_WINTERNITS_PARAM                  | 0x000c0003
LMS Driver | DRIVER_LMS_INVALID_PVALUE                            | 0x000c0004
LMS Driver | DRIVER_LMS_INVALID_HASH_WIDTH                        | 0x000c0005
LMS Driver | DRIVER_LMS_INVALID_TREE_HEIGHT                       | 0x000c0006
LMS Driver | DRIVER_LMS_INVALID_Q_VALUE                           | 0x000c0007
LMS Driver | DRIVER_LMS_INVALID_INDEX                             | 0x000c0008
LMS Driver | DRIVER_LMS_PATH_OUT_OF_BOUNDS                        | 0x000c0009
LMS Driver | DRIVER_LMS_INVALID_SIGNATURE_LENGTH                  | 0x000c000a
LMS Driver | DRIVER_LMS_INVALID_PUBLIC_KEY_LENGTH                 | 0x000c000b
LMS Driver | DRIVER_LMS_INVALID_SIGNATURE_DEPTH                   | 0x000c000c
LMS Driver | DRIVER_LMS_SIGNATURE_LMOTS_DOESNT_MATCH_PUBKEY_LMOTS | 0x000c000d
<br>
CSRNG Driver | DRIVER_CSRNG_INSTANTIATE   | 0x000d0001
CSRNG Driver | DRIVER_CSRNG_UNINSTANTIATE | 0x000d0002
CSRNG Driver | DRIVER_CSRNG_RESEED        | 0x000d0003
CSRNG Driver | DRIVER_CSRNG_GENERATE      | 0x000d0004
CSRNG Driver | DRIVER_CSRNG_UPDATE        | 0x000d0005
<br>
TRNG Driver | DRIVER_TRNG_EXT_TIMEOUT | 0x00100001
<br>
ROM | ROM_IDEVID_CSR_BUILDER_INIT_FAILURE         | 0x01000001
ROM | ROM_IDEVID_CSR_BUILDER_BUILD_FAILURE        | 0x01000002
ROM | ROM_IDEVID_INVALID_CSR                      | 0x01000003
ROM | ROM_IDEVID_CSR_VERIFICATION_FAILURE         | 0x01000004
ROM | ROM_IDEVID_CSR_OVERFLOW                     | 0x01000005
ROM | FW_PROC_MANIFEST_READ_FAILURE		          | 0x01020001
ROM | FW_PROC_INVALID_IMAGE_SIZE                  | 0x01020002
ROM | FW_PROC_MAILBOX_STATE_INCONSISTENT          | 0x01020003
ROM | FMC_ALIAS_CERT_VERIFY                       | 0x01030001
ROM | ROM_GLOBAL_NMI							  | 0x01050001
ROM | ROM_GLOBAL_EXCEPTION						  | 0x01050002
ROM | ROM_GLOBAL_PANIC							  | 0x01050003
ROM | ROM_GLOBAL_PCR_LOG_INVALID_ENTRY_ID		  |	0x01050004
ROM | ROM_GLOBAL_PCR_LOG_UNSUPPORTED_DATA_LENGTH  | 0x01050005
ROM | ROM_GLOBAL_FUSE_LOG_INVALID_ENTRY_ID		  | 0x01050006
ROM | ROM_GLOBAL_FUSE_LOG_UNSUPPORTED_DATA_LENGTH | 0x01050007
ROM | ROM_GLOBAL_UNSUPPORTED_LDEVID_TBS_SIZE      | 0x01050008
ROM | ROM_GLOBAL_UNSUPPORTED_FMCALIAS_TBS_SIZE    | 0x01050009
<br>
KAT | ROM_KAT_SHA256_DIGEST_FAILURE              | 0x90010001
KAT | ROM_KAT_SHA256_DIGEST_MISMATCH             | 0x90010002
KAT | ROM_KAT_SHA384_DIGEST_FAILURE              | 0x90020001
KAT | ROM_KAT_SHA384_DIGEST_MISMATCH             | 0x90020002
KAT | ROM_KAT_HMAC384_FAILURE                    | 0x90030001
KAT | ROM_KAT_HMAC384_TAG_MISMATCH               | 0x90030002
KAT | ROM_KAT_ECC384_SIGNATURE_GENERATE_FAILURE  | 0x90040001
KAT | ROM_KAT_ECC384_SIGNATURE_VERIFY_FAILURE    | 0x90040002
KAT | ROM_KAT_ECC384_SIGNATURE_MISMATCH          | 0x90040003
KAT | ROM_KAT_SHA384_ACC_DIGEST_START_OP_FAILURE | 0x90050001
KAT | ROM_KAT_SHA384_ACC_DIGEST_FAILURE          | 0x90050002
KAT | ROM_KAT_SHA384_ACC_DIGEST_MISMATCH         | 0x90050003
KAT | ROM_KAT_SHA1_DIGEST_FAILURE                | 0x90060001
KAT | ROM_KAT_SHA1_DIGEST_MISMATCH               | 0x90060002
KAT | ROM_KAT_LMS_DIGEST_FAILURE                 | 0x90070001
KAT | ROM_KAT_LMS_DIGEST_MISMATCH                | 0x90070002

<br><br>
# **Non-Fatal Errors**
These errors can be returned on the Update Reset flow. They are considered non-fatal when returned from the Update Reset flow. They are reported via the cptra_fw_error_non_fatal register.
Component | Error | Code |
---|---|---|
ROM | ROM_UPDATE_RESET_FLOW_MANIFEST_READ_FAILURE    | 0x01040002
ROM | ROM_UPDATE_RESET_FLOW_INVALID_FIRMWARE_COMMAND | 0x01040003
ROM | ROM_UPDATE_RESET_FLOW_MAILBOX_ACCESS_FAILURE   | 0x01040004
<br>
Verifier Library | IMAGE_VERIFIER_ERR_MANIFEST_MARKER_MISMATCH                         | 0x000b0001
Verifier Library | IMAGE_VERIFIER_ERR_MANIFEST_SIZE_MISMATCH                           | 0x000b0002
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_PUB_KEY_DIGEST_INVALID                    | 0x000b0003
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_PUB_KEY_DIGEST_FAILURE                    | 0x000b0004
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_PUB_KEY_DIGEST_MISMATCH                   | 0x000b0005
Verifier Library | IMAGE_VERIFIER_ERR_OWNER_PUB_KEY_DIGEST_FAILURE                     | 0x000b0006
Verifier Library | IMAGE_VERIFIER_ERR_OWNER_PUB_KEY_DIGEST_MISMATCH                    | 0x000b0007
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_ECC_PUB_KEY_INDEX_OUT_OF_BOUNDS           | 0x000b0008
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_ECC_PUB_KEY_REVOKED                       | 0x000b0009
Verifier Library | IMAGE_VERIFIER_ERR_HEADER_DIGEST_FAILURE                            | 0x000b000a
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_ECC_VERIFY_FAILURE                        | 0x000b000b
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_ECC_SIGNATURE_INVALID                     | 0x000b000c
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_ECC_PUB_KEY_INDEX_MISMATCH                | 0x000b000d
Verifier Library | IMAGE_VERIFIER_ERR_OWNER_ECC_VERIFY_FAILURE                         | 0x000b000e
Verifier Library | IMAGE_VERIFIER_ERR_OWNER_ECC_SIGNATURE_INVALID                      | 0x000b000f
Verifier Library | IMAGE_VERIFIER_ERR_TOC_ENTRY_COUNT_INVALID                          | 0x000b0010
Verifier Library | IMAGE_VERIFIER_ERR_TOC_DIGEST_FAILURES                              | 0x000b0011
Verifier Library | IMAGE_VERIFIER_ERR_TOC_DIGEST_MISMATCH                              | 0x000b0012
Verifier Library | IMAGE_VERIFIER_ERR_FMC_DIGEST_FAILURE                               | 0x000b0013
Verifier Library | IMAGE_VERIFIER_ERR_FMC_DIGEST_MISMATCH                              | 0x000b0014
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_DIGEST_FAILURE                           | 0x000b0015
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_DIGEST_MISMATCH                          | 0x000b0016
Verifier Library | IMAGE_VERIFIER_ERR_FMC_RUNTIME_OVERLAP                              | 0x000b0017
Verifier Library | IMAGE_VERIFIER_ERR_FMC_RUNTIME_INCORRECT_ORDER                      | 0x000b0018
Verifier Library | IMAGE_VERIFIER_ERR_OWNER_PUB_KEY_DIGEST_INVALID_ARG                 | 0x000b0019
Verifier Library | IMAGE_VERIFIER_ERR_OWNER_ECC_SIGNATURE_INVALID_ARG                  | 0x000b001a
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_PUB_KEY_DIGEST_INVALID_ARG                | 0x000b001b
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_ECC_SIGNATURE_INVALID_ARG                 | 0x000b001c
Verifier Library | IMAGE_VERIFIER_ERR_UPDATE_RESET_OWNER_DIGEST_FAILURE                | 0x000b001d
Verifier Library | IMAGE_VERIFIER_ERR_UPDATE_RESET_VENDOR_PUB_KEY_IDX_MISMATCH         | 0x000b001e
Verifier Library | IMAGE_VERIFIER_ERR_UPDATE_RESET_FMC_DIGEST_MISMATCH                 | 0x000b001f
Verifier Library | IMAGE_VERIFIER_ERR_FMC_LOAD_ADDR_INVALID                            | 0x000b0021
Verifier Library | IMAGE_VERIFIER_ERR_FMC_LOAD_ADDR_UNALIGNED                          | 0x000b0022
Verifier Library | IMAGE_VERIFIER_ERR_FMC_ENTRY_POINT_INVALID                          | 0x000b0023
Verifier Library | IMAGE_VERIFIER_ERR_FMC_ENTRY_POINT_UNALIGNED                        | 0x000b0024
Verifier Library | IMAGE_VERIFIER_ERR_FMC_SVN_GREATER_THAN_MAX_SUPPORTED               | 0x000b0025
Verifier Library | IMAGE_VERIFIER_ERR_FMC_SVN_LESS_THAN_MIN_SUPPORTED                  | 0x000b0026
Verifier Library | IMAGE_VERIFIER_ERR_FMC_SVN_LESS_THAN_FUSE                           | 0x000b0027
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_LOAD_ADDR_INVALID                        | 0x000b0028
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_LOAD_ADDR_UNALIGNED                      | 0x000b0029
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_ENTRY_POINT_INVALID                      | 0x000b002a
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_ENTRY_POINT_UNALIGNED                    | 0x000b002b
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_SVN_GREATER_THAN_MAX_SUPPORTED           | 0x000b002c
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_SVN_LESS_THAN_MIN_SUPPORTED              | 0x000b002d
Verifier Library | IMAGE_VERIFIER_ERR_RUNTIME_SVN_LESS_THAN_FUSE                       | 0x000b002e
Verifier Library | IMAGE_VERIFIER_ERR_IMAGE_LEN_MORE_THAN_BUNDLE_SIZE                  | 0x000b002f
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_LMS_PUB_KEY_INDEX_MISMATCH                | 0x000b0030
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_LMS_VERIFY_FAILURE                        | 0x000b0031
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_LMS_PUBKEY_INDEX_OUT_OF_BOUNDS            | 0x000b0032
Verifier Library | IMAGE_VERIFIER_ERR_VENDOR_LMS_SIGNATURE_INVALID                     | 0x000b0033
Verifier Library | IMAGE_VERIFIER_ERR_VEN_LMS_PUB_KEY_INDEX_OUT_OF_BOUNDS              | 0x000b0034
Verifier Library | IMAGE_VERIFIER_ERR_FMC_RUNTIME_LOAD_ADDR_OVERLAP                    | 0x000b0035
<br>
Mailbox Driver | DRIVER_MAILBOX_INVALID_STATE	 | 0x00080001