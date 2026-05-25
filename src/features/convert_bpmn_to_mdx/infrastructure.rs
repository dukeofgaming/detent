pub mod cli {
    pub mod compile {
        pub use crate::transpiler::commands::compile::run;
    }

    pub mod import {
        pub use crate::transpiler::commands::import::run;
    }

    pub mod validate {
        pub use crate::transpiler::commands::validate::run;
    }
}
