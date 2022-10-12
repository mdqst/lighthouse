pub use proto_array::{CountUnrealizedFull, ParticipationThreshold, ReOrgThreshold};
use serde_derive::{Deserialize, Serialize};
use std::time::Duration;
use types::Checkpoint;

pub const DEFAULT_RE_ORG_THRESHOLD: ReOrgThreshold = ReOrgThreshold(10);
pub const DEFAULT_RE_ORG_PARTICIPATION_THRESHOLD: ParticipationThreshold =
    ParticipationThreshold(80);
pub const DEFAULT_FORK_CHOICE_BEFORE_PROPOSAL_TIMEOUT: u64 = 250;

/// At 12s slot times, the means that the payload preparation routine will run 4s before the start
/// of each slot (`12 / 3 = 4`).
pub const DEFAULT_PREPARE_PAYLOAD_LOOKAHEAD_FACTOR: u32 = 3;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct ChainConfig {
    /// Maximum number of slots to skip when importing a consensus message (e.g., block,
    /// attestation, etc).
    ///
    /// If `None`, there is no limit.
    pub import_max_skip_slots: Option<u64>,
    /// A user-input `Checkpoint` that must exist in the beacon chain's sync path.
    ///
    /// If `None`, there is no weak subjectivity verification.
    pub weak_subjectivity_checkpoint: Option<Checkpoint>,
    /// Determine whether to reconstruct historic states, usually after a checkpoint sync.
    pub reconstruct_historic_states: bool,
    /// Whether timeouts on `TimeoutRwLock`s are enabled or not.
    pub enable_lock_timeouts: bool,
    /// The max size of a message that can be sent over the network.
    pub max_network_size: usize,
    /// Maximum percentage of committee weight at which to attempt re-orging the canonical head.
    pub re_org_threshold: Option<ReOrgThreshold>,
    /// Minimum participation at which a proposer re-org should be attempted.
    pub re_org_participation_threshold: ParticipationThreshold,
    /// Number of milliseconds to wait for fork choice before proposing a block.
    ///
    /// If set to 0 then block proposal will not wait for fork choice at all.
    pub fork_choice_before_proposal_timeout_ms: u64,
    /// Number of skip slots in a row before the BN refuses to use connected builders during payload construction.
    pub builder_fallback_skips: usize,
    /// Number of skip slots in the past `SLOTS_PER_EPOCH` before the BN refuses to use connected
    /// builders during payload construction.
    pub builder_fallback_skips_per_epoch: usize,
    /// Number of epochs since finalization before the BN refuses to use connected builders during
    /// payload construction.
    pub builder_fallback_epochs_since_finalization: usize,
    /// Whether any chain health checks should be considered when deciding whether to use the builder API.
    pub builder_fallback_disable_checks: bool,
    /// When set to `true`, weigh the "unrealized" FFG progression when choosing a head in fork
    /// choice.
    pub count_unrealized: bool,
    /// When set to `true`, forget any valid/invalid/optimistic statuses in fork choice during start
    /// up.
    pub always_reset_payload_statuses: bool,
    /// Whether to apply paranoid checks to blocks proposed by this beacon node.
    pub paranoid_block_proposal: bool,
    /// Whether to strictly count unrealized justified votes.
    pub count_unrealized_full: CountUnrealizedFull,
    /// The offset from the start of a proposal slot at which payload attributes should be sent.
    ///
    /// Low values are useful for execution engines which don't improve their payload after the
    /// first call, and high values are useful for ensuring the EL is given ample notice.
    pub prepare_payload_lookahead: Duration,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            import_max_skip_slots: None,
            weak_subjectivity_checkpoint: None,
            reconstruct_historic_states: false,
            enable_lock_timeouts: true,
            max_network_size: 10 * 1_048_576, // 10M
            re_org_threshold: None,
            re_org_participation_threshold: DEFAULT_RE_ORG_PARTICIPATION_THRESHOLD,
            fork_choice_before_proposal_timeout_ms: DEFAULT_FORK_CHOICE_BEFORE_PROPOSAL_TIMEOUT,
            // Builder fallback configs that are set in `clap` will override these.
            builder_fallback_skips: 3,
            builder_fallback_skips_per_epoch: 8,
            builder_fallback_epochs_since_finalization: 3,
            builder_fallback_disable_checks: false,
            count_unrealized: true,
            always_reset_payload_statuses: false,
            paranoid_block_proposal: false,
            count_unrealized_full: CountUnrealizedFull::default(),
            prepare_payload_lookahead: Duration::from_secs(4),
        }
    }
}
