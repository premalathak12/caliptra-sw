// Licensed under the Apache-2.0 license.
use super::CommandExecution;
use crate::{
    context::{ActiveContextArgs, ContextHandle, ContextState, ContextType},
    dpe_instance::DpeInstance,
    response::{DeriveChildResp, DpeErrorCode, Response},
    tci::TciMeasurement,
    DPE_PROFILE,
};
use crypto::Crypto;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, zerocopy::FromBytes)]
#[cfg_attr(test, derive(zerocopy::AsBytes))]
pub struct DeriveChildCmd {
    pub handle: ContextHandle,
    pub data: [u8; DPE_PROFILE.get_hash_size()],
    pub flags: u32,
    pub tci_type: u32,
    pub target_locality: u32,
}

impl DeriveChildCmd {
    pub const INTERNAL_INPUT_INFO: u32 = 1 << 31;
    pub const INTERNAL_INPUT_DICE: u32 = 1 << 30;
    pub const RETAIN_PARENT: u32 = 1 << 29;
    pub const MAKE_DEFAULT: u32 = 1 << 28;
    pub const CHANGE_LOCALITY: u32 = 1 << 27;

    const fn uses_internal_info_input(&self) -> bool {
        self.flags & Self::INTERNAL_INPUT_INFO != 0
    }

    const fn uses_internal_dice_input(&self) -> bool {
        self.flags & Self::INTERNAL_INPUT_DICE != 0
    }

    const fn retains_parent(&self) -> bool {
        self.flags & Self::RETAIN_PARENT != 0
    }

    const fn makes_default(&self) -> bool {
        self.flags & Self::MAKE_DEFAULT != 0
    }

    const fn changes_locality(&self) -> bool {
        self.flags & Self::CHANGE_LOCALITY != 0
    }

    /// Check if this will result in two default contexts in the same locality.
    ///
    /// There can only be one default context in each locality at any given time. A default context
    /// has a specific handle. If there are multiple, the DPE instance will not be able to
    /// differentiate between them.
    ///
    /// # Arguments
    ///
    /// * `parent_idx` - Index of the soon-to-be parent.
    /// * `default_context_idx` - Index of the current default context, if there is one.
    fn safe_to_make_default(&self, parent_idx: usize, default_context_idx: Option<usize>) -> bool {
        if let Some(default_idx) = default_context_idx {
            if default_idx != parent_idx || self.retains_parent() {
                return false;
            }
        }
        true
    }
}

