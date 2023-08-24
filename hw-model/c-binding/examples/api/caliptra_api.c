// Licensed under the Apache-2.0 license
#include <stdio.h>
#include <string.h>
#include <errno.h>
#include "../../../../hw-latest/caliptra-rtl/src/soc_ifc/rtl/caliptra_top_reg.h"
#include "caliptra_api.h"
#include "caliptra_fuses.h"
#include "caliptra_mbox.h"
// Check with jordan
#include "../../../../libcaliptra/inc/caliptra_if.h"



int caliptra_init_fuses(struct caliptra_model *model, struct caliptra_fuses *fuses)
{
    // Parameter check
    if (!model || !fuses) {
        return -EINVAL;
    }

    // Check whether caliptra is ready for fuses
    if (!caliptra_model_ready_for_fuses(model))
        return -EPERM;

    // Write Fuses
    caliptra_fuse_array_write(model, GENERIC_AND_FUSE_REG_FUSE_UDS_SEED_0, fuses->uds_seed, sizeof(fuses->uds_seed));
    caliptra_fuse_array_write(model, GENERIC_AND_FUSE_REG_FUSE_FIELD_ENTROPY_0, fuses->field_entropy, sizeof(fuses->field_entropy));
    caliptra_fuse_array_write(model, GENERIC_AND_FUSE_REG_FUSE_KEY_MANIFEST_PK_HASH_0, fuses->key_manifest_pk_hash, sizeof(fuses->key_manifest_pk_hash));
    caliptra_fuse_write(model, GENERIC_AND_FUSE_REG_FUSE_KEY_MANIFEST_PK_HASH_MASK, fuses->key_manifest_pk_hash_mask);
    caliptra_fuse_array_write(model, GENERIC_AND_FUSE_REG_FUSE_OWNER_PK_HASH_0, fuses->owner_pk_hash, sizeof(fuses->owner_pk_hash));
    caliptra_fuse_write(model, GENERIC_AND_FUSE_REG_FUSE_FMC_KEY_MANIFEST_SVN, fuses->fmc_key_manifest_svn);
    caliptra_fuse_array_write(model, GENERIC_AND_FUSE_REG_FUSE_FMC_KEY_MANIFEST_SVN, fuses->runtime_svn, sizeof(fuses->runtime_svn));
    caliptra_fuse_write(model, GENERIC_AND_FUSE_REG_FUSE_ANTI_ROLLBACK_DISABLE, (uint32_t)fuses->anti_rollback_disable);
    caliptra_fuse_array_write(model, GENERIC_AND_FUSE_REG_FUSE_IDEVID_CERT_ATTR_0, fuses->idevid_cert_attr, sizeof(fuses->idevid_cert_attr));
    caliptra_fuse_array_write(model, GENERIC_AND_FUSE_REG_FUSE_IDEVID_MANUF_HSM_ID_0, fuses->idevid_manuf_hsm_id, sizeof(fuses->idevid_manuf_hsm_id));
    caliptra_fuse_write(model, GENERIC_AND_FUSE_REG_FUSE_LIFE_CYCLE, (uint32_t)fuses->life_cycle);

    // Write to Caliptra Fuse Done
    caliptra_model_apb_write_u32(model, CALIPTRA_TOP_REG_GENERIC_AND_FUSE_REG_CPTRA_FUSE_WR_DONE, 1);

    // It shouldn`t be longer ready for fuses
    if (caliptra_model_ready_for_fuses(model))
        return -EIO;

    return 0;
}

int caliptra_bootfsm_go(struct caliptra_model *model)
{
    // Parameter check
    if (!model) {
        return -EINVAL;
    }

    // Write BOOTFSM_GO Register
    caliptra_model_apb_write_u32(model, CALIPTRA_TOP_REG_GENERIC_AND_FUSE_REG_CPTRA_BOOTFSM_GO, 1);

    return 0;
}

