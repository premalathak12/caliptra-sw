# Caliptra - FMC Specification v0.5

## Version History

| Date       | Version | Description                                                                        |
| :--------- | :------ | :----------------------------------------------------------------------------------|
| 01/18/2023 | 0.1     | Document Created, Boot flow defined                                                |
| 01/31/2023 | 0.1.1   | Added Overview and Pre-Conditions sections                                         |
| 02/10/2023 | 0.2     | Incorporate feedback and decisions from Caliptra WG meetings                       |
| 02/27/2023 | 0.4     | Update for decision that Anti-Rollback will be entirely handled by ROM             |
|            |         | Add details/clarifications in FMC Boot steps including where to store artifacts    |
|            |         | Add Firmware Handoff Table (FHT) definition                                        |
| 03/21/2023 | 0.5     | Additional fields added to FHT                                                     |
| 03/29/2023 | 0.5     | Changed the value for invalid adress fields in FHT                                                     |

## Scope

Caliptra is an open-source Hardware Root of Trust for Measurement (RTM). This document is the architecture specification for Caliptra First Mutable Code (FMC).
As an architecture specification for FMC, this document describes the following topics:

1. Provide high level requirements
2. Describe FMC load and measurement flow
3. Describe FMC functionality
4. Define FMC boot flows

## Glossary

| Term                | Description                                                               |
| :------------------ | :------------------------------------------------------------------------ |
| DCCM                | Data Closely Coupled Memory                                               |
| DICE                | Device Identifier Composition Engine                                      |
| FHT                 | Firmware Handoff Table                                                    |
| FMC                 | First Mutable Code                                                        |
| FW                  | Firmware                                                                  |
| ICCM                | Instruction Closely Coupled Memory                                        |
| RoT                 | Root of Trust                                                             |
| RT                  | Runtime                                                                   |
| RTM                 | Root of Trust for Measurement                                             |
| TCI                 | Trusted Component Identifier                                              |
| SVN                 | Security Version Number                                                   |

## Overview

First Mutable Code (FMC) is the first field-updatable firmware module in the Caliptra boot sequence. It is loaded, cryptographically verified,
and executed by the Caliptra ROM.

### Pre-Conditions / Assumptions

It is assumed that the Caliptra ROM has already performed a series of steps to prepare the Caliptra environment before calling the FMC entry point. The
following is a brief overview of those expectations. Further details can be found in the Caliptra ROM Specification.

- ROM is responsible for initializing its ROM-based FIPS Crypto Module
  <br> *(Note that this requirement is dependent on the chosen FIPS boundary. It only applies if there is a discrete FIPS ROM module that is isolated from the
  rest of the ROM. This is not expected to be the case for the first generation of Caliptra.)*
- ROM is responsible for locating the image containing all of Caliptra’s mutable firmware and loading it into ICCM.
- ROM is responsible for authentication of the Manifest and each individual FW Module loaded to ICCM.
- ROM is responsible for ensuring that the Anti-Rollback Protection is enforced for all mutable firmware modules.
- ROM is responsible for creating Caliptra’s initial DICE identity and extending it with measurements of the FMC Module.
- ROM jumps to the Caliptra FMC entry point.

At the time the Caliptra FMC entry point is executed, the Caliptra memory space will look like one of the following diagrams (dependent upon selected FIPS
Crypto boundary):

<center>

<br> *Current POR: All Caliptra FW in FIPS boundary*

![Block Diagram with all Caliptra FW in FIPS boundary](doc/diagrams/Caliptra_FMC_Internal_FW_FIPS.svg)

<br> *Alternate: Caliptra ROM and FW each have discrete FIPS modules*

![Alternate block Diagram with discrete FIPS Modules in both ROM and FW](doc/diagrams/Caliptra_FMC_Discrete_FW_FIPS.svg)

<br> *Alternate: Caliptra ROM implements FIPS Module used by all other components*

![Alternate block Diagram with singlge discrete FIPS Module ROM](doc/diagrams/Caliptra_FMC_ROM_FIPS.svg )

</center>

### FMC Responsibilities

FMC can be thought of as essentially a small, mutable extension of the ROM. Its primary purpose is to bridge execution from the immutable ROM code, prepare the
environment for the main runtime firmware, and then execute that runtime firmware. As such, the code should be kept to the bare minimum needed to perform that
task. “Feature-creep” in this area is undesirable, and all efforts shall be made to avoid it.

- FMC must initialize the FW-based FIPS Crypto Module that is loaded alongside it. This initialization must be completed before any cryptographic operations can
  be performed.
  <br> *(Note that this requirement is dependent on the chosen FIPS boundary. It only applies if there is a discrete FIPS firmware module that is loaded
  separately from the FMC FW module. This is not expected to be the case for the first generation of Caliptra.)*
