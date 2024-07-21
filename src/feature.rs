// Purpose is to allow users to opt-in to use specific lines or line chunks in their config files.

// To include specific settings these methods are allowed:
// 1. chunks
//      1 # dotkopper[myfeature]
//      2 some_bool_setting=true
//      3 language_setting="en/us"
//      4 # dotkopper-end
//
// 2. one-liner
//      1 some_bool_setting=true # dotkopper-ol[myfeature]

use crate::dotconfig::{DotConfig, Dotfile};

pub(crate) struct FeatureMerger {
    dotconfig: DotConfig,
}