int caliptra_mailbox_write_fifo(struct caliptra_model *model, struct caliptra_buffer *buffer)
{
    printf("******************************Test1\n");
    fflush(stdout);
    // Check against max size
    const uint32_t MBOX_SIZE = (128u * 1024u);
    if (buffer->len > MBOX_SIZE) {
        return -EINVAL;
    }

    printf("******************************Test2\n");
    fflush(stdout);

    // Write DLEN
    caliptra_mbox_write_dlen(model, buffer->len);

    printf("******************************Test3\n");
    fflush(stdout);

    uint32_t remaining_len = buffer->len;
    uint32_t *data_dw = (uint32_t *)buffer->data;

    printf("******************************Test4\n");
    fflush(stdout);

    // Copy DWord multiples
    while (remaining_len > sizeof(uint32_t)) {
        caliptra_mbox_write(model, MBOX_CSR_MBOX_DATAIN, *data_dw++);
        remaining_len -= sizeof(uint32_t);
    }

    printf("******************************Test5\n");
    fflush(stdout);

    // if un-aligned dword reminder...
    if (remaining_len) {
        uint32_t data = 0;
        memcpy(&data, data_dw, remaining_len);
        caliptra_mbox_write(model, MBOX_CSR_MBOX_DATAIN, data);
    }

    printf("******************************Test6\n");
    fflush(stdout);

    return 0;
}

static int caliptra_mailbox_read_buffer(struct caliptra_model *model, struct caliptra_buffer *buffer)
{

    // Check we have enough room in the buffer
    if (buffer->len < caliptra_mbox_read_dlen(model) || !buffer->data)
       return -EINVAL;

    uint32_t remaining_len = caliptra_mbox_read_dlen(model);
    uint32_t *data_dw = (uint32_t *)buffer->data;

    // Copy DWord multiples
    while (remaining_len > sizeof(uint32_t)) {
        *data_dw++ = caliptra_mbox_read(model, MBOX_CSR_MBOX_DATAOUT);
        remaining_len -= sizeof(uint32_t);
    }

    // if un-aligned dword reminder...
    if (remaining_len) {
        uint32_t data = caliptra_mbox_read(model, MBOX_CSR_MBOX_DATAOUT);
        memcpy(data_dw, &data, remaining_len);
    }

    return 0;
}


int caliptra_mailbox_execute(struct caliptra_model *model, uint32_t cmd, struct caliptra_buffer *mbox_tx_buffer, struct caliptra_buffer *mbox_rx_buffer)
{
    // Parameter check
    if (!model) {
        return -EINVAL;
    }

    // If mbox already locked return
    if (caliptra_mbox_is_lock(model)) {
        return -EBUSY;
    }

    // Write Cmd and Tx Buffer
    caliptra_mbox_write_cmd(model, cmd);
    caliptra_mailbox_write_fifo(model, mbox_tx_buffer);

    // Set Execute bit
    caliptra_mbox_write_execute(model, true);

    // Keep stepping until mbox status is busy
    while(caliptra_mbox_read_status(model) == CALIPTRA_MBOX_STATUS_BUSY)
        caliptra_model_step(model);

    // Check the Mailbox Status
    uint32_t status = caliptra_mbox_read_status(model);
    if (status == CALIPTRA_MBOX_STATUS_CMD_FAILURE) {
        caliptra_mbox_write_execute(model, false);
            printf("******************************Mailbox1\n");
    fflush(stdout);
        return -EIO;
    } else if(status == CALIPTRA_MBOX_STATUS_CMD_COMPLETE) {
        caliptra_mbox_write_execute(model, false);
                printf("******************************Mailbox2\n");
    fflush(stdout);
        return 0;
    } else if (status != CALIPTRA_MBOX_STATUS_DATA_READY) {
        return -EIO;
    }

    // Read Mbox out Data Len
    uint32_t dlen = caliptra_mbox_read_dlen(model);

    // Read Buffer
    caliptra_mailbox_read_buffer(model, mbox_rx_buffer);

    // Execute False
    caliptra_mbox_write_execute(model, false);

    // mbox_fsm_ps isn't updated immediately after execute is cleared (!?),
    // so step an extra clock cycle to wait for fm_ps to update
    caliptra_model_step(model);

    if (caliptra_mbox_read_status_fsm(model) != CALIPTRA_MBOX_STATUS_FSM_IDLE)
        return -EIO;

    return 0;
}