- FMC must measure the Runtime Firmware Module using services from the FIPS Crypto Module.
- FMC must extend the Caliptra DICE identity to the Runtime Firmware Module using FIPS Crypto services, generating artifacts CDI<sub>RT</sub>,
  AliasKeyPair<sub>RT</sub>, and certifying PublicKey<sub>RT</sub>.
- At any time during its flow, the FMC *MAY* be required to execute a workaround for an RTL or ROM bug that was discovered after Caliptra hardware was frozen.
  The nature, feasibility, and timing of such a workaround will be dependent on the specific details of the bug.
- FMC must make the CDI<sub>RT</sub>, AliasKeyPair<sub>RT</sub>, and Cert<sub>RT</sub> available to the Runtime Firmware Module, while making its own
  CDI<sub>FMC</sub> and PrivateKey<sub>FMC</sub> unavailable.
- FMC must execute the Runtime Firmware Module.

## Firmware Handoff Table

The Firmware Handoff Table is a data structure that is resident at a well-known location in DCCM. It is initially populated by ROM and modified by FMC as a way
to pass parameters and configuration information from one firmware layer to the next.

Table revisions with the same Major Version must remain backward compatible (i.e. fields may be added to the end of the table, or fields may be deprecated, but
fields may not be changed or removed). Table revisions with different Major Versions may or may not be compatible.

*Note: All fields are little-endian unless otherwise specified.*

| Field                 | Size (bytes) | Written By | Description                                                                                              |
|:----------------------|:-------------|:-----------|:---------------------------------------------------------------------------------------------------------|
| fht_marker            | 4            | ROM        | Magic Number marking start of FHT. Value must be 0x54484643, ‘CFHT’ when viewed as little-endian ASCII.  |
| fht_major_ver         | 2            | ROM        | Major version of FHT.                                                                                    |
| fht_minor_ver         | 2            | ROM, FMC   | Minor version of FHT. Initially written by ROM but may be changed to a higher version by FMC.            |
| manifest_load_addr    | 4            | ROM        | Physical base address of Manifest in DCCM SRAM.                                                          |
| fips_fw_load_addr_idx | 4            | ROM        | Index of base address of FIPS Module in ROM or ICCM SRAM. May be 0xFF if there is no discrete module.    |
| rt_fw_load_addr_idx   | 4            | ROM        | Index of load address of Runtime FW Module value in data vault.SRAM.                                                 |
| rt_fw_entry_point_idx | 4            | ROM        | Index of entry point of Runtime FW Module value in data vault. SRAM.                                                           |
| fmc_tci_dv_idx        | 1            | ROM        | Index of FMC TCI value in the Data Vault.                                                                |
| fmc_cdi_kv_idx        | 1            | ROM        | Index of FMC CDI value in the Key Vault. Value of 0xFF indicates not present.                            |
| fmc_priv_key_kv_idx   | 1            | ROM        | Index of FMC Private Alias Key in the Key Vault.                                                         |
| fmc_pub_key_x_dv_idx  | 1            | ROM        | Index of FMC Public Alias Key X Coordinate in the Data Vault.                                            |
| fmc_pub_key_y_dv_idx  | 1            | ROM        | Index of FMC Public Alias Key Y Coordinate in the Data Vault                                             |
| fmc_cert_sig_r_dv_idx | 1            | ROM        | Index of FMC Certificate Signature R Component in the Data Vault.                                        |
| fmc_cert_sig_s_dv_idx | 1            | ROM        | Index of FMC Certificate Signature S Component in the Data Vault.                                        |
| fmc_svn_dv_idx        | 1            | ROM        | Index of FMC SVN value in the Data Vault.                                                                |
| rt_tci_dv_idx         | 1            | ROM        | Index of RT TCI value in the Data Vault.                                                                 |
| rt_cdi_kv_idx         | 1            | FMC        | Index of RT CDI value in the Key Vault.                                                                  |
| rt_priv_key_kv_idx    | 1            | FMC        | Index of RT Private Alias Key in the Key Vault.                                                          |
| rt_pub_key_x_dv_idx   | 1            | FMC        | Index of RT Public Alias Key X Coordinate in the Data Vault.                                             |
| rt_pub_key_y_dv_idx   | 1            | FMC        | Index of RT Public Alias Key Y Coordinate in the Data Vault.                                             |
| rt_cert_sig_r_dv_idx  | 1            | FMC        | Index of RT Certificate Signature R Component in the Data Vault.                                         |
| rt_cert_sig_s_dv_idx  | 1            | FMC        | Index of RT Certificate Signature S Component in the Data Vault.                                         |
| rt_svn_dv_idx         | 1            | FMC        | Index of RT SVN value in the Data Vault.                                                                 |
| reserved              | 20           |            | Reserved for future use.                                                                                 |

*FHT is currently defined to be 60 bytes in length.*