impl<C: Crypto> CommandExecution<C> for DeriveChildCmd {
    fn execute(&self, dpe: &mut DpeInstance<C>, locality: u32) -> Result<Response, DpeErrorCode> {
        // Make sure the operation is supported.
        if !dpe.support.internal_info && self.uses_internal_info_input()
            || !dpe.support.internal_dice && self.uses_internal_dice_input()
        {
            return Err(DpeErrorCode::InvalidArgument);
        }

        let parent_idx = dpe
            .get_active_context_pos(&self.handle, locality)
            .ok_or(DpeErrorCode::InvalidHandle)?;
        let child_idx = dpe
            .get_next_inactive_context_pos()
            .ok_or(DpeErrorCode::MaxTcis)?;

        if self.uses_internal_info_input() {
            dpe.contexts[parent_idx].uses_internal_dpe_info = true;
        }

        let target_locality = if !self.changes_locality() {
            locality
        } else {
            // Make sure the target locality is valid.
            if !dpe.localities.iter().any(|&l| l == self.target_locality) {
                return Err(DpeErrorCode::InvalidLocality);
            }
            self.target_locality
        };

        // Make sure it can be the default if it is supposed to be.
        if self.makes_default() {
            let default_context_idx =
                dpe.get_active_context_pos(&ContextHandle::default(), target_locality);

            if !self.safe_to_make_default(parent_idx, default_context_idx) {
                return Err(DpeErrorCode::InvalidArgument);
            }
        }

        let child_handle = if self.makes_default() {
            ContextHandle::default()
        } else {
            dpe.generate_new_handle()?
        };

        if !self.retains_parent() {
            dpe.contexts[parent_idx].state = ContextState::Retired;
            dpe.contexts[parent_idx].handle = ContextHandle([0xff; ContextHandle::SIZE]);
        } else if !dpe.contexts[parent_idx].handle.is_default() {
            dpe.contexts[parent_idx].handle = dpe.generate_new_handle()?;
        }

        dpe.contexts[child_idx].activate(&ActiveContextArgs {
            context_type: ContextType::Normal,
            locality: target_locality,
            handle: &child_handle,
            tci_type: self.tci_type,
            parent_idx: parent_idx as u8,
        });

        dpe.add_tci_measurement(child_idx, &TciMeasurement(self.data), target_locality)?;

        // Add child to the parent's list of children.
        dpe.contexts[parent_idx].add_child(child_idx)?;

        Ok(Response::DeriveChild(DeriveChildResp {
            handle: child_handle,
            parent_handle: dpe.contexts[parent_idx].handle,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        commands::{tests::TEST_DIGEST, Command, CommandHdr, InitCtxCmd},
        dpe_instance::tests::{SIMULATION_HANDLE, TEST_LOCALITIES},
        support::Support,
        MAX_HANDLES,
    };
    use crypto::OpensslCrypto;
    use zerocopy::AsBytes;

    const TEST_DERIVE_CHILD_CMD: DeriveChildCmd = DeriveChildCmd {
        handle: SIMULATION_HANDLE,
        data: TEST_DIGEST,
        flags: 0x1234_5678,
        tci_type: 0x9876_5432,
        target_locality: 0x10CA_1171,
    };

    #[test]
    fn test_deserialize_derive_child() {
        let mut command = CommandHdr::new(Command::DeriveChild(TEST_DERIVE_CHILD_CMD))
            .as_bytes()
            .to_vec();
        command.extend(TEST_DERIVE_CHILD_CMD.as_bytes());
        assert_eq!(
            Ok(Command::DeriveChild(TEST_DERIVE_CHILD_CMD)),
            Command::deserialize(&command)
        );
    }

    #[test]
    fn test_initial_conditions() {
        let mut dpe =
            DpeInstance::<OpensslCrypto>::new_for_test(Support::default(), &TEST_LOCALITIES)
                .unwrap();

        // Right now this command doesn't support INTERNAL_INPUT_INFO, make sure it errors.
        assert_eq!(
            Err(DpeErrorCode::InvalidArgument),
            DeriveChildCmd {
                handle: ContextHandle::default(),
                data: [0; DPE_PROFILE.get_tci_size()],
                flags: DeriveChildCmd::INTERNAL_INPUT_INFO,
                tci_type: 0,
                target_locality: 0
            }
            .execute(&mut dpe, 0)
        );

        // Right now this command doesn't support INTERNAL_INPUT_DICE, make sure it errors.
        assert_eq!(
            Err(DpeErrorCode::InvalidArgument),
            DeriveChildCmd {
                handle: ContextHandle::default(),
                data: [0; DPE_PROFILE.get_tci_size()],
                flags: DeriveChildCmd::INTERNAL_INPUT_DICE,
                tci_type: 0,
                target_locality: 0
            }
            .execute(&mut dpe, 0)
        );

        InitCtxCmd::new_use_default().execute(&mut dpe, 0).unwrap();

        // Make sure it can detect wrong locality.
        assert_eq!(
            Err(DpeErrorCode::InvalidHandle),
            DeriveChildCmd {
                handle: ContextHandle::default(),
                data: [0; DPE_PROFILE.get_tci_size()],
                flags: 0,
                tci_type: 0,
                target_locality: 0
            }
            .execute(&mut dpe, 1)
        );

        // Make sure it can detect an invalid locality.
        assert_eq!(
            Err(DpeErrorCode::InvalidLocality),
            DeriveChildCmd {
                handle: ContextHandle::default(),
                data: [0; DPE_PROFILE.get_tci_size()],
                flags: DeriveChildCmd::CHANGE_LOCALITY,
                tci_type: 0,
                target_locality: 2
            }
            .execute(&mut dpe, 0)
        );
    }

    #[test]
    fn test_max_tcis() {
        let mut dpe = DpeInstance::<OpensslCrypto>::new_for_test(
            Support {
                auto_init: true,
                ..Support::default()
            },
            &TEST_LOCALITIES,
        )
        .unwrap();

        // Fill all contexts with children (minus the auto-init context).
        for _ in 0..MAX_HANDLES - 1 {
            DeriveChildCmd {
                handle: ContextHandle::default(),
                data: [0; DPE_PROFILE.get_tci_size()],
                flags: DeriveChildCmd::MAKE_DEFAULT,
                tci_type: 0,
                target_locality: 0,
            }
            .execute(&mut dpe, TEST_LOCALITIES[0])
            .unwrap();
        }

        // Try to create one too many.
        assert_eq!(
            Err(DpeErrorCode::MaxTcis),
            DeriveChildCmd {
                handle: ContextHandle::default(),
                data: [0; DPE_PROFILE.get_tci_size()],
                flags: 0,
                tci_type: 0,
                target_locality: 0
            }
            .execute(&mut dpe, TEST_LOCALITIES[0])
        );
    }

    #[test]
    fn test_set_child_parent_relationship() {
        let mut dpe = DpeInstance::<OpensslCrypto>::new_for_test(
            Support {
                auto_init: true,
                ..Support::default()
            },
            &TEST_LOCALITIES,
        )
        .unwrap();

        let parent_idx = dpe
            .get_active_context_pos(&ContextHandle::default(), TEST_LOCALITIES[0])
            .unwrap();
        DeriveChildCmd {
            handle: ContextHandle::default(),
            data: [0; DPE_PROFILE.get_tci_size()],
            flags: DeriveChildCmd::MAKE_DEFAULT | DeriveChildCmd::CHANGE_LOCALITY,
            tci_type: 7,
            target_locality: TEST_LOCALITIES[1],
        }
        .execute(&mut dpe, TEST_LOCALITIES[0])
        .unwrap();
        let child_idx = dpe
            .get_active_context_pos(&ContextHandle::default(), TEST_LOCALITIES[1])
            .unwrap();
        let child = &dpe.contexts[child_idx];

        assert_eq!(parent_idx, child.parent_idx as usize);
        assert_eq!(
            child_idx,
            dpe.contexts[parent_idx].children.trailing_zeros() as usize
        );
        assert_eq!(7, child.tci.tci_type);
        assert_eq!(TEST_LOCALITIES[1], child.locality);
    }

    #[test]
    fn test_set_other_values() {
        let mut dpe = DpeInstance::<OpensslCrypto>::new_for_test(
            Support {
                auto_init: true,
                ..Support::default()
            },
            &TEST_LOCALITIES,
        )
        .unwrap();

        DeriveChildCmd {
            handle: ContextHandle::default(),
            data: [0; DPE_PROFILE.get_tci_size()],
            flags: DeriveChildCmd::MAKE_DEFAULT | DeriveChildCmd::CHANGE_LOCALITY,
            tci_type: 7,
            target_locality: TEST_LOCALITIES[1],
        }
        .execute(&mut dpe, TEST_LOCALITIES[0])
        .unwrap();

        let child = &dpe.contexts[dpe
            .get_active_context_pos(&ContextHandle::default(), TEST_LOCALITIES[1])
            .unwrap()];

        assert_eq!(7, child.tci.tci_type);
        assert_eq!(TEST_LOCALITIES[1], child.locality);
        assert_eq!(ContextType::Normal, child.context_type);
    }

    #[test]
    fn test_correct_child_handle() {
        let mut dpe = DpeInstance::<OpensslCrypto>::new_for_test(
            Support {
                auto_init: true,
                ..Support::default()
            },
            &TEST_LOCALITIES,
        )
        .unwrap();

        // Make sure child handle is default when creating default child.
        assert_eq!(
            Ok(Response::DeriveChild(DeriveChildResp {
                handle: ContextHandle::default(),
                parent_handle: ContextHandle([0xff; ContextHandle::SIZE])
            })),
            DeriveChildCmd {
                handle: ContextHandle::default(),
                data: [0; DPE_PROFILE.get_tci_size()],
                flags: DeriveChildCmd::MAKE_DEFAULT,
                tci_type: 0,
                target_locality: 0,
            }
            .execute(&mut dpe, TEST_LOCALITIES[0])
        );

        // Make sure child has a random handle when not creating default.
        assert_eq!(
            Ok(Response::DeriveChild(DeriveChildResp {
                handle: SIMULATION_HANDLE,
                parent_handle: ContextHandle([0xff; ContextHandle::SIZE])
            })),
            DeriveChildCmd {
                handle: ContextHandle::default(),
                data: [0; DPE_PROFILE.get_tci_size()],
                flags: 0,
                tci_type: 0,
                target_locality: 0,
            }
            .execute(&mut dpe, TEST_LOCALITIES[0])
        );
    }

    #[test]
    fn test_correct_parent_handle() {
        let mut dpe = DpeInstance::<OpensslCrypto>::new_for_test(
            Support {
                auto_init: true,
                ..Support::default()
            },
            &TEST_LOCALITIES,
        )
        .unwrap();

        // Make sure the parent handle is non-sense when not retaining.
        assert_eq!(
            Ok(Response::DeriveChild(DeriveChildResp {
                handle: ContextHandle::default(),
                parent_handle: ContextHandle([0xff; ContextHandle::SIZE])
            })),
            DeriveChildCmd {
                handle: ContextHandle::default(),
                data: [0; DPE_PROFILE.get_tci_size()],
                flags: DeriveChildCmd::MAKE_DEFAULT,
                tci_type: 0,
                target_locality: 0,
            }
            .execute(&mut dpe, TEST_LOCALITIES[0])
        );

        // Make sure the default parent handle stays the default handle when retained.
        assert_eq!(
            Ok(Response::DeriveChild(DeriveChildResp {
                handle: ContextHandle::default(),
                parent_handle: ContextHandle::default(),
            })),
            DeriveChildCmd {
                handle: ContextHandle::default(),
                data: [0; DPE_PROFILE.get_tci_size()],
                flags: DeriveChildCmd::RETAIN_PARENT
                    | DeriveChildCmd::MAKE_DEFAULT
                    | DeriveChildCmd::CHANGE_LOCALITY,
                tci_type: 0,
                target_locality: TEST_LOCALITIES[1],
            }
            .execute(&mut dpe, TEST_LOCALITIES[0])
        );

        // The next test case is to make sure the parent handle rotates when not the default and
        // parent is retained. For this to work, we need the child to be created as the default (see
        // note below). Right now both localities have a default. We need to mutate one of them so
        // we can create a new child as the default in the locality.
        let old_default_idx = dpe
            .get_active_context_pos(&ContextHandle::default(), TEST_LOCALITIES[0])
            .unwrap();
        dpe.contexts[old_default_idx].handle = ContextHandle([0x1; ContextHandle::SIZE]);

        // Make sure the parent handle is regenerated when being retained.
        assert_eq!(
            Ok(Response::DeriveChild(DeriveChildResp {
                handle: ContextHandle::default(),
                parent_handle: SIMULATION_HANDLE,
            })),
            DeriveChildCmd {
                handle: dpe.contexts[old_default_idx].handle,
                data: [0; DPE_PROFILE.get_tci_size()],
                flags: DeriveChildCmd::RETAIN_PARENT | DeriveChildCmd::MAKE_DEFAULT,
                tci_type: 0,
                target_locality: 0,
            }
            .execute(&mut dpe, TEST_LOCALITIES[0])
        );

        // NOTE: The deterministic RNG we use in tests will create the same value every time. This
        // makes it so we can't test the case where neither parent or child are default and the
        // parent will be retained. It would try to generate two new handles, but they would be the
        // same value and throw an error. We either need to change the RNG or test this test case
        // outside of unit tests.
    }

    #[test]
    fn test_safe_to_make_default() {
        let mut make_default_in_0 = DeriveChildCmd {
            handle: ContextHandle::default(),
            data: TciMeasurement::default().0,
            flags: DeriveChildCmd::MAKE_DEFAULT,
            tci_type: 0,
            target_locality: 0,
        };
        let parent_idx = 0;
        // No default context.
        assert!(make_default_in_0.safe_to_make_default(parent_idx, None));
        // Default context at parent, but not going to retain parent.
        assert!(make_default_in_0.safe_to_make_default(parent_idx, Some(parent_idx)));
        // Make default in a different locality that already has a default.
        assert!(!make_default_in_0.safe_to_make_default(parent_idx, Some(1)));

        make_default_in_0.flags |= DeriveChildCmd::RETAIN_PARENT;

        // Retain parent and make default in another locality that doesn't have a default.
        assert!(make_default_in_0.safe_to_make_default(parent_idx, None));
        // Retain default parent and make default in another locality that has a default.
        assert!(!make_default_in_0.safe_to_make_default(parent_idx, Some(1)));
        // Retain default parent.
        assert!(!make_default_in_0.safe_to_make_default(parent_idx, Some(parent_idx)));
    }
}