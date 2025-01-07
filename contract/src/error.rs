//! Error handling on the Casper platform.
use casper_types::ApiError;

/// Errors that the contract can return.
///
/// When an `Error` is returned from a smart contract, it is converted to an [`ApiError::User`].
///
/// While the code consuming this contract needs to define further error variants, it can
/// return those via the [`Error::User`] variant or equivalently via the [`ApiError::User`]
/// variant.
#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum SwapError {
    InsufficientRights = 3001,
    UnexpectedKeyVariant = 3002,
    InvalidStorageUref = 3003,
    MissingStorageUref = 3004,
    InvalidKey = 3005,
    MissingKey = 3006,
    Phantom = 3007,
    FailedToGetArgBytes = 3008,
    InvalidEventsMode = 3009,
    MissingEventsMode = 3010,
    InvalidUpgradeFlag = 3011,
    MissingSwapName = 3012,
    InvalidSwapName = 3013,
    InvalidContractHash = 3014,
    MissingContractHash = 3015,
    InvalidAdminList = 3016,
    InvalidNoneList = 3017,
    InvalidPackageHash = 3018,
    MissingPackageHash = 3019,
    ContractAlreadyInitialized = 3020,
    MissingPackageHashForUpgrade = 3021,
    Overflow = 3022,
    MissingInstaller = 3023,
    InvalidInstaller = 3024,
    InvalidTokenContractPackage = 3025,
    MissingTokenContractPackage = 3026,
    InvalidAmount = 3027,
    InvalidTimeWindow = 3028,
    MissingPurse = 3029,
    InvalidPurseTransfer = 3030,
    InvalidRate = 3031,
    BelowMinimumSwap = 3032,
    SwapNotActive = 3033,
    SwapExpired = 3034,
    MissingStartTime = 3035,
    InvalidStartTime = 3036,
    MissingEndTime = 3037,
    InvalidEndTime = 3038,
}

impl From<SwapError> for ApiError {
    fn from(error: SwapError) -> Self {
        ApiError::User(error as u16)
    }
}