### fht_marker

This is a "magic number" used to identify the start of the table, allowing the FMC or RT firmware modules to determine that the FHT has been populated. The
expected value 0x54484643 will appear as ASCII ‘CFHT’ when viewed as a hex dump.

### fht_major_ver & fht_minor_ver

The Major and Minor version numbers of the Firmware Handoff Table. All FHT versions with the same Major version number must remain backward compatible.
Therefore, fields must remain at constant offsets, and no fields may be redefined. It is possible to deprecate existing fields or define new fields within the
reserved space at the end of the table by incrementing the Minor version number

For example, a Caliptra ROM is be frozen with FHT version 1.0. During later stages of development, it is found that an additional 4 byte data field must be
passed from FMC to Runtime. During boot, the ROM will populate the FHT as version 1.0. When FMC executes, it will update the table version to 1.1 and add the
additional data to the first 4 bytes of the reserved space at the end of the FHT.

### manifest_load_addr

This is the physical address of the location in SRAM where ROM has placed a complete copy of the Firmware Manifest. This must remain resident such that firmware
is able to re-run firmware integrity checks on-demand (required by FIPS 140-3).

### fips_fw_load_addr_idx

*Future feature, not currently supported.* This field provides the index of the DV entry that stores the physical address of the location in ROM or SRAM where a discrete FIPS Crypto module resides. If a
discrete FIPS module does not exist, then this field shall be 0xFF and ROM, FMC, and RT FW must all carry their own code for accessing crypto resources and
keys.

### rt_fw_load_addr_idx

This field provides the index of the DV entry that stores the physical address of the location in ICCM SRAM where ROM has placed the authenticated Runtime Firmware module.

### rt_fw_entry_point_idx

This field provides the index of the DV entry that stores the physical address of the Entry Point of Runtime FW Module in ICCM SRAM.

### fmc_tci_dv_idx

This field provides the index into the Data Vault where the TCI<sub>FMC</sub> is stored. TCI<sub>FMC</sub> is a SHA-384 Hash of the FMC Module.

### fmc_cdi_kv_idx

This field provides the index into the Key Vault where the CDI<sub>FMC</sub> is stored.

### fmc_priv_key_kv_idx

This field provides the index into the Key Vault where the PrivateKey<sub>FMC</sub> is stored.

### fmc_pub_key_x_dv_idx, fmc_pub_key_y_dv_idx

These fields provide the indices into the Data Vault where the PublicKey<sub>FMC</sub> X and Y coordinates are stored.

### fmc_cert_sig_r_dv_idx, fmc_cert_sig_s_dv_idx

These fields provide the indices into the Data Vault where the Cert<sub>FMC</sub> signature R and S components are stored.

### fmc_svn_dv_idx

This field provides the index into the Data Vault where the SVN<sub>FMC</sub> is stored.

### rt_tci_dv_idx

This field provides the index into the Data Vault where the TCI<sub>RT</sub> is stored. TCI<sub>RT</sub> is a SHA-384 Hash of the RT FW Module.

### rt_cdi_kv_idx

This field provides the index into the Key Vault where the CDI<sub>RT</sub> is stored.

### rt_priv_key_kv_idx

This field provides the index into the Key Vault where the PrivateKey<sub>RT</sub> is stored.

### rt_pub_key_x_dv_idx, rt_pub_key_y_dv_idx

These fields provide the indices into the Data Vault where the PublicKey<sub>RT</sub> X and Y coordinates are stored.

### rt_cert_sig_r_dv_idx, rt_cert_sig_s_dv_idx

These fields provide the indices into the Data Vault where the Cert<sub>RT</sub> signature R and S components are stored.

### rt_svn_dv_idx

This field provides the index into the Data Vault where the SVN<sub>RT</sub> is stored.

### reserved

This area is reserved for definition of additional fields that may be added during Minor version updates of the FHT.

## FMC Boot Flow

The following list of steps are to be performed by FMC on each boot when ROM jumps to its entry point:

1. FMC locates the Firmware Handoff Table (FHT) responsible for passing vital configuration and other data from one firmware layer to the next. This is found
   at well-known address CALIPTRA_FHT_ADDR.
1. FMC sanity checks FHT by verifying that fht.fht_marker == ‘CFHT’ and version is known/supported by FMC.
1. FMC locates the discrete FW-based FIPS Crypto Module in ICCM using fht.fips_fw_base_addr (if not 0xFFFF_FFFF) and calls its initialization routine. Otherwise FMC
   utilizes the ROM-based FIPS Crypto Module or its own internal FIPS Crypto services in implementations without a discrete FW-based FIPS Crypto Module.
