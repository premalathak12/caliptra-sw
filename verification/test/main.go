package main

/*
#include <stdint.h>
#include <stdlib.h>

typedef struct caliptra_buffer {
    const uint8_t *data;
    uintptr_t len;
} caliptra_buffer;


// Declare a C function to create a CommandHdr struct in Go and return a pointer to it
void* createCommandHdr(uint32_t magic, uint32_t cmd, uint16_t major, uint16_t minor) {
    struct CommandHdr {
        uint32_t magic;
        uint32_t cmd;
        struct {
            uint16_t major_version;
            uint16_t minor_version;
        } profile;
    };
    
    struct CommandHdr* cmdHdr = (struct CommandHdr*)malloc(sizeof(struct CommandHdr));
    if (cmdHdr != NULL) {
        cmdHdr->magic = magic;
        cmdHdr->cmd = cmd;
        cmdHdr->profile.major_version = major;
        cmdHdr->profile.minor_version = minor;
    }
    
    return cmdHdr;
}

// Declare a C function to free the memory allocated for CommandHdr
void freeCommandHdr(void* cmdHdr) {
    free(cmdHdr);
}
*/
import "C"

import (
	"unsafe"
	"fmt"
)

const (
	CmdMagic  uint32 = 0x44504543
	CURRENT_PROFILE_MAJOR_VERSION uint16 = 0
	CURRENT_PROFILE_MINOR_VERSION uint16 = 8
)

type CommandCode uint32

const (
	CommandGetProfile        CommandCode = 0x1
	CommandInitializeContext CommandCode = 0x7
	CommandCertifyKey        CommandCode = 0x9
	CommandDestroyContext    CommandCode = 0xf
	CommandTagTCI            CommandCode = 0x82
	CommandGetTaggedTCI      CommandCode = 0x83
)

func main() {
	// Create CommandHdr in Go and get a pointer to it
	cmdHdrPtr := C.createCommandHdr(CmdMagic, C.uint32_t(CommandGetProfile), C.uint16_t(CURRENT_PROFILE_MAJOR_VERSION), C.uint16_t(CURRENT_PROFILE_MINOR_VERSION))
	defer C.freeCommandHdr(cmdHdrPtr)

	var buffer C.caliptra_buffer
	buffer.data = (*C.uint8_t)(cmdHdrPtr)
	buffer.len = C.uintptr_t(unsafe.Sizeof(CCommandHdr{}))
}