int caliptra_upload_fw(struct caliptra_model *model, struct caliptra_buffer *fw_buffer)
{
    const uint32_t FW_LOAD_CMD_OPCODE = 0x46574C44u;
    return caliptra_mailbox_execute(model, FW_LOAD_CMD_OPCODE, fw_buffer, NULL);
}

static uint32_t calculate_caliptra_checksum(uint32_t cmd, uint8_t *buffer, uint32_t len)
{
    uint32_t i, sum = 0;

    if ((buffer == NULL) && (len != 0))
    {
        // Don't respect bad parameters
        return 0;
    }

    for (i = 0; i < sizeof(uint32_t); i++)
    {
        sum += ((uint8_t*)(&cmd))[i];
    }

    for (i = 0; i < len; i++)
    {
        sum += buffer[i];
    }

    return (0 - sum);
}

int caliptra_get_fips_version(struct caliptra_model *model,struct caliptra_fips_version *version)
{
    // Parameter check
    if (version == NULL)
        return -EINVAL;

    const uint32_t OP_FIPS_VERSION= 0x46505652;
    int checksum = calculate_caliptra_checksum(OP_FIPS_VERSION, NULL, 0);

    struct caliptra_buffer in_buf = {
        .data = (uint8_t *)&checksum,
        .len = sizeof(checksum),
    };
    struct caliptra_buffer out_buf = {
        .data = (uint8_t *)version,
        .len = sizeof(struct caliptra_fips_version),
    };

    int status = caliptra_mailbox_execute(model,OP_FIPS_VERSION, &in_buf, &out_buf);

    if (!status)
    {
        return status;
    }

    return status;
}

int caliptra_get_profile(struct caliptra_model *model, struct caliptra_buffer *fw_buffer,uint32_t statusCheckRead, struct caliptra_output *test)
{
    const uint32_t OP_INVOKE_DPE_COMMAND = 0x44504543;
    uint32_t *status;
    int mStatus;
    const uint32_t error_code = 0x3003000c;
       struct caliptra_buffer out_buf = {
        .data = (uint8_t *)test,
        .len = sizeof(struct caliptra_output),
    };
    mStatus = caliptra_mailbox_execute(model,OP_INVOKE_DPE_COMMAND, fw_buffer, &out_buf);
    printf("***********Profile 1**********\n");
    fflush(stdout);
   for (size_t i = 0; i < out_buf.len; i++) {
    printf("%02x ", out_buf.data[i]); // Printing bytes as hexadecimal
    fflush(stdout);
}
printf("\n");
    fflush(stdout);
     struct caliptra_fips_version fips_version;
     caliptra_get_fips_version(model,&fips_version);
    printf("Checksum: 0x%08x\n", fips_version.cpl.checksum);
    fflush(stdout);
    printf("Mode: %u\n", fips_version.mode);
    fflush(stdout);
    printf("FIPS Revision: %u.%u.%u\n", fips_version.fips_rev[0], fips_version.fips_rev[1], fips_version.fips_rev[2]);
    fflush(stdout);
    printf("Name: %s\n", fips_version.name);
    fflush(stdout);
    printf("***********profile 2**********\n");
    fflush(stdout);
    status = (uint32_t *)malloc(10 * sizeof(uint32_t));
    caliptra_model_apb_read_u32(model,error_code, status);
    printf("%x\n",*status);
    fflush(stdout);
    statusCheckRead = *status;
    return mStatus;
}