1. FMC locates the Manifest at fht.manifest_load_addr.
1. FMC reads the measurement of the Runtime FW Module, TCI<sub>RT</sub>, from the Data Vault that has previously been validated by ROM.
1. FMC extends Caliptra PCR registers with TCI<sub>RT</sub>.
1. FMC derives CDI<sub>RT</sub> from CDI<sub>FMC</sub> mixed with TCI<sub>RT</sub> and stores it in the Key Vault.
1. FMC updates fht.rt_cdi_kv_idx in the FHT.
1. FMC derives AliasKeyPair<sub>RT</sub> from CDI<sub>RT</sub>. The Private Key is stored in the Key Vault while the Public Key X and Y coordinates are stored
   in the Data Vault.
1. FMC updates fht.rt_priv_key_kv_idx, fht.rt_pub_key_x_dv_idx, and fht.rt_pub_key_y_dv_idx in the FHT.
1. FMC generates an x509 certificate with PubKey<sub>RT</sub> as the subject and signed by PrivKey<sub>FMC</sub>.
1. FMC stores the Cert<sub>RT</sub> signature in the Data Vault.
1. FMC updates fht.rt_cert_sig_r_dv_idx and fht.rt_cert_sig_r_dv_idx in the FHT.
1. FMC ensures that CDI<sub>FMC</sub> and PrivateKey<sub>FMC</sub> are locked to block further usage until the next boot.
1. FMC locates the Runtime FW Module in ICCM at fht.rt_fw_load_addr.
1. FMC jumps to the Runtime FW Module entry point at fht.rt_fw_entry_point.

<center>

<br> *FMC Boot Sequence*

```mermaid
sequenceDiagram
    participant ROM as Boot ROM
    participant FIPS as FIPS Crypto
    participant FMC as FMC
    participant RT as Runtime FW

    ROM->>+ROM: Early ROM Flow
    ROM->>ROM: Authenticate FW Modules
    ROM->>ROM: Enforce Anti-Rollback Protection
    ROM->>ROM: Create fht
    ROM->>-FMC: Jump to FMC Entry Point

    FMC->>+FMC: SanityCheckFht(CALIPTRA_FHT_ADDR)
    FMC->>FMC: LocateFipsFw(fht) (if needed)
    FMC->>+FIPS: InitFipsFw() (if needed)
    FIPS-->>-FMC: return()
    FMC->>FMC: LocateManifest(fht)
    FMC->>FMC: GetRtMeasurement(fht.rt_tci_dv_idx)
    FMC->>+FIPS: ExtendPcr(PCR_IDX_RT, RtTci)
    FIPS-->>-FMC: return()

    rect rgba(0, 0, 200, .2)
    note over FIPS, FMC: DICE-related derivations will be<br> defined in greater detail later

    FMC->>+FIPS: DeriveCdi(fht.FmcCdiKvIdx, RtTci)
    FIPS-->>-FMC: return(fht.rt_cdi_kv_idx)
    FMC->>+FIPS: DeriveKeyPair(fht.rt_cdi_kv_idx)
    FIPS-->>-FMC: return(fht.rt_priv_key_kv_idx,<br> fht.rt_pub_key_x_dv_idx,<br> fht.rt_pub_key_y_dv_idx)
    FMC->>+FIPS: CertifyKey(fht.rt_pub_key_x_dv_idx,<br> fht.rt_pub_key_y_dv_idx,<br> fht.fmc_priv_key_kv_idx)
    FIPS-->>-FMC: return(fht.rt_cert_sig_r_dv_idx, fht.rt_cert_sig_s_dv_idx)
    FMC->>+FIPS: LockKey(fht.fmc_cdi_kv_idx)
    FIPS-->>-FMC: return()
    FMC->>+FIPS: LockKey(fht.fmc_priv_key_kv_idx)
    FIPS-->>-FMC: return()

    end %% rect

    FMC->>FMC: LocateRtFw(fht)
    FMC->>-RT: Jump to Runtime Entry Point

    activate RT
    RT->>RT: RtFwInitFlow()
    deactivate RT
```

</center>

## FMC Firmware Update Flow

**TBD: Is this section needed?**

## FMC Recovery Flow

*Section to be filled in.*

## Opens and ToDo Items

- Document FW Update flow for FMC. TBD if there is any difference that is needed.
  - Impact of Hitless update?
- Determine which PCR registers should be used for RT measurement
  - One for current FW
  - One for journey
- Document error handling and recovery flows (resiliency)

## Future

- Current POR is for FIPS Crypto boundary to encompass all of Caliptra FW, including ROM, FMC, and Runtime. With this boundary, there is no need for any
  dedicated crypto module, and each layer of FW will include the library code it needs to access any required crypto functionality. In the future, if a more
  strict FIPS boundary is created, FMC will need to be changed to handle crypto operations differently. Depending on where it is implemented, it may or may not
  have to initilize the FIPS Crypto module, and it may need to use a different calling convention.